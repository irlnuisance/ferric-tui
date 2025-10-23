use std::{fs, path::{Path, PathBuf}};

use tokio::task;

use crate::{app::state::{IsoMeta, Device}, event::{Event}};

use super::msg::{Cmd, Msg};

pub fn spawn_all(cmds: Vec<Cmd>, tx: tokio::sync::mpsc::UnboundedSender<Event>) {
    for cmd in cmds {
        match cmd {
            Cmd::Noop => {}
            Cmd::ScanIso { query } => {
                let tx = tx.clone();
                task::spawn(async move {
                    let results = task::spawn_blocking(move || scan_iso_default_roots(&query)).await.unwrap_or_default();
                    let _ = tx.send(Event::App(Msg::IsoResults(results)));
                });
            }
            Cmd::RefreshDevices => {
                let tx = tx.clone();
                task::spawn(async move {
                    let devs = task::spawn_blocking(move || refresh_devices()).await.unwrap_or_default();
                    let _ = tx.send(Event::App(Msg::DevicesRefreshed(devs)));
                });
            }
            Cmd::Write { iso, device } => {
                let tx = tx.clone();
                task::spawn(async move {
                    let _ = tx.send(Event::App(Msg::WriteStarted { total: file_size(&iso).unwrap_or(0) }));
                    let tx2 = tx.clone();
                    let res = task::spawn_blocking(move || write_image(iso, device, tx2)).await;
                    if let Err(e) = res {
                        let _ = tx.send(Event::App(Msg::WriteFinished(Err(format!("Join error: {}", e)))));
                    }
                });
            }
            Cmd::Verify { iso, device, size } => {
                let tx = tx.clone();
                task::spawn(async move {
                    let _ = tx.send(Event::App(Msg::VerifyStarted { total: size }));
                    let tx2 = tx.clone();
                    let res = task::spawn_blocking(move || verify_image(iso, device, size, tx2)).await;
                    if let Err(e) = res {
                        let _ = tx.send(Event::App(Msg::VerifyFinished(Err(format!("Join error: {}", e)))));
                    }
                });
            }
            Cmd::ReexecWithSudo => {
                ratatui::restore();
                let exe = std::env::current_exe().unwrap_or_else(|_| std::path::PathBuf::from("ferric"));
                let args: Vec<std::ffi::OsString> = std::env::args_os().skip(1).collect();
                let mut cmd = std::process::Command::new("sudo");
                cmd.arg("-E").arg("--").arg(exe);
                for a in &args { cmd.arg(a); }
                #[cfg(unix)]
                {
                    use std::os::unix::process::CommandExt;
                    let _ = cmd.exec();
                    let _ = tx.send(Event::App(Msg::WriteFinished(Err("Failed to exec sudo".to_string()))));
                    std::process::exit(1);
                }
                #[cfg(not(unix))]
                {
                    match cmd.status() {
                        Ok(st) if st.success() => std::process::exit(0),
                        _ => {
                            let _ = tx.send(Event::App(Msg::WriteFinished(Err("Failed to run sudo".to_string()))));
                        }
                    }
                }
            }
        }
    }
}

fn scan_iso_default_roots(query: &str) -> Vec<IsoMeta> {
    let mut roots = Vec::new();
    if let Ok(cwd) = std::env::current_dir() { roots.push(cwd); }
    if let Some(home) = dirs_home() { 
        roots.push(home.join("Downloads"));
        roots.push(home);
    }
    scan_iso(roots, query, 5, 100 * 1024 * 1024)
}

fn dirs_home() -> Option<PathBuf> {
    if let Ok(home) = std::env::var("HOME") { return Some(PathBuf::from(home)); }
    None
}

fn scan_iso(roots: Vec<PathBuf>, query: &str, max_depth: usize, min_size: u64) -> Vec<IsoMeta> {
    let mut out = Vec::new();
    for root in roots {
        if root.exists() { walk(&root, 0, max_depth, min_size, query, &mut out); }
    }
    out.sort_by(|a, b| b.modified.cmp(&a.modified).then_with(|| a.path.cmp(&b.path)));
    out
}

