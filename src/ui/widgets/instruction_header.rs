use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    text::Line,
    widgets::{Paragraph, Widget},
};

pub struct InstructionHeader<'a> {
    pub lines: Vec<Line<'a>>,
}

impl<'a> Widget for InstructionHeader<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(self.lines)
            .alignment(Alignment::Left)
            .render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::text::Span;

    #[test]
    fn test_instruction_header_render() {
        let header = InstructionHeader {
            lines: vec![
                Line::from("Title"),
                Line::from(vec![Span::raw("Key: "), Span::raw("Value")]),
            ],
        };

        let mut buf = Buffer::empty(Rect::new(0, 0, 20, 5));
        header.render(Rect::new(0, 0, 20, 5), &mut buf);

        assert_eq!(buf[(0, 0)].symbol(), "T");
        assert_eq!(buf[(0, 1)].symbol(), "K");
    }
}
