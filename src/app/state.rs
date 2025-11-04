//! Application state model and UI state machine.
//!
//! The model is a single immutable struct updated by the event loop.
//! Counters are in bytes unless stated otherwise.

pub use crate::domain::device::Device;
pub use crate::domain::iso::IsoMeta;
pub use crate::domain::paths::{DevicePath, IsoPath};

/// UI screens
///
/// Order is not meaningful. The type is `#[non_exhaustive]` to allow
/// adding new screens without a breaking change.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum Screen {
    IsoSearch,
    DeviceSelect,
    Confirm,
    Writing,
    Done,
}

/// Which sub‑panel currently has focus for keyboard input.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivePanel {
    IsoList,
    DeviceList,
    ConfirmInput,
}

/// Full application model.
///
/// Invariants
/// - If `iso_results` is non‑empty, `iso_selected < iso_results.len()`
/// - If `devices` is non‑empty, `device_selected < devices.len()`
/// - While `is_writing()`, `write_result.is_none()`
/// - While `is_verifying()`, `verify_result.is_none()`
/// - `writing_written <= writing_total` and `verifying_checked <= verifying_total`
#[derive(Debug, Clone)]
pub struct Model {
    /// Current screen in the UI state machine
    pub screen: Screen,

    /// Current ISO query
    pub iso_query: String,
    /// Search results for query
    pub iso_results: Vec<IsoMeta>,
    pub iso_selected: usize,
    pub iso_searching: bool,
    pub iso_debounce_until: Option<std::time::Instant>,
    pub iso_chosen: Option<IsoPath>,

    /// Discovered writable block devices
    pub devices: Vec<Device>,
    pub device_selected: usize,
    pub device_refreshing: bool,
    pub device_chosen: Option<DevicePath>,

    /// Raw user input for destructive action confirmation
    pub confirm_input: String,

    /// Total number of bytes to write
    pub writing_total: u64,
    /// Number of bytes written so far
    pub writing_written: u64,
    /// Start time of the current write, if any
    pub writing_started: Option<std::time::Instant>,
    /// Estimated write throughput in bytes/sec
    pub writing_speed_bps: f64,
    /// Result of the write: `None` while running; `Some(Ok(()))` on success
    /// `Some(Err(msg))` on failure
    pub write_result: Option<Result<(), String>>,

    pub verify_after_write: bool,
    pub verifying: bool,
    /// Total number of bytes to verify
    pub verifying_total: u64,
    /// Number of bytes verified so far
    pub verifying_checked: u64,
    /// Estimated verify throughput in bytes/sec
    pub verifying_speed_bps: f64,
    /// Result of the verify: `None` while running; `Some(Ok(()))` on success
    /// `Some(Err(msg))` on failure
    pub verify_result: Option<Result<(), String>>,

    pub is_root: bool,

    pub active_panel: ActivePanel,
}

impl Model {
    /// True when both an ISO and a device have been selected
    pub fn has_both_selections(&self) -> bool {
        self.iso_chosen.is_some() && self.device_chosen.is_some()
    }

    /// True when model has the prerequisites to start a write
    ///
    /// Requires running as root and both selections
    pub fn can_write(&self) -> bool {
        self.is_root && self.has_both_selections()
    }

    /// True while write screen is active and the write has not finished
    pub fn is_writing(&self) -> bool {
        matches!(self.screen, Screen::Writing) && self.write_result.is_none()
    }

    /// True while verifying is in progress
    pub fn is_verifying(&self) -> bool {
        self.verifying && self.verify_result.is_none()
    }

    /// True when confirmation input matches the input string
    pub fn is_confirmation_valid(&self) -> bool {
        self.confirm_input == "YES"
    }
}

impl Default for Model {
    /// Iniy an empty model with sensible defaults
    fn default() -> Self {
        Self {
            screen: Screen::IsoSearch,
            iso_query: String::new(),
            iso_results: Vec::new(),
            iso_selected: 0,
            iso_searching: false,
            iso_debounce_until: None,
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