fn walk(path: &Path, depth: usize, max_depth: usize, min_size: u64, query: &str, out: &mut Vec<IsoMeta>) {
    if depth > max_depth { return; }
    let entries = match fs::read_dir(path) { Ok(rd) => rd, Err(_) => return };
    for entry in entries.flatten() {
        let path = entry.path();
        let file_name = entry.file_name();
        let file_name = file_name.to_string_lossy();
        if file_name.starts_with('.') && path.is_dir() { continue; }
        let file_type = match entry.file_type() { Ok(ft) => ft, Err(_) => continue };
        if file_type.is_symlink() { continue; }
        if file_type.is_dir() {
            walk(&path, depth + 1, max_depth, min_size, query, out);
        } else if file_type.is_file() {
            if !is_iso_like(&path) { continue; }
            if !matches_query(&file_name, query) { continue; }
            match fs::metadata(&path) {
                Ok(meta) => {
                    let size = meta.len();
                    if size < min_size { continue; }
                    let modified = meta.modified().ok();
                    out.push(IsoMeta { path: path.clone(), size, modified });
                }
                Err(_) => continue,
            }
        }
    }
}

fn is_iso_like(p: &Path) -> bool {
    match p.extension().and_then(|e| e.to_str()).map(|s| s.to_ascii_lowercase()) {
        Some(ext) if matches!(ext.as_str(), "iso" | "img" | "raw") => true,
        _ => false,
    }
}

fn matches_query(name: &str, query: &str) -> bool {
    if query.is_empty() { return true; }
    let name = name.to_ascii_lowercase();
    let q = query.to_ascii_lowercase();
    name.contains(&q)
}

fn refresh_devices() -> Vec<Device> {
    let mut cmd = std::process::Command::new("lsblk");
    cmd.arg("-P").arg("-b").arg("-o").arg("NAME,TYPE,SIZE,RM,RO,MODEL,SERIAL,TRAN,HOTPLUG,MOUNTPOINT,PKNAME,LABEL");
    let output = match cmd.output() { Ok(o) => o, Err(_) => return Vec::new() };
    if !output.status.success() { return Vec::new(); }
    let stdout = String::from_utf8_lossy(&output.stdout);

    use std::collections::HashMap;
    #[derive(Default)]
    struct DiskAgg {
        size: u64,
        rm: bool,
        ro: bool,
        model: Option<String>,
        serial: Option<String>,
        tran: Option<String>,
        hotplug: bool,
        any_mounted: bool,
        has_root: bool,
        labels: Vec<String>,
    }

    let mut disks: HashMap<String, DiskAgg> = HashMap::new();

    for line in stdout.lines() {
        let kv = parse_kv_line(line);
        let typ = kv.get("TYPE").map(|s| s.as_str()).unwrap_or("");
        let name = match kv.get("NAME") { Some(n) => n.clone(), None => continue };
        match typ {
            "disk" => {
                let size = kv.get("SIZE").and_then(|s| s.parse::<u64>().ok()).unwrap_or(0);
                let rm = kv.get("RM").map(|s| s == "1").unwrap_or(false);
                let ro = kv.get("RO").map(|s| s == "1").unwrap_or(false);
                let hotplug = kv.get("HOTPLUG").map(|s| s == "1").unwrap_or(false);
                let model = kv.get("MODEL").filter(|s| !s.is_empty()).cloned();
                let serial = kv.get("SERIAL").filter(|s| !s.is_empty()).cloned();
                let tran = kv.get("TRAN").filter(|s| !s.is_empty()).cloned();
                disks.insert(name.clone(), DiskAgg { size, rm, ro, hotplug, model, serial, tran, any_mounted: false, has_root: false, labels: Vec::new() });
            }
            "part" => {
                let mp = kv.get("MOUNTPOINT").cloned().unwrap_or_default();
                let pk = kv.get("PKNAME").cloned().unwrap_or_default();
                let lbl = kv.get("LABEL").cloned().unwrap_or_default();
                if pk.is_empty() { continue; }
                if let Some(d) = disks.get_mut(&pk) {
                    if !mp.is_empty() {
                        d.any_mounted = true;
                        if mp == "/" { d.has_root = true; }
                    }
                    let mut label = lbl;
                    if label.is_empty() && !mp.is_empty() {
                        if let Some(last) = mp.rsplit('/').next() { if !last.is_empty() { label = last.to_string(); } }
                    }
                    if !label.is_empty() && !d.labels.iter().any(|l| l == &label) {
                        d.labels.push(label);
                    }
                }
            }
            _ => {}
        }
    }

    let mut out = Vec::new();
    for (name, d) in disks.into_iter() {
        if name.starts_with("loop") { continue; }
        if d.ro { continue; }
        if d.has_root { continue; }
        let path = PathBuf::from(format!("/dev/{}", name));
        out.push(Device { name, path, size: d.size, model: d.model, serial: d.serial, tran: d.tran, removable: d.rm, hotplug: d.hotplug, ro: d.ro, mounted: d.any_mounted, labels: d.labels });
    }
    // hotplug/removable first
    out.sort_by(|a, b| b.hotplug.cmp(&a.hotplug)
        .then(b.removable.cmp(&a.removable))
        .then(a.mounted.cmp(&b.mounted))
        .then(a.name.cmp(&b.name)));
    out
}

