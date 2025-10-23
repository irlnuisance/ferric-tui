use std::{path::PathBuf, time::SystemTime};

#[derive(Debug, Clone)]
pub enum Screen {
    IsoSearch,
    DeviceSelect,
    Confirm,
    Writing,
    Done,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivePanel {
    IsoList,
    DeviceList,
    ConfirmInput,
}

#[derive(Debug, Clone)]
pub struct IsoMeta {
    pub path: PathBuf,
    pub size: u64,
    pub modified: Option<SystemTime>,
}

#[derive(Debug, Clone)]
pub struct Device {
    pub name: String,
    pub path: PathBuf,
    pub size: u64,
    pub model: Option<String>,
    pub serial: Option<String>,
    pub tran: Option<String>,
    pub removable: bool,
    pub hotplug: bool,
    pub ro: bool,
    pub mounted: bool,
    pub labels: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub screen: Screen,
    // ISO search state
    pub iso_query: String,
    pub iso_results: Vec<IsoMeta>,
    pub iso_selected: usize,
    pub iso_searching: bool,
    pub iso_chosen: Option<PathBuf>,
    // Device selection state
    pub devices: Vec<Device>,
    pub device_selected: usize,
    pub device_refreshing: bool,
    pub device_chosen: Option<PathBuf>,
    // Confirm screen
    pub confirm_input: String,
    // Writing
    pub writing_total: u64,
    pub writing_written: u64,
    pub writing_started: Option<std::time::Instant>,
    pub writing_speed_bps: f64,
    pub write_result: Option<Result<(), String>>,
    // Verification
    pub verify_after_write: bool,
    pub verifying: bool,
    pub verifying_total: u64,
    pub verifying_checked: u64,
    pub verifying_speed_bps: f64,
    pub verify_result: Option<Result<(), String>>,
    // Environment
    pub is_root: bool,
    // UI State
    pub active_panel: ActivePanel,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            screen: Screen::IsoSearch,
            iso_query: String::new(),
            iso_results: Vec::new(),
            iso_selected: 0,
            iso_searching: false,
            iso_chosen: None,
            devices: Vec::new(),
            device_selected: 0,
            device_refreshing: false,
            device_chosen: None,
            confirm_input: String::new(),
            writing_total: 0,
            writing_written: 0,
            writing_started: None,
            writing_speed_bps: 0.0,
            write_result: None,
            verify_after_write: false,
            verifying: false,
            verifying_total: 0,
            verifying_checked: 0,
            verifying_speed_bps: 0.0,
            verify_result: None,
            is_root: detect_is_root(),
            active_panel: ActivePanel::IsoList,
        }
    }
}

fn detect_is_root() -> bool {
    // Linux-only: parse /proc/self/status Uid line. Fallback: false.
    if let Ok(status) = std::fs::read_to_string("/proc/self/status") {
        for line in status.lines() {
            if let Some(rest) = line.strip_prefix("Uid:") {
                let mut it = rest.split_whitespace();
                if let Some(real) = it.next() {
                    if real == "0" { return true; }
                }
                break;
            }
        }
    }
    false
}
