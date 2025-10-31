use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::{Paragraph, Tabs, Widget},
};

use crate::app::state::{Model, Screen};
use crate::ui::components::human_size;
use crate::ui::theme::styles;

pub struct StepHeader<'a> {
    pub steps: &'a [&'a str],
    pub selected_index: usize,
    pub status_line: Line<'a>,
}

impl<'a> Widget for StepHeader<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let titles: Vec<Line> = self
            .steps
            .iter()
            .enumerate()
            .map(|(i, s)| {
                let prefix = format!("{}.", i + 1);
                Line::from(vec![
                    Span::styled(prefix, styles::text_dim()),
                    Span::raw(" "),
                    Span::styled(*s, styles::text()),
                ])
            })
            .collect();

        let tabs = Tabs::new(titles)
            .select(self.selected_index)
            .style(styles::text())
            .highlight_style(styles::highlight())
            .divider(Span::styled(" / ", styles::text_dim()));

        let header_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Length(1)])
            .split(area);

        tabs.render(header_layout[0], buf);

        Paragraph::new(self.status_line)
            .alignment(Alignment::Center)
            .render(header_layout[1], buf);
    }
}

pub fn build_status_line(model: &Model) -> Line<'static> {
    match model.screen {
        Screen::IsoSearch => {
            if model.iso_searching {
                Line::from(vec![
                    Span::styled("● ", styles::emphasis()),
                    Span::styled("Searching for ISOs...", styles::text_muted()),
                ])
            } else if model.iso_results.is_empty() {
                Line::from(Span::styled("No ISOs found", styles::text_dim()))
            } else {
                Line::from(vec![
                    Span::styled("Results: ", styles::text_muted()),
                    Span::styled(format!("{}", model.iso_results.len()), styles::emphasis()),
                    Span::styled("  •  Enter: select", styles::text_dim()),
                ])
            }
        }
        Screen::DeviceSelect => {
            if model.device_refreshing {
                Line::from(vec![
                    Span::styled("● ", styles::emphasis()),
                    Span::styled("Refreshing devices...", styles::text_muted()),
                ])
            } else if model.devices.is_empty() {
                Line::from(Span::styled(
                    "No devices found (press 'r' to refresh)",
                    styles::text_dim(),
                ))
            } else {
                Line::from(vec![
                    Span::styled("Devices: ", styles::text_muted()),
                    Span::styled(format!("{}", model.devices.len()), styles::emphasis()),
                    Span::styled("  •  Enter: select", styles::text_dim()),
                ])
            }
        }
        Screen::Confirm => {
            let verify = if model.verify_after_write {
                "ON"
            } else {
                "OFF"
            };
            if model.confirm_input == "YES" {
                Line::from(vec![
                    Span::styled("✓ ", styles::success()),
                    Span::styled("Ready • Enter to write • Verify: ", styles::text_muted()),
                    Span::styled(verify, styles::emphasis()),
                ])
            } else {
                Line::from(vec![
                    Span::styled("Type ", styles::text_muted()),
                    Span::styled("YES", styles::warning()),
                    Span::styled(" to enable Enter • Verify: ", styles::text_muted()),
                    Span::styled(verify, styles::emphasis()),
                ])
            }
        }
        Screen::Writing => {
            let pct = if model.writing_total > 0 {
                (model.writing_written as f64 / model.writing_total as f64) * 100.0
            } else {
                0.0
            };
            Line::from(vec![
                Span::styled("Writing ", styles::text_muted()),
                Span::styled(format!("{:>5.1}%", pct), styles::highlight()),
                Span::styled("  •  Speed ", styles::text_muted()),
                Span::styled(
                    human_size(model.writing_speed_bps as u64) + "/s",
                    styles::code(),
                ),
            ])
        }
        Screen::Done => {
            if let Some(Ok(())) = model.verify_result.as_ref().or(model.write_result.as_ref()) {
                Line::from(vec![
                    Span::styled("✓ ", styles::success()),
                    Span::styled("Completed successfully", styles::success()),
                ])
            } else if let Some(Err(_)) =
                model.verify_result.as_ref().or(model.write_result.as_ref())
            {
                Line::from(vec![
                    Span::styled("✗ ", styles::danger()),
                    Span::styled("Completed with errors", styles::danger()),
                ])
            } else {
                Line::from(Span::styled("Done", styles::text_dim()))
            }
        }
    }
}

pub fn screen_to_step_index(screen: &Screen) -> usize {
    match screen {
        Screen::IsoSearch => 0,
        Screen::DeviceSelect => 1,
        Screen::Confirm => 2,
        Screen::Writing => 3,
        Screen::Done => 4,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screen_to_step_index() {
        assert_eq!(screen_to_step_index(&Screen::IsoSearch), 0);
        assert_eq!(screen_to_step_index(&Screen::DeviceSelect), 1);
        assert_eq!(screen_to_step_index(&Screen::Confirm), 2);
        assert_eq!(screen_to_step_index(&Screen::Writing), 3);
        assert_eq!(screen_to_step_index(&Screen::Done), 4);
    }
}
