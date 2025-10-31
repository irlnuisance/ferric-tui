use ferric::app::msg::{Cmd, Msg};
use ferric::app::state::{Device, IsoMeta, Model, Screen};
use ferric::app::update::update;
use ferric::domain::{ByteSize, DevicePath, IsoPath};
use std::path::PathBuf;

fn make_test_iso(path: &str, size: u64) -> IsoMeta {
    IsoMeta {
        path: IsoPath::from(PathBuf::from(path)),
        size: ByteSize::from(size),
        modified: None,
    }
}

fn make_test_device(name: &str, path: &str) -> Device {
    Device {
        name: name.to_string(),
        path: DevicePath::from(PathBuf::from(path)),
        size: ByteSize::from(16_000_000_000),
        model: Some("Test USB Drive".to_string()),
        serial: None,
        tran: Some("usb".to_string()),
        removable: true,
        hotplug: true,
        ro: false,
        mounted: false,
        labels: vec![],
    }
}

#[test]
fn test_iso_selection() {
    let mut model = Model::default();
    model.screen = Screen::IsoSearch;
    model.iso_results = vec![
        make_test_iso("/home/user/ubuntu.iso", 3_000_000_000),
        make_test_iso("/home/user/debian.iso", 4_000_000_000),
    ];
    model.iso_selected = 1;

    let (new_model, cmds) = update(model, Msg::IsoConfirmSelect);

    assert_eq!(
        new_model.iso_chosen,
        Some(IsoPath::from(PathBuf::from("/home/user/debian.iso")))
    );
    assert!(matches!(new_model.screen, Screen::DeviceSelect));
    assert!(new_model.device_refreshing);

    assert_eq!(cmds.len(), 1);
    assert!(matches!(cmds[0], Cmd::RefreshDevices));
}

#[test]
fn test_device_selection() {
    let mut model = Model::default();
    model.screen = Screen::DeviceSelect;
    model.iso_chosen = Some(IsoPath::from(PathBuf::from("/home/user/test.iso")));
    model.devices = vec![
        make_test_device("sdb", "/dev/sdb"),
        make_test_device("sdc", "/dev/sdc"),
    ];
    model.device_selected = 0;

    let (new_model, cmds) = update(model, Msg::DeviceConfirmSelect);

    assert_eq!(
        new_model.device_chosen,
        Some(DevicePath::from(PathBuf::from("/dev/sdb")))
    );
    assert!(matches!(new_model.screen, Screen::Confirm));
    assert_eq!(new_model.confirm_input, "");

    assert_eq!(cmds.len(), 0);
}

#[test]
fn test_iso_query_change() {
    let mut model = Model::default();
    model.screen = Screen::IsoSearch;
    model.iso_results = vec![
        make_test_iso("/home/user/test1.iso", 1_000_000),
        make_test_iso("/home/user/test2.iso", 2_000_000),
    ];
    model.iso_selected = 1;

    let (new_model, cmds) = update(model, Msg::IsoQueryChanged("ubuntu".to_string()));

    assert_eq!(new_model.iso_query, "ubuntu");
    assert_eq!(new_model.iso_selected, 0);
    assert!(new_model.iso_searching);

    assert_eq!(cmds.len(), 1);
    match &cmds[0] {
        Cmd::ScanIso { query } => assert_eq!(query, "ubuntu"),
        _ => panic!("Expected ScanIso command"),
    }
}

#[test]
fn test_write_lifecycle() {
    let mut model = Model::default();
    model.screen = Screen::Writing;

    let (model, cmds) = update(
        model,
        Msg::WriteStarted {
            total: 5_000_000_000,
        },
    );
    assert_eq!(model.writing_total, 5_000_000_000);
    assert_eq!(model.writing_written, 0);
    assert!(model.writing_started.is_some());
    assert_eq!(cmds.len(), 0);

    let (model, cmds) = update(
        model,
        Msg::WriteProgress {
            written: 1_000_000_000,
            total: 5_000_000_000,
            bps: 50_000_000.0,
        },
    );
    assert_eq!(model.writing_written, 1_000_000_000);
    assert_eq!(model.writing_speed_bps, 50_000_000.0);
    assert_eq!(cmds.len(), 0);

    let (model, cmds) = update(model, Msg::WriteFinished(Ok(())));
    assert_eq!(model.write_result, Some(Ok(())));
    assert!(matches!(model.screen, Screen::Done));
    assert_eq!(cmds.len(), 0);
}

