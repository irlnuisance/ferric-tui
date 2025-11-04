use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use crate::ui::{
    components::{Input, InputState},
    core::{ConfirmProps, UiCtx, View},
    theme::{colors, styles},
    widgets::CenteredModal,
};

pub struct ConfirmScreen;

impl View for ConfirmScreen {
    type Props<'a> = ConfirmProps<'a>;

    fn render<'a>(&self, props: &Self::Props<'a>, _ctx: &UiCtx, area: Rect, buf: &mut Buffer) {
        let modal = CenteredModal {
            horizontal_margin_pct: 20,
            vertical_margin_pct: 15,
        };
        let content_area = modal.compute_area(area);

        let mut lines = vec![];

        lines.push(Line::from(Span::styled(
            "⚠  Confirm Action",
            styles::warning(),
        )));
        lines.push(Line::from(""));

        lines.push(Line::from(vec![
            Span::styled("⚠ WARNING: ", styles::danger()),
            Span::styled("This will ", styles::text()),
            Span::styled("ERASE ALL DATA", styles::danger()),
            Span::styled(" on the target drive!", styles::text()),
        ]));
        lines.push(Line::from(""));

        lines.push(Line::from(vec![Span::styled(
            "Selected ISO: ",
            styles::text(),
        )]));
        match &props.iso_path {
            Some(p) => {
                lines.push(Line::from(vec![
                    Span::raw("  "),
                    Span::styled(format!("{}", p), styles::code()),
                ]));
            }
            None => {
                lines.push(Line::from(vec![
                    Span::raw("  "),
                    Span::styled("<none>", styles::text_dim()),
                ]));
            }
        }

        lines.push(Line::from(vec![Span::styled(
            "Selected Device: ",
            styles::text(),
        )]));
        match &props.device_path {
            Some(p) => {
                lines.push(Line::from(vec![
                    Span::raw("  "),
                    Span::styled(format!("{}", p), styles::danger()),
                ]));
            }
            None => {
                lines.push(Line::from(vec![
                    Span::raw("  "),
                    Span::styled("<none>", styles::text_dim()),
                ]));
            }
        }
        lines.push(Line::from(""));

        let verify_status = if props.verify_after_write {
            Span::styled("ON", styles::success())
        } else {
            Span::styled("OFF", styles::text_dim())
        };
        lines.push(Line::from(vec![
            Span::styled("Verify after write: ", styles::text()),
            verify_status,
            Span::styled("  (press ", styles::text_muted()),
            Span::styled("'v'", styles::highlight()),
            Span::styled(" to toggle)", styles::text_muted()),
        ]));
        lines.push(Line::from(""));

        lines.push(Line::from(vec![
            Span::styled("Type ", styles::text_muted()),
            Span::styled("YES", styles::warning()),
            Span::styled(" then press ", styles::text_muted()),
            Span::styled("Enter", styles::highlight()),
            Span::styled(" to confirm", styles::text_muted()),
        ]));

        if !props.is_root {
            lines.push(Line::from(vec![
                Span::styled("Press ", styles::text_muted()),
                Span::styled("Ctrl-S", styles::highlight()),
                Span::styled(" to rerun with sudo", styles::text_muted()),
            ]));
        }

        lines.push(Line::from(""));
        lines.push(Line::from(""));
        if !props.is_root {
            lines.push(Line::from(vec![
                Span::styled("⚠ ", colors::WARNING),
                Span::styled("Not running as root. ", styles::warning()),
                Span::styled("Writing typically requires ", styles::text_muted()),
                Span::styled("sudo", styles::code()),
                Span::styled(".", styles::text_muted()),
            ]));
        }

        if props.iso_path.is_none() || props.device_path.is_none() {
            lines.push(Line::from(vec![
                Span::styled("⚠ ", colors::WARNING),
                Span::styled(
                    "Please choose an ISO and a device first.",
                    styles::warning(),
                ),
            ]));
        }

        if props.confirm_input == "YES" {
            lines.push(Line::from(vec![
                Span::styled("✓ ", colors::SUCCESS),
                Span::styled("Enter", styles::highlight()),
                Span::styled(" will proceed.", styles::success()),
            ]));
        } else {
            lines.push(Line::from(vec![
                Span::styled("● ", colors::TEXT_DIM),
                Span::styled("Enter is disabled until you type YES.", styles::text_dim()),
            ]));
        }

        let paragraph = Paragraph::new(lines).alignment(Alignment::Center);
        paragraph.render(content_area, buf);

        // Render Input component centered below the text
        let input_state = if props.confirm_input == "YES" {
            InputState::Valid
        } else if !props.confirm_input.is_empty() {
            InputState::Invalid
        } else {
            InputState::Focused
        };

        let input = Input::new(props.confirm_input)
            .state(input_state)
            .show_cursor(true)
            .width(8);

        let input_y = content_area.y + (content_area.height / 2).saturating_sub(2);
        let input_width = 12_u16;
        let input_x = content_area.x + (content_area.width / 2).saturating_sub(input_width / 2);
        let input_area = Rect::new(input_x, input_y, input_width, 3);

        input.render(input_area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_confirm_screen_view_trait() {
        let screen = ConfirmScreen;
        let props = ConfirmProps {
            iso_path: None,
            device_path: None,
            confirm_input: "",
            verify_after_write: false,
            is_root: true,
        };
        let ctx = UiCtx::new();
        let area = Rect::new(0, 0, 80, 24);
        let mut buf = Buffer::empty(area);

        // Should not panic
        screen.render(&props, &ctx, area, &mut buf);
    }
}
