use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph, Widget},
};

use super::icons::Icons;
use super::theme::{colors, styles};

pub struct DetailPanel<'a> {
    pub title: &'a str,
    pub items: Vec<DetailItem<'a>>,
    pub synced_focus: bool,
}

pub struct DetailItem<'a> {
    pub label: &'a str,
    pub value: String,
    pub style: Style,
}

impl<'a> Widget for DetailPanel<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let border_color = if self.synced_focus {
            colors::BORDER_ACTIVE
        } else {
            colors::BORDER_INACTIVE
        };

        let block = Block::bordered()
            .title(self.title)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(border_color));

        let inner = block.inner(area);
        block.render(area, buf);

        let mut lines = vec![];

        for item in self.items {
            lines.push(Line::from(vec![
                Span::styled(format!("{}: ", item.label), styles::text_muted()),
                Span::styled(item.value, item.style),
            ]));
        }

        let paragraph = Paragraph::new(lines).alignment(Alignment::Left);
        paragraph.render(inner, buf);
    }
}

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

#[allow(dead_code)]
pub struct StatusBadge<'a> {
    pub text: &'a str,
    pub style: Style,
}

impl<'a> Widget for StatusBadge<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let text = format!(" {} ", self.text);
        let paragraph = Paragraph::new(text)
            .style(self.style)
            .alignment(Alignment::Center);
        paragraph.render(area, buf);
    }
}

pub fn human_size(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;
    let b = bytes as f64;
    if b >= GB {
        format!("{:.1} GiB", b / GB)
    } else if b >= MB {
        format!("{:.1} MiB", b / MB)
    } else if b >= KB {
        format!("{:.0} KiB", b / KB)
    } else {
        format!("{} B", bytes)
    }
}

pub fn format_seconds(secs: u64) -> String {
    if secs < 60 {
        format!("{}s", secs)
    } else if secs < 3600 {
        format!("{}m {}s", secs / 60, secs % 60)
    } else {
        format!("{}h {}m", secs / 3600, (secs % 3600) / 60)
    }
}
