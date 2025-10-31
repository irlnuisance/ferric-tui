use super::msg::{Cmd, Msg};
use crate::{adapters, domain, event::Event};
use tokio::task;

pub fn spawn_all(cmds: Vec<Cmd>, tx: tokio::sync::mpsc::UnboundedSender<Event>) {
    for cmd in cmds {
        match cmd {
            Cmd::Noop => {}
            Cmd::ScanIso { query } => {
                let tx = tx.clone();
                task::spawn(async move {
                    let res =
                        task::spawn_blocking(move || domain::iso::scan_default_roots(&query)).await;
                    match res {
                        Ok(results) => {
                            let _: Result<_, _> = tx.send(Event::App(Msg::IsoResults(results)));
                        }
                        Err(e) => {
                            let _: Result<_, _> =
                                tx.send(Event::App(Msg::IsoSearchFailed(e.to_string())));
                        }
                    }
                });
            }
            Cmd::RefreshDevices => {
                let tx = tx.clone();
                task::spawn(async move {
                    let res = task::spawn_blocking(adapters::lsblk::refresh_devices).await;
                    match res {
                        Ok(devs) => {
                            let _: Result<_, _> = tx.send(Event::App(Msg::DevicesRefreshed(devs)));
                        }
                        Err(e) => {
                            let _: Result<_, _> =
                                tx.send(Event::App(Msg::DevicesRefreshFailed(e.to_string())));
                        }
                    }
                });
            }
            Cmd::Write { iso, device } => {
                let tx = tx.clone();
                task::spawn(async move {
                    let tx2 = tx.clone();
                    let res =
                        task::spawn_blocking(move || domain::writer::write_image(iso, device, tx2))
                            .await;
                    if let Err(e) = res {
                        let _: Result<_, _> = tx.send(Event::App(Msg::WriteFinished(Err(
                            format!("Join error: {}", e),
                        ))));
                    }
                });
            }
            Cmd::Verify { iso, device, size } => {
                let tx = tx.clone();
                task::spawn(async move {
                    let tx2 = tx.clone();
                    let res = task::spawn_blocking(move || {
                        domain::writer::verify_image(iso, device, size, tx2)
                    })
                    .await;
                    if let Err(e) = res {
                        let _: Result<_, _> = tx.send(Event::App(Msg::VerifyFinished(Err(
                            format!("Join error: {}", e),
                        ))));
                    }
                });
            }
            Cmd::ReexecWithSudo => {
                ratatui::restore();
                let exe = match std::env::current_exe() {
                    Ok(p) => p,
                    Err(e) => {
                        let _: Result<_, _> = tx.send(Event::App(Msg::WriteFinished(Err(
                            format!("Cannot determine current executable: {}", e),
                        ))));
                        return;
                    }
                };
                let args: Vec<std::ffi::OsString> = std::env::args_os().skip(1).collect();
                let mut cmd = std::process::Command::new("sudo");
                cmd.arg("-E").arg("--").arg(exe);
                for a in &args {
                    cmd.arg(a);
                }
                #[cfg(unix)]
                {
                    use std::os::unix::process::CommandExt;
                    let _err = cmd.exec();
                    let _: Result<_, _> = tx.send(Event::App(Msg::WriteFinished(Err(
                        "Failed to exec sudo".to_string(),
                    ))));
                    std::process::exit(1);
                }
                #[cfg(not(unix))]
                {
                    match cmd.status() {
                        Ok(st) if st.success() => std::process::exit(0),
                        _ => {
                            let _: Result<_, _> = tx.send(Event::App(Msg::WriteFinished(Err(
                                "Failed to run sudo".to_string(),
                            ))));
                        }
                    }
                }
            }
        }
    }
}
