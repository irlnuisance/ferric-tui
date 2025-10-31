use crate::{
    app::msg::Msg,
    domain::paths::{DevicePath, IsoPath},
    event::Event,
};
use std::{
    fs::OpenOptions,
    io::{Read, Write},
};

pub fn write_image(
    iso_path: IsoPath,
    device_path: DevicePath,
    tx: tokio::sync::mpsc::UnboundedSender<Event>,
) {
    let _ = crate::adapters::platform::unmount_partitions_of(device_path.as_path());
    let mut src = match std::fs::File::open(iso_path.as_path()) {
        Ok(f) => f,
        Err(e) => {
            let _: Result<_, _> = tx.send(Event::App(Msg::WriteFinished(Err(format!(
                "Failed to open ISO: {}",
                e
            )))));
            return;
        }
    };
    let total = match src.metadata().map(|m| m.len()) {
        Ok(sz) => sz,
        Err(e) => {
            let _: Result<_, _> = tx.send(Event::App(Msg::WriteFinished(Err(format!(
                "Failed to stat ISO: {}",
                e
            )))));
            return;
        }
    };
    let _: Result<_, _> = tx.send(Event::App(Msg::WriteStarted { total }));
    let mut dst = match OpenOptions::new().write(true).open(device_path.as_path()) {
        Ok(f) => f,
        Err(e) => {
            let _: Result<_, _> = tx.send(Event::App(Msg::WriteFinished(Err(format!(
                "Failed to open device {}: {}",
                device_path, e
            )))));
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
                        Ok(w) => {
                            off += w;
                            written += w as u64;
                        }
                        Err(e) => {
                            let _: Result<_, _> = tx.send(Event::App(Msg::WriteFinished(Err(
                                format!("Write error: {}", e),
                            ))));
                            return;
                        }
                    }
                }
                let elapsed = start.elapsed().as_secs_f64().max(0.000_001);
                let bps = (written as f64) / elapsed;
                let _: Result<_, _> = tx.send(Event::App(Msg::WriteProgress {
                    written,
                    total,
                    bps,
                }));
            }
            Err(e) => {
                let _: Result<_, _> = tx.send(Event::App(Msg::WriteFinished(Err(format!(
                    "Read error: {}",
                    e
                )))));
                return;
            }
        }
    }

    if let Err(e) = dst.flush() {
        let _: Result<_, _> = tx.send(Event::App(Msg::WriteFinished(Err(format!(
            "Flush error: {}",
            e
        )))));
        return;
    }
    if let Err(e) = dst.sync_all() {
        let _: Result<_, _> = tx.send(Event::App(Msg::WriteFinished(Err(format!(
            "sync_all error: {}",
            e
        )))));
        return;
    }

    let _: Result<(), String> = crate::adapters::platform::partprobe(device_path.as_path());

    let _: Result<_, _> = tx.send(Event::App(Msg::WriteFinished(Ok(()))));
}

pub fn verify_image(
    iso_path: IsoPath,
    device_path: DevicePath,
    size: u64,
    tx: tokio::sync::mpsc::UnboundedSender<Event>,
) {
    let mut iso = match std::fs::File::open(iso_path.as_path()) {
        Ok(f) => f,
        Err(e) => {
            let _: Result<_, _> = tx.send(Event::App(Msg::VerifyFinished(Err(format!(
                "Failed to open ISO for verify: {}",
                e
            )))));
            return;
        }
    };
    let mut dev = match std::fs::File::open(device_path.as_path()) {
        Ok(f) => f,
        Err(e) => {
            let _: Result<_, _> = tx.send(Event::App(Msg::VerifyFinished(Err(format!(
                "Failed to open device for verify: {}",
                e
            )))));
            return;
        }
    };
    let _: Result<_, _> = tx.send(Event::App(Msg::VerifyStarted { total: size }));
    let mut left = size;
    let mut checked: u64 = 0;
    let mut buf_iso = vec![0u8; 4 * 1024 * 1024];
    let mut buf_dev = vec![0u8; 4 * 1024 * 1024];
    let start = std::time::Instant::now();
    while left > 0 {
        let to_read = std::cmp::min(left, buf_iso.len() as u64) as usize;
        if let Err(e) = iso.read_exact(&mut buf_iso[..to_read]) {
            let _: Result<_, _> = tx.send(Event::App(Msg::VerifyFinished(Err(format!(
                "ISO read error during verify: {}",
                e
            )))));
            return;
        }
        if let Err(e) = dev.read_exact(&mut buf_dev[..to_read]) {
            let _: Result<_, _> = tx.send(Event::App(Msg::VerifyFinished(Err(format!(
                "Device read error during verify: {}",
                e
            )))));
            return;
        }
        if buf_iso[..to_read] != buf_dev[..to_read] {
            let _: Result<_, _> = tx.send(Event::App(Msg::VerifyFinished(Err(
                "Mismatch between ISO and device".to_string(),
            ))));
            return;
        }
        left -= to_read as u64;
        checked += to_read as u64;
        let elapsed = start.elapsed().as_secs_f64().max(0.000_001);
        let bps = (checked as f64) / elapsed;
        let _: Result<_, _> = tx.send(Event::App(Msg::VerifyProgress {
            checked,
            total: size,
            bps,
        }));
    }
    let _: Result<_, _> = tx.send(Event::App(Msg::VerifyFinished(Ok(()))));
}
