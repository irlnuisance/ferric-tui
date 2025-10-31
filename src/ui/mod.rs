mod components;
pub(crate) mod icons;
pub(crate) mod layout;
mod screens;
pub(crate) mod theme;
pub(crate) mod tokens;
mod widgets;

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use crate::app::state::{ActivePanel, Model, Screen};
use icons::Icons;
use theme::styles;
use tokens::UiCapabilities;
use widgets::{AppShell, app_shell::compute_border_color};

impl Widget for &Model {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let capabilities = UiCapabilities::detect();
        let icons = Icons::from_capabilities(&capabilities);

        let shell = AppShell {
            title: " ferric ",
            border_color: compute_border_color(self),
            capabilities: &capabilities,
        };

        shell.render(self, &icons, area, buf);
    }
}

pub(crate) fn render_iso_search(model: &Model, area: Rect, buf: &mut Buffer, icons: &Icons) {
    screens::iso_search::render(model, area, buf, icons);
}

pub(crate) fn render_device_select(model: &Model, area: Rect, buf: &mut Buffer, icons: &Icons) {
    screens::device_select::render(model, area, buf, icons);
}

pub(crate) fn render_confirm(model: &Model, area: Rect, buf: &mut Buffer, icons: &Icons) {
    screens::confirm::render(model, area, buf, icons);
}

pub(crate) fn render_writing(model: &Model, area: Rect, buf: &mut Buffer, icons: &Icons) {
    screens::writing::render(model, area, buf, icons);
}

pub(crate) fn render_done(model: &Model, area: Rect, buf: &mut Buffer, icons: &Icons) {
    screens::done::render(model, area, buf, icons);
}

pub(crate) fn render_footer(model: &Model, area: Rect, buf: &mut Buffer) {
    let hint = nav_cycle_hint(model);
    let paragraph = Paragraph::new(Line::from(Span::styled(hint, styles::text_dim())))
        .alignment(Alignment::Center);
    paragraph.render(area, buf);
}

fn nav_cycle_hint(m: &Model) -> String {
    match m.screen {
        Screen::IsoSearch => {
            let focus_label = match m.active_panel {
                ActivePanel::IsoList => "ISO Search",
                ActivePanel::DeviceList => "Device Select",
                ActivePanel::ConfirmInput => "Confirm",
            };
            format!(
                "Focus: {} | Tab/Shift-Tab: Switch Panels | q: Quit",
                focus_label
            )
        }
        Screen::DeviceSelect => {
            let focus_label = match m.active_panel {
                ActivePanel::IsoList => "ISO Search",
                ActivePanel::DeviceList => "Device Select",
                ActivePanel::ConfirmInput => "Confirm",
            };
            format!(
                "Focus: {} | Tab/Shift-Tab: Switch Panels | r: Refresh | q: Quit",
                focus_label
            )
        }
        Screen::Confirm => {
            format!("Focus: Confirm | Type YES then Enter | Esc: Back | q: Quit")
        }
        Screen::Writing => "Writing in progress... Please wait | Do NOT remove device".to_string(),
        Screen::Done => "Complete | Esc: Back | q: Quit".to_string(),
    }
}
