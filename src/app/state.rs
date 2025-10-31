pub use crate::domain::device::Device;
pub use crate::domain::iso::IsoMeta;
pub use crate::domain::paths::{DevicePath, IsoPath};

#[non_exhaustive]
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
pub struct Model {
    pub screen: Screen,

    pub iso_query: String,
    pub iso_results: Vec<IsoMeta>,
    pub iso_selected: usize,
    pub iso_searching: bool,
    pub iso_chosen: Option<IsoPath>,

    pub devices: Vec<Device>,
    pub device_selected: usize,
    pub device_refreshing: bool,
    pub device_chosen: Option<DevicePath>,

    pub confirm_input: String,

    pub writing_total: u64,
    pub writing_written: u64,
    pub writing_started: Option<std::time::Instant>,
    pub writing_speed_bps: f64,
    pub write_result: Option<Result<(), String>>,

    pub verify_after_write: bool,
    pub verifying: bool,
    pub verifying_total: u64,
    pub verifying_checked: u64,
    pub verifying_speed_bps: f64,
    pub verify_result: Option<Result<(), String>>,

    pub is_root: bool,

    pub active_panel: ActivePanel,
}

impl Model {
    pub fn has_both_selections(&self) -> bool {
        self.iso_chosen.is_some() && self.device_chosen.is_some()
    }

    pub fn can_write(&self) -> bool {
        self.is_root && self.has_both_selections()
    }

    pub fn is_writing(&self) -> bool {
        matches!(self.screen, Screen::Writing) && self.write_result.is_none()
    }

    pub fn is_verifying(&self) -> bool {
        self.verifying && self.verify_result.is_none()
    }

    pub fn is_confirmation_valid(&self) -> bool {
        self.confirm_input == "YES"
    }
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
            is_root: crate::adapters::platform::is_root(),
            active_panel: ActivePanel::IsoList,
        }
    }
}
