use ratatui::crossterm::event::KeyEvent;

#[derive(Debug, Clone)]
pub enum Msg {
    Tick,
    Key(KeyEvent),
    Quit,
    Back,
    NextScreen,
    PrevScreen,
    // ISO search
    IsoQueryChanged(String),
    IsoSearchRequested,
    IsoResults(Vec<crate::app::state::IsoMeta>),
    IsoMoveSelection(i32),
    IsoConfirmSelect,
    // Devices
    DevicesRefreshed(Vec<crate::app::state::Device>),
    DeviceMoveSelection(i32),
    DeviceConfirmSelect,
    RefreshDevicesRequested,
    // Writing
    WriteStarted { total: u64 },
    WriteProgress { written: u64, total: u64, bps: f64 },
    WriteFinished(Result<(), String>),
    // Verification
    VerifyStarted { total: u64 },
    VerifyProgress { checked: u64, total: u64, bps: f64 },
    VerifyFinished(Result<(), String>),
    // Elevation
    ElevateRequested,
}

#[derive(Debug, Clone)]
pub enum Cmd {
    Noop,
    ScanIso { query: String },
    RefreshDevices,
    Write { iso: std::path::PathBuf, device: std::path::PathBuf },
    Verify { iso: std::path::PathBuf, device: std::path::PathBuf, size: u64 },
    ReexecWithSudo,
}
