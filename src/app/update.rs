use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use super::{
    msg::{Cmd, Msg},
    state::{ActivePanel, Model, Screen},
};

pub fn update(mut model: Model, msg: Msg) -> (Model, Vec<Cmd>) {
    match msg {
        Msg::Tick => {
            if matches!(model.screen, Screen::IsoSearch)
                && !model.iso_searching
                && model.iso_results.is_empty()
            {
                model.iso_searching = true;
                let query = model.iso_query.clone();
                return (model, vec![Cmd::ScanIso { query }]);
            }
            if matches!(model.screen, Screen::DeviceSelect)
                && !model.device_refreshing
                && model.devices.is_empty()
            {
                model.device_refreshing = true;
                return (model, vec![Cmd::RefreshDevices]);
            }
        }
        Msg::Quit => {}
        Msg::Back => match model.screen {
            Screen::IsoSearch => {}
            Screen::DeviceSelect => {
                model.screen = Screen::IsoSearch;
                model.active_panel = sync_active_panel(&model.screen);
            }
            Screen::Confirm => {
                model.screen = Screen::DeviceSelect;
                model.active_panel = sync_active_panel(&model.screen);
            }
            Screen::Writing => {
                model.screen = Screen::Confirm;
                model.active_panel = sync_active_panel(&model.screen);
            }
            Screen::Done => {
                model.screen = Screen::DeviceSelect;
                model.active_panel = sync_active_panel(&model.screen);
            }
        },
        Msg::Key(key) => {
            if key.kind == KeyEventKind::Press {
                if let Some(cmd) = handle_key(&mut model, key) {
                    return (model, vec![cmd]);
                }
            }
        }
        Msg::NextScreen => {
            model.screen = next_screen(&model);
            model.active_panel = sync_active_panel(&model.screen);
        }
        Msg::PrevScreen => {
            model.screen = prev_screen(&model);
            model.active_panel = sync_active_panel(&model.screen);
        }
        Msg::IsoQueryChanged(q) => {
            model.iso_query = q;
            model.iso_selected = 0;
            model.iso_searching = true;
            let query = model.iso_query.clone();
            return (model, vec![Cmd::ScanIso { query }]);
        }
        Msg::IsoSearchRequested => {
            model.iso_searching = true;
            let query = model.iso_query.clone();
            return (model, vec![Cmd::ScanIso { query }]);
        }
        Msg::IsoSearchFailed(_err) => {
            model.iso_searching = false;
        }
        Msg::IsoResults(results) => {
            model.iso_results = results;
            if model.iso_selected >= model.iso_results.len() {
                model.iso_selected = model.iso_results.len().saturating_sub(1);
            }
            model.iso_searching = false;
        }
        Msg::IsoMoveSelection(delta) => {
            let len = model.iso_results.len();
            if len > 0 {
                let idx = model.iso_selected as isize + delta as isize;
                let idx = idx.clamp(0, (len - 1) as isize) as usize;
                model.iso_selected = idx;
            }
        }
        Msg::IsoConfirmSelect => {
            if let Some(meta) = model.iso_results.get(model.iso_selected).cloned() {
                model.iso_chosen = Some(meta.path);
                model.screen = Screen::DeviceSelect;
                model.active_panel = sync_active_panel(&model.screen);
                model.device_refreshing = true;
                return (model, vec![Cmd::RefreshDevices]);
            }
        }
        Msg::DevicesRefreshFailed(_err) => {
            model.device_refreshing = false;
        }
        Msg::DevicesRefreshed(devs) => {
            model.devices = devs;
            if model.device_selected >= model.devices.len() {
                model.device_selected = model.devices.len().saturating_sub(1);
            }
            model.device_refreshing = false;
        }
        Msg::DeviceMoveSelection(delta) => {
            let len = model.devices.len();
            if len > 0 {
                let idx = model.device_selected as isize + delta as isize;
                let idx = idx.clamp(0, (len - 1) as isize) as usize;
                model.device_selected = idx;
            }
        }
        Msg::DeviceConfirmSelect => {
            if let Some(dev) = model.devices.get(model.device_selected).cloned() {
                model.device_chosen = Some(dev.path);
                model.confirm_input.clear();
                model.screen = Screen::Confirm;
                model.active_panel = sync_active_panel(&model.screen);
            }
        }
        Msg::RefreshDevicesRequested => {
            model.device_refreshing = true;
            return (model, vec![Cmd::RefreshDevices]);
        }
        Msg::ElevateRequested => {
            if !model.is_root {
                return (model, vec![Cmd::ReexecWithSudo]);
            }
        }
        Msg::WriteStarted { total } => {
            model.writing_total = total;
            model.writing_written = 0;
            model.writing_started = Some(std::time::Instant::now());
            model.writing_speed_bps = 0.0;
        }
        Msg::WriteProgress {
            written,
            total,
            bps,
        } => {
            model.writing_written = written;
            model.writing_total = total;
            model.writing_speed_bps = bps;
        }
        Msg::WriteFinished(result) => {
            model.write_result = Some(result);
            if model.write_result == Some(Ok(())) && model.verify_after_write {
                let total = model.writing_total;
                model.verifying = true;
                model.verifying_total = total;
                if let (Some(iso), Some(dev)) =
                    (model.iso_chosen.clone(), model.device_chosen.clone())
                {
                    return (
                        model,
                        vec![Cmd::Verify {
                            iso,
                            device: dev,
                            size: total,
                        }],
                    );
                }
            } else {
                model.screen = Screen::Done;
                model.active_panel = sync_active_panel(&model.screen);
            }
        }
        Msg::VerifyStarted { total } => {
            model.verifying = true;
            model.verifying_total = total;
            model.verifying_checked = 0;
            model.verifying_speed_bps = 0.0;
        }
        Msg::VerifyProgress {
            checked,
            total,
            bps,
        } => {
            model.verifying_checked = checked;
            model.verifying_total = total;
            model.verifying_speed_bps = bps;
        }
        Msg::VerifyFinished(result) => {
            model.verify_result = Some(result);
            model.verifying = false;
            model.screen = Screen::Done;
            model.active_panel = sync_active_panel(&model.screen);
        }
    }
    (model, Vec::new())
}

