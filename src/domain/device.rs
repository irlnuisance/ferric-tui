use crate::domain::{paths::DevicePath, units::ByteSize};

#[derive(Debug, Clone)]
pub struct Device {
    pub name: String,
    pub path: DevicePath,
    pub size: ByteSize,
    pub model: Option<String>,
    pub serial: Option<String>,
    pub tran: Option<String>,
    pub removable: bool,
    pub hotplug: bool,
    pub ro: bool,
    pub mounted: bool,
    pub labels: Vec<String>,
}

impl Device {
    pub fn is_safe_target(&self) -> bool {
        !self.ro
    }
}
