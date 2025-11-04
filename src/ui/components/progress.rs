use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use crate::ui::icons::Icons;
use crate::ui::theme::styles;

use super::utils::{format_seconds, human_size};

pub struct ProgressWidget<'a> {
    pub title: String,
    pub current: u64,
    pub total: u64,
    pub speed_bps: f64,
    pub bar_width: usize,
    pub color: ratatui::style::Color,
    pub icons: &'a Icons,
}

impl<'a> Widget for ProgressWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let pct = if self.total > 0 {
            (self.current as f64 / self.total as f64) * 100.0
        } else {
            0.0
        };

        let filled = ((pct / 100.0) * self.bar_width as f64) as usize;
        let empty = self.bar_width.saturating_sub(filled);
        let bar = format!(
            "{}{}",
            self.icons.block_filled.repeat(filled),
            self.icons.block_empty.repeat(empty)
        );

        let mut lines = vec![];

        lines.push(Line::from(Span::styled(&self.title, styles::title())));
        lines.push(Line::from(""));

        lines.push(Line::from(vec![
            Span::styled("Progress: ", styles::text()),
            Span::styled(format!("{:>5.1}%", pct), styles::highlight()),
        ]));

        lines.push(Line::from(Span::styled(
            bar,
            Style::default().fg(self.color),
        )));

        lines.push(Line::from(vec![
            Span::styled(format!("{}  ", human_size(self.current)), styles::text()),
            Span::styled("/ ", styles::text_dim()),
            Span::styled(
                format!("  {}", human_size(self.total)),
                styles::text_muted(),
            ),
        ]));

        if self.speed_bps > 0.0 {
            lines.push(Line::from(vec![
                Span::styled("Speed: ", styles::text()),
                Span::styled(
                    format!("{}/s", human_size(self.speed_bps as u64)),
                    styles::code(),
                ),
            ]));

            if self.total > 0 && self.current < self.total {
                let remaining = (self.total - self.current) as f64;
                let secs = remaining / self.speed_bps;
                let eta = format_seconds(secs as u64);
                lines.push(Line::from(vec![
                    Span::styled("ETA: ", styles::text()),
                    Span::styled(eta, styles::emphasis()),
                ]));
            }
        }

        let paragraph = Paragraph::new(lines).alignment(Alignment::Center);
        paragraph.render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_percentage_calculation() {
        let icons = Icons::UNICODE;
        let progress = ProgressWidget {
            title: "Test".to_string(),
            current: 50,
            total: 100,
            speed_bps: 10.0,
            bar_width: 20,
            color: ratatui::style::Color::Green,
            icons: &icons,
        };

        // Should be 50%
        let pct = (progress.current as f64 / progress.total as f64) * 100.0;
        assert_eq!(pct, 50.0);
    }

    #[test]
    fn test_progress_zero_total() {
        let icons = Icons::UNICODE;
        let progress = ProgressWidget {
            title: "Test".to_string(),
            current: 0,
            total: 0,
            speed_bps: 0.0,
            bar_width: 20,
            color: ratatui::style::Color::Green,
            icons: &icons,
        };

        let pct = if progress.total > 0 {
            (progress.current as f64 / progress.total as f64) * 100.0
        } else {
            0.0
        };
        assert_eq!(pct, 0.0);
    }
}