fn handle_key(model: &mut Model, key: KeyEvent) -> Option<Cmd> {
    match model.screen {
        Screen::IsoSearch => match key.code {
            KeyCode::Tab => {
                model.screen = next_screen(&model);
                model.active_panel = sync_active_panel(&model.screen);
            }
            KeyCode::BackTab => {
                model.screen = prev_screen(&model);
                model.active_panel = sync_active_panel(&model.screen);
            }
            KeyCode::Up => {
                let len = model.iso_results.len();
                if len > 0 {
                    if model.iso_selected > 0 {
                        model.iso_selected -= 1;
                    }
                }
            }
            KeyCode::Down => {
                let len = model.iso_results.len();
                if len > 0 {
                    if model.iso_selected + 1 < len {
                        model.iso_selected += 1;
                    }
                }
            }
            KeyCode::Enter => {
                if let Some(meta) = model.iso_results.get(model.iso_selected).cloned() {
                    model.iso_chosen = Some(meta.path);
                    model.screen = Screen::DeviceSelect;
                    model.active_panel = sync_active_panel(&model.screen);
                    return Some(Cmd::RefreshDevices);
                }
            }
            KeyCode::Backspace => {
                model.iso_query.pop();
                model.iso_selected = 0;
                model.iso_searching = true;
                return Some(Cmd::ScanIso {
                    query: model.iso_query.clone(),
                });
            }
            KeyCode::Char(c) => {
                model.iso_query.push(c);
                model.iso_selected = 0;
                model.iso_searching = true;
                return Some(Cmd::ScanIso {
                    query: model.iso_query.clone(),
                });
            }
            _ => {}
        },
        Screen::DeviceSelect => match key.code {
            KeyCode::Tab => {
                model.screen = next_screen(model);
                model.active_panel = sync_active_panel(&model.screen);
            }
            KeyCode::BackTab => {
                model.screen = prev_screen(model);
                model.active_panel = sync_active_panel(&model.screen);
            }
            KeyCode::Up => {
                let len = model.devices.len();
                if len > 0 && model.device_selected > 0 {
                    model.device_selected -= 1;
                }
            }
            KeyCode::Down => {
                let len = model.devices.len();
                if len > 0 && model.device_selected + 1 < len {
                    model.device_selected += 1;
                }
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                model.device_refreshing = true;
                return Some(Cmd::RefreshDevices);
            }
            KeyCode::Enter => {
                if let Some(dev) = model.devices.get(model.device_selected).cloned() {
                    model.device_chosen = Some(dev.path);
                    model.confirm_input.clear();
                    model.screen = Screen::Confirm;
                    model.active_panel = sync_active_panel(&model.screen);
                }
            }
            _ => {}
        },
        Screen::Confirm => match key.code {
            KeyCode::Tab => {
                model.screen = next_screen(model);
                model.active_panel = sync_active_panel(&model.screen);
            }
            KeyCode::BackTab => {
                model.screen = prev_screen(model);
                model.active_panel = sync_active_panel(&model.screen);
            }
            KeyCode::Backspace => {
                model.confirm_input.pop();
            }
            KeyCode::Enter => {
                if can_confirm(model) {
                    model.screen = Screen::Writing;
                    model.active_panel = sync_active_panel(&model.screen);
                    model.writing_written = 0;
                    model.writing_total = 0;
                    model.writing_started = Some(std::time::Instant::now());
                    model.writing_speed_bps = 0.0;
                    if let (Some(iso), Some(dev)) =
                        (model.iso_chosen.clone(), model.device_chosen.clone())
                    {
                        return Some(Cmd::Write { iso, device: dev });
                    }
                }
            }
            KeyCode::Char('s') | KeyCode::Char('S')
                if key.modifiers.contains(KeyModifiers::CONTROL) =>
            {
                if !model.is_root {
                    return Some(Cmd::ReexecWithSudo);
                }
            }
            KeyCode::Char('v') | KeyCode::Char('V') => {
                model.verify_after_write = !model.verify_after_write;
            }
            KeyCode::Char(c) => {
                if c.is_ascii_alphabetic() && model.confirm_input.len() < 3 {
                    model.confirm_input.push(c.to_ascii_uppercase());
                }
            }
            _ => {}
        },
        Screen::Writing => match key.code {
            KeyCode::Tab => {
                model.screen = next_screen(model);
                model.active_panel = sync_active_panel(&model.screen);
            }
            KeyCode::BackTab => {
                model.screen = prev_screen(model);
                model.active_panel = sync_active_panel(&model.screen);
            }
            _ => {}
        },
        _ => match key.code {
            KeyCode::Tab => {
                model.screen = next_screen(model);
                model.active_panel = sync_active_panel(&model.screen);
            }
            KeyCode::BackTab => {
                model.screen = prev_screen(model);
                model.active_panel = sync_active_panel(&model.screen);
            }
            _ => {}
        },
    }
    None
}

