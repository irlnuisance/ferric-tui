use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph, Widget},
};

use crate::ui::theme::{colors, styles};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detail_item_creation() {
        let item = DetailItem {
            label: "Test",
            value: "Value".to_string(),
            style: Style::default(),
        };
        assert_eq!(item.label, "Test");
        assert_eq!(item.value, "Value");
    }

    #[test]
    fn test_detail_panel_focus_states() {
        let panel_focused = DetailPanel {
            title: "Test",
            items: vec![],
            synced_focus: true,
        };
        let panel_unfocused = DetailPanel {
            title: "Test",
            items: vec![],
            synced_focus: false,
        };

        assert!(panel_focused.synced_focus);
        assert!(!panel_unfocused.synced_focus);
    }
}