fn parse_kv_line(s: &str) -> std::collections::BTreeMap<String, String> {
    use std::collections::BTreeMap;
    let mut out = BTreeMap::new();
    let mut i = 0; let b = s.as_bytes();
    while i < b.len() {
        // skip spaces
        while i < b.len() && b[i].is_ascii_whitespace() { i += 1; }
        if i >= b.len() { break; }
        let start = i; while i < b.len() && b[i] != b'=' { i += 1; }
        if i >= b.len() { break; }
        let key = &s[start..i]; i += 1; // '='
        if i < b.len() && b[i] == b'"' { i += 1; }
        let vstart = i;
        while i < b.len() && b[i] != b'"' { i += 1; }
        let val = &s[vstart..i];
        if i < b.len() && b[i] == b'"' { i += 1; }
        while i < b.len() && b[i].is_ascii_whitespace() { i += 1; }
        out.insert(key.to_string(), val.to_string());
    }
    out
}

fn file_size(p: &PathBuf) -> Option<u64> {
    std::fs::metadata(p).ok().map(|m| m.len())
}

fn write_image(iso_path: PathBuf, device_path: PathBuf, tx: tokio::sync::mpsc::UnboundedSender<Event>) {
    use std::fs::OpenOptions;
    use std::io::{Read, Write};
    let _ = unmount_partitions_of(&device_path);
    let total = file_size(&iso_path).unwrap_or(0);
    let mut src = match std::fs::File::open(&iso_path) {
        Ok(f) => f,
        Err(e) => {
            let _ = tx.send(Event::App(Msg::WriteFinished(Err(format!("Failed to open ISO: {}", e)))));
            return;
        }
    };
    let mut dst = match OpenOptions::new().write(true).open(&device_path) {
        Ok(f) => f,
        Err(e) => {
            let _ = tx.send(Event::App(Msg::WriteFinished(Err(format!("Failed to open device {}: {}", device_path.display(), e)))));
            return;
        }
    };

    let mut buf = vec![0u8; 4 * 1024 * 1024];
    let start = std::time::Instant::now();
    let mut written: u64 = 0;

    loop {
        match src.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                let mut off = 0;
                while off < n {
                    match dst.write(&buf[off..n]) {
                        Ok(w) => { off += w; written += w as u64; },
                        Err(e) => {
                            let _ = tx.send(Event::App(Msg::WriteFinished(Err(format!("Write error: {}", e)))));
                            return;
                        }
                    }
                }
                let elapsed = start.elapsed().as_secs_f64().max(0.000_001);
                let bps = (written as f64) / elapsed;
                let _ = tx.send(Event::App(Msg::WriteProgress { written, total, bps }));
            }
            Err(e) => {
                let _ = tx.send(Event::App(Msg::WriteFinished(Err(format!("Read error: {}", e)))));
                return;
            }
        }
    }

    if let Err(e) = dst.flush() {
        let _ = tx.send(Event::App(Msg::WriteFinished(Err(format!("Flush error: {}", e)))));
        return;
    }
    if let Err(e) = dst.sync_all() {
        let _ = tx.send(Event::App(Msg::WriteFinished(Err(format!("sync_all error: {}", e)))));
        return;
    }

    let _ = partprobe(&device_path);

    let _ = tx.send(Event::App(Msg::WriteFinished(Ok(()))));
}