fn next_screen(model: &Model) -> Screen {
    let can_confirm = can_confirm(model);
    match model.screen {
        Screen::IsoSearch => Screen::DeviceSelect,
        Screen::DeviceSelect => {
            if can_confirm {
                Screen::Confirm
            } else {
                Screen::IsoSearch
            }
        }
        Screen::Confirm => Screen::IsoSearch,
        Screen::Writing => Screen::Writing,
        Screen::Done => Screen::Done,
    }
}

fn prev_screen(model: &Model) -> Screen {
    let can_confirm = can_confirm(model);
    match model.screen {
        Screen::IsoSearch => {
            if can_confirm {
                Screen::Confirm
            } else {
                Screen::DeviceSelect
            }
        }
        Screen::DeviceSelect => Screen::IsoSearch,
        Screen::Confirm => Screen::DeviceSelect,
        Screen::Writing => Screen::Confirm,
        Screen::Done => Screen::DeviceSelect,
    }
}

fn can_confirm(model: &Model) -> bool {
    model.iso_chosen.is_some() && model.device_chosen.is_some() && model.confirm_input == "YES"
}

fn sync_active_panel(screen: &Screen) -> ActivePanel {
    match screen {
        Screen::IsoSearch => ActivePanel::IsoList,
        Screen::DeviceSelect => ActivePanel::DeviceList,
        Screen::Confirm => ActivePanel::ConfirmInput,
        Screen::Writing | Screen::Done => ActivePanel::ConfirmInput,
    }
}
