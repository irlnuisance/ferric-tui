use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Widget},
};

use crate::app::state::{Model, Screen};
use crate::ui::core::UiRouter;
use crate::ui::layout::split_header_body_footer;
use crate::ui::theme::colors;

use super::step_header::{StepHeader, build_status_line, screen_to_step_index};

pub struct AppShell<'a> {
    pub title: &'a str,
    pub border_color: Color,
}

impl<'a> AppShell<'a> {
    pub fn render(self, model: &Model, router: &UiRouter, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title(self.title)
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(self.border_color));

        let inner = block.inner(area);
        block.render(area, buf);

        for y in inner.top()..inner.bottom() {
            for x in inner.left()..inner.right() {
                buf[(x, y)].set_bg(colors::BG_PRIMARY);
            }
        }

        let [header_area, body_area, footer_area] = split_header_body_footer(inner);

        let steps = ["ISO", "Device", "Confirm", "Write", "Done"];
        let selected_index = screen_to_step_index(&model.screen);
        let status_line = build_status_line(model);

        let header = StepHeader {
            steps: &steps,
            selected_index,
            status_line,
        };
        header.render(header_area, buf);

        // Use Router to render the screen content
        router.render_screen(model, body_area, buf);

        render_footer(model, footer_area, buf);
    }
}

pub fn compute_border_color(model: &Model) -> Color {
    match model.screen {
        Screen::IsoSearch | Screen::DeviceSelect => colors::BORDER_ACTIVE,
        Screen::Confirm => colors::WARNING,
        Screen::Writing => colors::PRIMARY,
        Screen::Done => {
            if matches!(model.write_result, Some(Ok(_))) {
                colors::SUCCESS
            } else if matches!(model.write_result, Some(Err(_))) {
                colors::DANGER
            } else {
                colors::BORDER_INACTIVE
            }
        }
    }
}

fn render_footer(model: &Model, area: Rect, buf: &mut Buffer) {
    super::super::render_footer(model, area, buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_border_color_iso_search() {
        let mut model = Model::default();
        model.screen = Screen::IsoSearch;
        assert_eq!(compute_border_color(&model), colors::BORDER_ACTIVE);
    }

    #[test]
    fn test_compute_border_color_confirm() {
        let mut model = Model::default();
        model.screen = Screen::Confirm;
        assert_eq!(compute_border_color(&model), colors::WARNING);
    }

    #[test]
    fn test_compute_border_color_writing() {
        let mut model = Model::default();
        model.screen = Screen::Writing;
        assert_eq!(compute_border_color(&model), colors::PRIMARY);
    }

    #[test]
    fn test_compute_border_color_done_success() {
        let mut model = Model::default();
        model.screen = Screen::Done;
        model.write_result = Some(Ok(()));
        assert_eq!(compute_border_color(&model), colors::SUCCESS);
    }

    #[test]
    fn test_compute_border_color_done_error() {
        let mut model = Model::default();
        model.screen = Screen::Done;
        model.write_result = Some(Err("error".to_string()));
        assert_eq!(compute_border_color(&model), colors::DANGER);
    }
}
