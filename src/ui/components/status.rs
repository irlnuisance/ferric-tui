use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Style,
    widgets::{Paragraph, Widget},
};

/// Status badge component for displaying styled status text
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_badge_creation() {
        let badge = StatusBadge {
            text: "Ready",
            style: Style::default(),
        };
        assert_eq!(badge.text, "Ready");
    }
}
