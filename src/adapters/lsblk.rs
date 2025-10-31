use crate::domain::{device::Device, paths::DevicePath, units::ByteSize};
use std::{
    collections::{BTreeMap, HashMap},
    path::PathBuf,
};

pub fn refresh_devices() -> Vec<Device> {
    let mut cmd = std::process::Command::new("lsblk");
    cmd.arg("-P")
        .arg("-b")
        .arg("-o")
        .arg("NAME,TYPE,SIZE,RM,RO,MODEL,SERIAL,TRAN,HOTPLUG,MOUNTPOINT,PKNAME,LABEL");
    let output = match cmd.output() {
        Ok(o) => o,
        Err(_) => return Vec::new(),
    };
    if !output.status.success() {
        return Vec::new();
    }
    let stdout = String::from_utf8_lossy(&output.stdout);

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
        let kv = parse_key_value_line(line);
        let typ = kv.get("TYPE").map(|s| s.as_str()).unwrap_or("");
        let name = match kv.get("NAME") {
            Some(n) => n.clone(),
            None => continue,
        };
        match typ {
            "disk" => {
                let size = kv
                    .get("SIZE")
                    .and_then(|s| s.parse::<u64>().ok())
                    .unwrap_or(0);
                let rm = kv.get("RM").map(|s| s == "1").unwrap_or(false);
                let ro = kv.get("RO").map(|s| s == "1").unwrap_or(false);
                let hotplug = kv.get("HOTPLUG").map(|s| s == "1").unwrap_or(false);
                let model = kv.get("MODEL").filter(|s| !s.is_empty()).cloned();
                let serial = kv.get("SERIAL").filter(|s| !s.is_empty()).cloned();
                let tran = kv.get("TRAN").filter(|s| !s.is_empty()).cloned();
                disks.insert(
                    name.clone(),
                    DiskAgg {
                        size,
                        rm,
                        ro,
                        hotplug,
                        model,
                        serial,
                        tran,
                        any_mounted: false,
                        has_root: false,
                        labels: Vec::new(),
                    },
                );
            }
            "part" => {
                let mp = kv.get("MOUNTPOINT").cloned().unwrap_or_default();
                let pk = kv.get("PKNAME").cloned().unwrap_or_default();
                let lbl = kv.get("LABEL").cloned().unwrap_or_default();
                if pk.is_empty() {
                    continue;
                }
                if let Some(d) = disks.get_mut(&pk) {
                    if !mp.is_empty() {
                        d.any_mounted = true;
                        if mp == "/" {
                            d.has_root = true;
                        }
                    }
                    let mut label = lbl;
                    if label.is_empty() && !mp.is_empty() {
                        if let Some(last) = mp.rsplit('/').next() {
                            if !last.is_empty() {
                                label = last.to_string();
                            }
                        }
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
        if name.starts_with("loop") {
            continue;
        }
        if d.ro {
            continue;
        }
        if d.has_root {
            continue;
        }
        let path = DevicePath::from(PathBuf::from(format!("/dev/{}", name)));
        out.push(Device {
            name,
            path,
            size: ByteSize::new(d.size),
            model: d.model,
            serial: d.serial,
            tran: d.tran,
            removable: d.rm,
            hotplug: d.hotplug,
            ro: d.ro,
            mounted: d.any_mounted,
            labels: d.labels,
        });
    }
    out.sort_by(|a, b| {
        b.hotplug
            .cmp(&a.hotplug)
            .then(b.removable.cmp(&a.removable))
            .then(a.mounted.cmp(&b.mounted))
            .then(a.name.cmp(&b.name))
    });
    out
}

pub fn parse_key_value_line(s: &str) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    let mut i = 0;
    let b = s.as_bytes();
    while i < b.len() {
        while i < b.len() && b[i].is_ascii_whitespace() {
            i += 1;
        }
        if i >= b.len() {
            break;
        }
        let start = i;
        while i < b.len() && b[i] != b'=' {
            i += 1;
        }
        if i >= b.len() {
            break;
        }
        let key = &s[start..i];
        i += 1;
        if i < b.len() && b[i] == b'"' {
            i += 1;
        }
        let vstart = i;
        while i < b.len() && b[i] != b'"' {
            i += 1;
        }
        let val = &s[vstart..i];
        if i < b.len() && b[i] == b'"' {
            i += 1;
        }
        while i < b.len() && b[i].is_ascii_whitespace() {
            i += 1;
        }
        out.insert(key.to_string(), val.to_string());
    }
    out
}