#[test]
fn test_write_lifecycle_with_verify() {
    let mut model = Model::default();
    model.screen = Screen::Writing;
    model.verify_after_write = true;
    model.iso_chosen = Some(IsoPath::from(PathBuf::from("/test.iso")));
    model.device_chosen = Some(DevicePath::from(PathBuf::from("/dev/sdb")));

    let (model, _) = update(model, Msg::WriteStarted { total: 1_000_000 });

    let (model, cmds) = update(model, Msg::WriteFinished(Ok(())));

    assert_eq!(model.write_result, Some(Ok(())));
    assert!(model.verifying);
    assert_eq!(model.verifying_total, 1_000_000);

    assert_eq!(cmds.len(), 1);
    match &cmds[0] {
        Cmd::Verify { iso, device, size } => {
            assert_eq!(iso, &IsoPath::from(PathBuf::from("/test.iso")));
            assert_eq!(device, &DevicePath::from(PathBuf::from("/dev/sdb")));
            assert_eq!(*size, 1_000_000);
        }
        _ => panic!("Expected Verify command"),
    }
}

#[test]
fn test_iso_move_selection() {
    let mut model = Model::default();
    model.iso_results = vec![
        make_test_iso("/home/user/test1.iso", 1_000_000),
        make_test_iso("/home/user/test2.iso", 2_000_000),
        make_test_iso("/home/user/test3.iso", 3_000_000),
    ];
    model.iso_selected = 0;

    let (model, _) = update(model, Msg::IsoMoveSelection(1));
    assert_eq!(model.iso_selected, 1);

    let (model, _) = update(model, Msg::IsoMoveSelection(1));
    assert_eq!(model.iso_selected, 2);

    let (model, _) = update(model, Msg::IsoMoveSelection(5));
    assert_eq!(model.iso_selected, 2);

    let (model, _) = update(model, Msg::IsoMoveSelection(-1));
    assert_eq!(model.iso_selected, 1);

    let (model, _) = update(model, Msg::IsoMoveSelection(-5));
    assert_eq!(model.iso_selected, 0);
}

#[test]
fn test_predictability() {
    let model = Model::default();
    let msg = Msg::IsoQueryChanged("test".to_string());

    let (result1, cmds1) = update(model.clone(), msg.clone());
    let (result2, cmds2) = update(model.clone(), msg.clone());

    assert_eq!(result1.iso_query, result2.iso_query);
    assert_eq!(result1.iso_selected, result2.iso_selected);
    assert_eq!(result1.iso_searching, result2.iso_searching);
    assert_eq!(cmds1.len(), cmds2.len());
}

#[test]
fn test_tab_navigation_without_selections() {
    let model = Model::default();
    assert!(matches!(model.screen, Screen::IsoSearch));

    let (model, _) = update(model, Msg::NextScreen);
    assert!(matches!(model.screen, Screen::DeviceSelect));

    let (model, _) = update(model, Msg::NextScreen);
    assert!(matches!(model.screen, Screen::IsoSearch));
}

#[test]
fn test_tab_navigation_with_selections() {
    let mut model = Model::default();
    model.iso_chosen = Some(IsoPath::from(PathBuf::from("/test.iso")));
    model.device_chosen = Some(DevicePath::from(PathBuf::from("/dev/sdb")));
    model.confirm_input = "YES".to_string();
    model.screen = Screen::IsoSearch;

    let (model, _) = update(model, Msg::NextScreen);
    assert!(matches!(model.screen, Screen::DeviceSelect));

    let (model, _) = update(model, Msg::NextScreen);
    assert!(matches!(model.screen, Screen::Confirm));

    let (model, _) = update(model, Msg::NextScreen);
    assert!(matches!(model.screen, Screen::IsoSearch));
}