fn verify_image(iso_path: PathBuf, device_path: PathBuf, size: u64, tx: tokio::sync::mpsc::UnboundedSender<Event>) {
    use std::io::Read;
    let mut iso = match std::fs::File::open(&iso_path) {
        Ok(f) => f,
        Err(e) => {
            let _ = tx.send(Event::App(Msg::VerifyFinished(Err(format!("Failed to open ISO for verify: {}", e)))));
            return;
        }
    };
    let mut dev = match std::fs::File::open(&device_path) {
        Ok(f) => f,
        Err(e) => {
            let _ = tx.send(Event::App(Msg::VerifyFinished(Err(format!("Failed to open device for verify: {}", e)))));
            return;
        }
    };
    let mut left = size;
    let mut checked: u64 = 0;
    let mut buf_iso = vec![0u8; 4 * 1024 * 1024];
    let mut buf_dev = vec![0u8; 4 * 1024 * 1024];
    let start = std::time::Instant::now();
    while left > 0 {
        let to_read = std::cmp::min(left, buf_iso.len() as u64) as usize;
        if let Err(e) = iso.read_exact(&mut buf_iso[..to_read]) {
            let _ = tx.send(Event::App(Msg::VerifyFinished(Err(format!("ISO read error during verify: {}", e)))));
            return;
        }
        if let Err(e) = dev.read_exact(&mut buf_dev[..to_read]) {
            let _ = tx.send(Event::App(Msg::VerifyFinished(Err(format!("Device read error during verify: {}", e)))));
            return;
        }
        if buf_iso[..to_read] != buf_dev[..to_read] {
            let _ = tx.send(Event::App(Msg::VerifyFinished(Err("Mismatch between ISO and device".to_string()))));
            return;
        }
        left -= to_read as u64;
        checked += to_read as u64;
        let elapsed = start.elapsed().as_secs_f64().max(0.000_001);
        let bps = (checked as f64) / elapsed;
        let _ = tx.send(Event::App(Msg::VerifyProgress { checked, total: size, bps }));
    }
    let _ = tx.send(Event::App(Msg::VerifyFinished(Ok(()))));
}

fn unmount_partitions_of(device_path: &Path) -> Result<(), String> {
    let name = device_basename(device_path).ok_or_else(|| "invalid device path".to_string())?;
    let mut cmd = std::process::Command::new("lsblk");
    cmd.arg("-P").arg("-b").arg("-o").arg("NAME,TYPE,MOUNTPOINT,PKNAME");
    let output = cmd.output().map_err(|e| format!("lsblk failed: {}", e))?;
    if !output.status.success() { return Err("lsblk returned non-zero".to_string()); }
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        let kv = parse_kv_line(line);
        match (kv.get("TYPE").map(|s| s.as_str()), kv.get("PKNAME"), kv.get("MOUNTPOINT")) {
            (Some("part"), Some(pk), Some(mp)) if pk == &name && !mp.is_empty() => {
                let _ = std::process::Command::new("umount").arg(mp).status();
            }
            _ => {}
        }
    }
    Ok(())
}

fn device_basename(p: &Path) -> Option<String> {
    let s = p.file_name()?.to_string_lossy().to_string();
    Some(s)
}

fn partprobe(device_path: &Path) -> Result<(), String> {
    if let Ok(st) = std::process::Command::new("partprobe").arg(device_path).status() {
        if st.success() { return Ok(()); }
    }
    if let Ok(st) = std::process::Command::new("blockdev").arg("--rereadpt").arg(device_path).status() {
        if st.success() { return Ok(()); }
    }
    Err("partprobe/blockdev failed".to_string())
}
