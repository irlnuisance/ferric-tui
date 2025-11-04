use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use crate::ui::{
    components::ProgressWidget,
    core::{UiCtx, View, WritingProps},
    theme::{colors, styles},
};

pub struct WritingScreen;

impl View for WritingScreen {
    type Props<'a> = WritingProps;

    fn render<'a>(&self, props: &Self::Props<'a>, ctx: &UiCtx, area: Rect, buf: &mut Buffer) {
        let centered_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(60),
                Constraint::Percentage(20),
            ])
            .split(area);

        let vertical_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Min(1),
                Constraint::Percentage(20),
            ])
            .split(centered_layout[1]);

        let content_area = vertical_layout[1];

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(10),
                Constraint::Length(1),
                Constraint::Min(1),
            ])
            .split(content_area);

        let write_progress = ProgressWidget {
            title: "⚡ Writing".to_string(),
            current: props.written,
            total: props.total,
            speed_bps: props.speed_bps,
            bar_width: 40,
            color: colors::SUCCESS,
            icons: &ctx.icons,
        };
        write_progress.render(layout[0], buf);

        let mut extra_lines = vec![];
        extra_lines.push(Line::from(vec![
            Span::styled("⚠ ", colors::WARNING),
            Span::styled("Please wait ", styles::warning()),
            Span::styled("- do not remove the device.", styles::text()),
        ]));

        if props.verify_after_write {
            extra_lines.push(Line::from(""));
            if props.verifying {
                let verify_progress = ProgressWidget {
                    title: "🔍 Verifying".to_string(),
                    current: props.verified,
                    total: props.verify_total,
                    speed_bps: props.verify_speed_bps,
                    bar_width: 40,
                    color: colors::ACCENT,
                    icons: &ctx.icons,
                };

                let verify_area = Rect {
                    x: layout[2].x,
                    y: layout[2].y,
                    width: layout[2].width,
                    height: 10,
                };
                verify_progress.render(verify_area, buf);
            } else if props.write_result == Some(Ok(())) {
                extra_lines.push(Line::from(vec![
                    Span::styled("● ", colors::PRIMARY),
                    Span::styled("Verification pending...", styles::text_muted()),
                ]));
            }
        }

        let remaining_area = if props.verify_after_write && props.verifying {
            Rect {
                x: layout[2].x,
                y: layout[2].y + 10,
                width: layout[2].width,
                height: layout[2].height.saturating_sub(10),
            }
        } else {
            layout[2]
        };

        Paragraph::new(extra_lines)
            .alignment(Alignment::Center)
            .render(remaining_area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_writing_screen_without_verify() {
        let screen = WritingScreen;
        let props = WritingProps {
            written: 1024,
            total: 2048,
            speed_bps: 512.0,
            verify_after_write: false,
            verifying: false,
            verified: 0,
            verify_total: 0,
            verify_speed_bps: 0.0,
            write_result: None,
        };
        let ctx = UiCtx::new();
        let area = Rect::new(0, 0, 120, 40);
        let mut buf = Buffer::empty(area);

        screen.render(&props, &ctx, area, &mut buf);
    }

    #[test]
    fn test_writing_screen_with_verify() {
        let screen = WritingScreen;
        let props = WritingProps {
            written: 2048,
            total: 2048,
            speed_bps: 512.0,
            verify_after_write: true,
            verifying: true,
            verified: 1024,
            verify_total: 2048,
            verify_speed_bps: 256.0,
            write_result: Some(Ok(())),
        };
        let ctx = UiCtx::new();
        let area = Rect::new(0, 0, 120, 40);
        let mut buf = Buffer::empty(area);

        screen.render(&props, &ctx, area, &mut buf);
    }
}
