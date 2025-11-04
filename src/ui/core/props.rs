use crate::app::state::{ActivePanel, Model};
use crate::domain::{Device, DevicePath, IsoMeta, IsoPath};

#[derive(Debug, Clone)]
pub struct IsoSearchProps<'a> {
    pub query: &'a str,
    pub searching: bool,
    pub results: &'a [IsoMeta],
    pub selected: usize,
    pub focused: bool,
}

#[derive(Debug, Clone)]
pub struct DeviceSelectProps<'a> {
    pub devices: &'a [Device],
    pub selected: usize,
    pub refreshing: bool,
    pub focused: bool,
}

#[derive(Debug, Clone)]
pub struct ConfirmProps<'a> {
    pub iso_path: Option<&'a IsoPath>,
    pub device_path: Option<&'a DevicePath>,
    pub confirm_input: &'a str,
    pub verify_after_write: bool,
    pub is_root: bool,
}

#[derive(Debug, Clone)]
pub struct WritingProps {
    /// Bytes written so far
    pub written: u64,
    /// Total bytes to write
    pub total: u64,
    pub speed_bps: f64,
    pub verify_after_write: bool,
    pub verifying: bool,
    pub verified: u64,
    pub verify_total: u64,
    pub verify_speed_bps: f64,
    pub write_result: Option<Result<(), String>>,
}

/// Props for the Done screen.
///
/// Shows completion status with success or error message.
#[derive(Debug, Clone)]
pub struct DoneProps {
    pub result: Option<Result<(), String>>,
    pub is_root: bool,
}

// Projection Functions: Model → Props

impl<'a> IsoSearchProps<'a> {
    pub fn from_model(model: &'a Model) -> Self {
        Self {
            query: &model.iso_query,
            searching: model.iso_searching,
            results: &model.iso_results,
            selected: model.iso_selected,
            focused: model.active_panel == ActivePanel::IsoList,
        }
    }
}

impl<'a> DeviceSelectProps<'a> {
    pub fn from_model(model: &'a Model) -> Self {
        Self {
            devices: &model.devices,
            selected: model.device_selected,
            refreshing: model.device_refreshing,
            focused: model.active_panel == ActivePanel::DeviceList,
        }
    }
}

impl<'a> ConfirmProps<'a> {
    pub fn from_model(model: &'a Model) -> Self {
        Self {
            iso_path: model.iso_chosen.as_ref(),
            device_path: model.device_chosen.as_ref(),
            confirm_input: &model.confirm_input,
            verify_after_write: model.verify_after_write,
            is_root: model.is_root,
        }
    }
}

impl WritingProps {
    pub fn from_model(model: &Model) -> Self {
        Self {
            written: model.writing_written,
            total: model.writing_total,
            speed_bps: model.writing_speed_bps,
            verify_after_write: model.verify_after_write,
            verifying: model.verifying,
            verified: model.verifying_checked,
            verify_total: model.verifying_total,
            verify_speed_bps: model.verifying_speed_bps,
            write_result: model
                .write_result
                .clone()
                .map(|r| r.map_err(|e| e.to_string())),
        }
    }
}

impl DoneProps {
    pub fn from_model(model: &Model) -> Self {
        Self {
            result: model
                .write_result
                .clone()
                .map(|r| r.map_err(|e| e.to_string())),
            is_root: model.is_root,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_iso_search_props_projection() {
        let mut model = Model::default();
        model.iso_query = "ubuntu".to_string();
        model.iso_searching = true;
        model.iso_selected = 2;
        model.active_panel = ActivePanel::IsoList;

        let props = IsoSearchProps::from_model(&model);

        assert_eq!(props.query, "ubuntu");
        assert!(props.searching);
        assert_eq!(props.selected, 2);
        assert!(props.focused);
    }

    #[test]
    fn test_device_select_props_projection() {
        let mut model = Model::default();
        model.device_selected = 1;
        model.device_refreshing = true;
        model.active_panel = ActivePanel::DeviceList;

        let props = DeviceSelectProps::from_model(&model);

        assert_eq!(props.selected, 1);
        assert!(props.refreshing);
        assert!(props.focused);
    }

    #[test]
    fn test_confirm_props_projection() {
        let mut model = Model::default();
        model.iso_chosen = Some(IsoPath::from(PathBuf::from("/path/to/nixos.iso")));
        model.device_chosen = Some(DevicePath::new(PathBuf::from("/dev/sdb")));
        model.confirm_input = "YES".to_string();
        model.verify_after_write = true;
        model.is_root = false;

        let props = ConfirmProps::from_model(&model);

        assert!(props.iso_path.is_some());
        assert_eq!(props.iso_path.unwrap().to_string(), "/path/to/nixos.iso");
        assert!(props.device_path.is_some());
        assert_eq!(props.device_path.unwrap().to_string(), "/dev/sdb");
        assert_eq!(props.confirm_input, "YES");
        assert!(props.verify_after_write);
        assert!(!props.is_root);
    }

    #[test]
    fn test_writing_props_projection() {
        let mut model = Model::default();
        model.writing_written = 1024;
        model.writing_total = 2048;
        model.writing_speed_bps = 512.0;
        model.verify_after_write = true;
        model.verifying = true;

        let props = WritingProps::from_model(&model);

        assert_eq!(props.written, 1024);
        assert_eq!(props.total, 2048);
        assert_eq!(props.speed_bps, 512.0);
        assert!(props.verify_after_write);
        assert!(props.verifying);
    }

    #[test]
    fn test_done_props_projection_success() {
        let mut model = Model::default();
        model.write_result = Some(Ok(()));
        model.is_root = true;

        let props = DoneProps::from_model(&model);

        assert!(props.result.is_some());
        assert!(props.result.as_ref().unwrap().is_ok());
        assert!(props.is_root);
    }

    #[test]
    fn test_done_props_projection_error() {
        let mut model = Model::default();
        model.write_result = Some(Err("Write failed".to_string()));
        model.is_root = false;

        let props = DoneProps::from_model(&model);

        assert!(props.result.is_some());
        assert!(props.result.as_ref().unwrap().is_err());
        assert!(!props.is_root);
    }

    #[test]
    fn test_iso_search_props_not_focused() {
        let mut model = Model::default();
        model.active_panel = ActivePanel::DeviceList;

        let props = IsoSearchProps::from_model(&model);

        assert!(!props.focused);
    }

    #[test]
    fn test_device_select_props_not_focused() {
        let mut model = Model::default();
        model.active_panel = ActivePanel::IsoList;

        let props = DeviceSelectProps::from_model(&model);

        assert!(!props.focused);
    }
}
