pub mod components;
mod core;
pub(crate) mod icons;
pub(crate) mod layout;
mod screens;
pub(crate) mod theme;
pub(crate) mod tokens;
mod widgets;

pub use core::{UiCtx, UiRouter, View};

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use crate::app::state::{ActivePanel, Model, Screen};
use theme::styles;

impl Widget for &Model {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Route using View + Props projection
        UiRouter::default().render(self, area, buf);
    }
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
