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
    let mut errors: Vec<String> = Vec::new();
    let mut attempted = 0usize;
    for line in stdout.lines() {
        let kv = crate::adapters::lsblk::parse_key_value_line(line);
        match (
            kv.get("TYPE").map(|s| s.as_str()),
            kv.get("PKNAME"),
            kv.get("MOUNTPOINT"),
        ) {
            (Some("part"), Some(pk), Some(mp)) if pk == &name && !mp.is_empty() => {
                attempted += 1;
                match std::process::Command::new("umount").arg(mp).output() {
                    Ok(out) => {
                        if !out.status.success() {
                            let mut msg = format!(
                                "umount {} failed with status {}",
                                mp,
                                out.status.code().unwrap_or(-1)
                            );
                            let stderr = String::from_utf8_lossy(&out.stderr);
                            let stderr = stderr.trim();
                            if !stderr.is_empty() {
                                msg.push_str(": ");
                                msg.push_str(stderr);
                            }
                            errors.push(msg);
                        }
                    }
                    Err(e) => errors.push(format!("failed to run umount {}: {}", mp, e)),
                }
            }
            _ => {}
        }
    }
    if errors.is_empty() {
        return Ok(());
    }
    Err(format!(
        "failed to unmount {} of {} partition(s): {}",
        errors.len(),
        attempted,
        errors.join("; ")
    ))
}

#[must_use = "partprobe errors may indicate device not ready"]
pub fn partprobe(device_path: &Path) -> Result<(), String> {
    match std::process::Command::new("partprobe")
        .arg(device_path)
        .output()
    {
        Ok(out) if out.status.success() => return Ok(()),
        Ok(out) => {
            let err = String::from_utf8_lossy(&out.stderr).trim().to_string();
            let code = out.status.code().unwrap_or(-1);
            match std::process::Command::new("blockdev")
                .arg("--rereadpt")
                .arg(device_path)
                .output()
            {
                Ok(o2) if o2.status.success() => return Ok(()),
                Ok(o2) => {
                    let err2 = String::from_utf8_lossy(&o2.stderr).trim().to_string();
                    let code2 = o2.status.code().unwrap_or(-1);
                    return Err(format!(
                        "partprobe failed (code {}): {}; blockdev --rereadpt failed (code {}): {}",
                        code, err, code2, err2
                    ));
                }
                Err(e2) => {
                    return Err(format!(
                        "partprobe failed (code {}): {}; blockdev --rereadpt spawn error: {}",
                        code, err, e2
                    ));
                }
            }
        }
        Err(e) => {
            match std::process::Command::new("blockdev")
                .arg("--rereadpt")
                .arg(device_path)
                .output()
            {
                Ok(o2) if o2.status.success() => return Ok(()),
                Ok(o2) => {
                    let err2 = String::from_utf8_lossy(&o2.stderr).trim().to_string();
                    let code2 = o2.status.code().unwrap_or(-1);
                    return Err(format!(
                        "partprobe spawn error: {}; blockdev --rereadpt failed (code {}): {}",
                        e, code2, err2
                    ));
                }
                Err(e2) => {
                    return Err(format!(
                        "partprobe spawn error: {}; blockdev --rereadpt spawn error: {}",
                        e, e2
                    ));
                }
            }
        }
    }
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