#[test]
fn test_back_navigation() {
    let mut model = Model::default();
    model.screen = Screen::IsoSearch;
    let (model, _) = update(model, Msg::Back);
    assert!(matches!(model.screen, Screen::IsoSearch));

    let mut model = Model::default();
    model.screen = Screen::DeviceSelect;
    let (model, _) = update(model, Msg::Back);
    assert!(matches!(model.screen, Screen::IsoSearch));

    let mut model = Model::default();
    model.screen = Screen::Confirm;
    let (model, _) = update(model, Msg::Back);
    assert!(matches!(model.screen, Screen::DeviceSelect));

    let mut model = Model::default();
    model.screen = Screen::Writing;
    let (model, _) = update(model, Msg::Back);
    assert!(matches!(model.screen, Screen::Confirm));

    let mut model = Model::default();
    model.screen = Screen::Done;
    let (model, _) = update(model, Msg::Back);
    assert!(matches!(model.screen, Screen::DeviceSelect));
}

#[test]
fn test_confirmation_requires_yes_input() {
    let mut model = Model::default();
    model.screen = Screen::Confirm;
    model.iso_chosen = Some(IsoPath::from(PathBuf::from("/test.iso")));
    model.device_chosen = Some(DevicePath::from(PathBuf::from("/dev/sdb")));

    model.confirm_input = "".to_string();
    let (new_model, cmds) = update(
        model.clone(),
        Msg::Key(ratatui::crossterm::event::KeyEvent {
            code: ratatui::crossterm::event::KeyCode::Enter,
            modifiers: ratatui::crossterm::event::KeyModifiers::empty(),
            kind: ratatui::crossterm::event::KeyEventKind::Press,
            state: ratatui::crossterm::event::KeyEventState::empty(),
        }),
    );
    assert!(matches!(new_model.screen, Screen::Confirm));
    assert_eq!(cmds.len(), 0);

    model.confirm_input = "yes".to_string();
    let (new_model, cmds) = update(
        model.clone(),
        Msg::Key(ratatui::crossterm::event::KeyEvent {
            code: ratatui::crossterm::event::KeyCode::Enter,
            modifiers: ratatui::crossterm::event::KeyModifiers::empty(),
            kind: ratatui::crossterm::event::KeyEventKind::Press,
            state: ratatui::crossterm::event::KeyEventState::empty(),
        }),
    );
    assert!(matches!(new_model.screen, Screen::Confirm));
    assert_eq!(cmds.len(), 0);

    model.confirm_input = "YES".to_string();
    let (new_model, cmds) = update(
        model,
        Msg::Key(ratatui::crossterm::event::KeyEvent {
            code: ratatui::crossterm::event::KeyCode::Enter,
            modifiers: ratatui::crossterm::event::KeyModifiers::empty(),
            kind: ratatui::crossterm::event::KeyEventKind::Press,
            state: ratatui::crossterm::event::KeyEventState::empty(),
        }),
    );
    assert!(matches!(new_model.screen, Screen::Writing));
    assert_eq!(cmds.len(), 1);
    assert!(matches!(cmds[0], Cmd::Write { .. }));
}

#[test]
fn test_device_refresh_command() {
    let mut model = Model::default();
    model.screen = Screen::DeviceSelect;

    let (new_model, cmds) = update(model, Msg::RefreshDevicesRequested);

    assert!(new_model.device_refreshing);
    assert_eq!(cmds.len(), 1);
    assert!(matches!(cmds[0], Cmd::RefreshDevices));
}

#[test]
fn test_verify_finished_transitions_to_done() {
    let mut model = Model::default();
    model.screen = Screen::Writing;
    model.verifying = true;

    let (new_model, cmds) = update(model, Msg::VerifyFinished(Ok(())));

    assert_eq!(new_model.verify_result, Some(Ok(())));
    assert!(!new_model.verifying);
    assert!(matches!(new_model.screen, Screen::Done));
    assert_eq!(cmds.len(), 0);
}
