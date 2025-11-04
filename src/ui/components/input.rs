use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph, Widget},
};

use crate::ui::theme::{colors, styles};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputState {
    Normal,
    Focused,
    Valid,
    Invalid,
}

/// Reusable input field component
pub struct Input<'a> {
    pub value: &'a str,
    pub placeholder: &'a str,
    pub state: InputState,
    pub show_cursor: bool,
    pub width: u16,
}

impl<'a> Input<'a> {
    pub fn new(value: &'a str) -> Self {
        Self {
            value,
            placeholder: "",
            state: InputState::Normal,
            show_cursor: true,
            width: 10,
        }
    }

    pub fn placeholder(mut self, placeholder: &'a str) -> Self {
        self.placeholder = placeholder;
        self
    }

    pub fn state(mut self, state: InputState) -> Self {
        self.state = state;
        self
    }

    pub fn show_cursor(mut self, show: bool) -> Self {
        self.show_cursor = show;
        self
    }

    pub fn width(mut self, width: u16) -> Self {
        self.width = width;
        self
    }
}

impl<'a> Widget for Input<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (border_color, border_type, text_color) = match self.state {
            InputState::Normal => (
                colors::BORDER_INACTIVE,
                BorderType::Rounded,
                colors::TEXT_PRIMARY,
            ),
            InputState::Focused => (
                colors::BORDER_FOCUS,
                BorderType::Double,
                colors::TEXT_PRIMARY,
            ),
            InputState::Valid => (colors::SUCCESS, BorderType::Rounded, colors::SUCCESS),
            InputState::Invalid => (colors::DANGER, BorderType::Rounded, colors::DANGER_BRIGHT),
        };

        let block = Block::bordered()
            .border_type(border_type)
            .border_style(Style::default().fg(border_color));

        let inner = block.inner(area);
        block.render(area, buf);

        let display_text = if self.value.is_empty() {
            if self.placeholder.is_empty() {
                String::from("")
            } else {
                self.placeholder.to_string()
            }
        } else if self.show_cursor {
            format!("{}_", self.value)
        } else {
            self.value.to_string()
        };

        let text_style = if self.value.is_empty() && !self.placeholder.is_empty() {
            styles::text_dim()
        } else {
            Style::default().fg(text_color)
        };

        let padding_needed = (self.width as usize).saturating_sub(display_text.len());
        let left_pad = padding_needed / 2;
        let right_pad = padding_needed - left_pad;

        let line = Line::from(vec![
            Span::styled(" ".repeat(left_pad), Style::default()),
            Span::styled(display_text, text_style),
            Span::styled(" ".repeat(right_pad), Style::default()),
        ]);

        let paragraph = Paragraph::new(line).alignment(Alignment::Center);
        paragraph.render(inner, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::buffer::Buffer;
    use ratatui::layout::Rect;

    #[test]
    fn test_input_new() {
        let input = Input::new("test");
        assert_eq!(input.value, "test");
        assert_eq!(input.placeholder, "");
        assert_eq!(input.state, InputState::Normal);
        assert!(input.show_cursor);
        assert_eq!(input.width, 10);
    }

    #[test]
    fn test_input_builder_pattern() {
        let input = Input::new("value")
            .placeholder("hint")
            .state(InputState::Focused)
            .show_cursor(false)
            .width(20);

        assert_eq!(input.value, "value");
        assert_eq!(input.placeholder, "hint");
        assert_eq!(input.state, InputState::Focused);
        assert!(!input.show_cursor);
        assert_eq!(input.width, 20);
    }

    #[test]
    fn test_input_renders_without_panic() {
        let input = Input::new("YES").state(InputState::Valid);
        let area = Rect::new(0, 0, 15, 3);
        let mut buf = Buffer::empty(area);

        input.render(area, &mut buf);
        // If we make it here without panic, rendering succeeded
    }

    #[test]
    fn test_input_state_variants() {
        assert_ne!(InputState::Normal, InputState::Focused);
        assert_ne!(InputState::Valid, InputState::Invalid);
    }
}
