use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph, Widget},
};

use super::{colors, styles};


pub struct ListPanel<'a> {
    pub title: &'a str,
    pub items: Vec<ListItem<'a>>,
    pub selected: usize,
    pub max_display: usize,
    pub focused: bool,
}

pub struct ListItem<'a> {
    pub primary: String,
    pub secondary: Option<String>,
    pub badges: Vec<Badge>,
    pub marker: Option<&'a str>,
}

pub struct Badge {
    pub text: String,
    pub style: Style,
}

impl<'a> Widget for ListPanel<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (border_color, border_type) = if self.focused {
            (colors::BORDER_FOCUS, BorderType::Double)
        } else {
            (colors::BORDER_INACTIVE, BorderType::Rounded)
        };

        let block = Block::bordered()
            .title(self.title)
            .border_type(border_type)
            .border_style(Style::default().fg(border_color));

        let inner = block.inner(area);
        block.render(area, buf);

        let mut lines = vec![];

        for (i, item) in self.items.iter().take(self.max_display).enumerate() {
            let is_selected = i == self.selected;
            let mut spans = vec![];

            if is_selected {
                let marker = item.marker.unwrap_or("▶");
                spans.push(Span::styled(format!("{} ", marker), styles::selected_marker()));

                spans.push(Span::styled(&item.primary, styles::selected()));

                if let Some(ref secondary) = item.secondary {
                    spans.push(Span::raw(" "));
                    spans.push(Span::styled(
                        secondary,
                        Style::default()
                            .fg(colors::TEXT_SECONDARY)
                            .bg(colors::SELECTED_BG),
                    ));
                }

                for badge in &item.badges {
                    spans.push(Span::raw(" "));
                    spans.push(Span::styled(
                        &badge.text,
                        badge.style.bg(colors::SELECTED_BG),
                    ));
                }
            } else {
                spans.push(Span::raw("  "));
                spans.push(Span::styled(&item.primary, styles::text()));

                if let Some(ref secondary) = item.secondary {
                    spans.push(Span::raw(" "));
                    spans.push(Span::styled(secondary, styles::text_dim()));
                }

                for badge in &item.badges {
                    spans.push(Span::raw(" "));
                    spans.push(Span::styled(&badge.text, badge.style));
                }
            }

            lines.push(Line::from(spans));
        }

        if self.items.len() > self.max_display {
            lines.push(Line::from(Span::styled(
                format!("... and {} more", self.items.len() - self.max_display),
                styles::text_dim(),
            )));
        }

        let paragraph = Paragraph::new(lines).alignment(Alignment::Left);
        paragraph.render(inner, buf);
    }
}


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


pub struct ProgressWidget {
    pub title: String,
    pub current: u64,
    pub total: u64,
    pub speed_bps: f64,
    pub bar_width: usize,
    pub color: ratatui::style::Color,
}

impl Widget for ProgressWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let pct = if self.total > 0 {
            (self.current as f64 / self.total as f64) * 100.0
        } else {
            0.0
        };

        let filled = ((pct / 100.0) * self.bar_width as f64) as usize;
        let empty = self.bar_width.saturating_sub(filled);
        let bar = format!("{}{}", "█".repeat(filled), "░".repeat(empty));

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
            Span::styled(
                format!("{}  ", human_size(self.current)),
                styles::text(),
            ),
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
