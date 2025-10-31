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
        !self.ro && !self.mounted
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn make_device(ro: bool, mounted: bool) -> Device {
        Device {
            name: "sdx".to_string(),
            path: DevicePath::from(PathBuf::from("/dev/sdx")),
            size: ByteSize::from(1_000_000),
            model: None,
            serial: None,
            tran: None,
            removable: true,
            hotplug: true,
            ro,
            mounted,
            labels: vec![],
        }
    }

    #[test]
    fn mounted_device_is_not_safe_target() {
        let d = make_device(false, true);
        assert!(!d.is_safe_target());
    }
}
