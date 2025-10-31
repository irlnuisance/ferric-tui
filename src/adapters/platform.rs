use std::path::Path;

#[must_use = "unmount errors should be handled or logged"]
pub fn unmount_partitions_of(device_path: &Path) -> Result<(), String> {
    let name = device_basename(device_path).ok_or_else(|| "invalid device path".to_string())?;
    let mut cmd = std::process::Command::new("lsblk");
    cmd.arg("-P")
        .arg("-b")
        .arg("-o")
        .arg("NAME,TYPE,MOUNTPOINT,PKNAME");
    let output = cmd.output().map_err(|e| format!("lsblk failed: {}", e))?;
    if !output.status.success() {
        return Err("lsblk returned non-zero".to_string());
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        let kv = crate::adapters::lsblk::parse_key_value_line(line);
        match (
            kv.get("TYPE").map(|s| s.as_str()),
            kv.get("PKNAME"),
            kv.get("MOUNTPOINT"),
        ) {
            (Some("part"), Some(pk), Some(mp)) if pk == &name && !mp.is_empty() => {
                let _ = std::process::Command::new("umount").arg(mp).status();
            }
            _ => {}
        }
    }
    Ok(())
}

#[must_use = "partprobe errors may indicate device not ready"]
pub fn partprobe(device_path: &Path) -> Result<(), String> {
    if let Ok(st) = std::process::Command::new("partprobe")
        .arg(device_path)
        .status()
    {
        if st.success() {
            return Ok(());
        }
    }
    if let Ok(st) = std::process::Command::new("blockdev")
        .arg("--rereadpt")
        .arg(device_path)
        .status()
    {
        if st.success() {
            return Ok(());
        }
    }
    Err("partprobe/blockdev failed".to_string())
}

pub fn is_root() -> bool {
    if let Ok(status) = std::fs::read_to_string("/proc/self/status") {
        for line in status.lines() {
            if let Some(rest) = line.strip_prefix("Uid:") {
                let mut it = rest.split_whitespace();
                if let Some(real) = it.next() {
                    if real == "0" {
                        return true;
                    }
                }
                break;
            }
        }
    }
    false
}

fn device_basename(p: &Path) -> Option<String> {
    let s = p.file_name()?.to_string_lossy().to_string();
    Some(s)
}
