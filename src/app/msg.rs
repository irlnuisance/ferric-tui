use ratatui::crossterm::event::KeyEvent;

#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum Msg {
    Tick,
    Key(KeyEvent),
    Quit,
    Back,
    NextScreen,
    PrevScreen,

    IsoQueryChanged(String),
    IsoSearchRequested,
    IsoSearchFailed(String),
    IsoResults(Vec<crate::app::state::IsoMeta>),
    IsoMoveSelection(i32),
    IsoConfirmSelect,

    DevicesRefreshFailed(String),
    DevicesRefreshed(Vec<crate::app::state::Device>),
    DeviceMoveSelection(i32),
    DeviceConfirmSelect,
    RefreshDevicesRequested,

    WriteStarted { total: u64 },
    WriteProgress { written: u64, total: u64, bps: f64 },
    WriteFinished(Result<(), String>),

    VerifyStarted { total: u64 },
    VerifyProgress { checked: u64, total: u64, bps: f64 },
    VerifyFinished(Result<(), String>),

    ElevateRequested,
}

#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum Cmd {
    Noop,
    ScanIso {
        query: String,
    },
    RefreshDevices,
    Write {
        iso: crate::domain::paths::IsoPath,
        device: crate::domain::paths::DevicePath,
    },
    Verify {
        iso: crate::domain::paths::IsoPath,
        device: crate::domain::paths::DevicePath,
        size: u64,
    },
    ReexecWithSudo,
}
