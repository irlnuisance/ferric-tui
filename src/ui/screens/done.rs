use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use crate::app::state::Model;
use crate::ui::{
    icons::Icons,
    theme::{colors, styles},
    widgets::CenteredModal,
};

pub fn render(model: &Model, area: Rect, buf: &mut Buffer, _icons: &Icons) {
    let modal = CenteredModal::default();
    let content_area = modal.compute_area(area);

    let mut lines = vec![];

    match &model.write_result {
        Some(Ok(())) => {
            lines.push(Line::from(Span::styled("✓ Done", styles::success())));
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::styled("✓ ", colors::SUCCESS),
                Span::styled("Write completed successfully!", styles::success()),
            ]));
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::styled("Note: ", styles::text()),
                Span::styled(
                    "You may need to reinsert the drive or run ",
                    styles::text_muted(),
                ),
                Span::styled("partprobe", styles::code()),
                Span::styled(".", styles::text_muted()),
            ]));
        }
        Some(Err(e)) => {
            lines.push(Line::from(Span::styled("✗ Failed", styles::danger())));
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::styled("✗ ", colors::DANGER),
                Span::styled("Write failed:", styles::danger()),
            ]));
            lines.push(Line::from(""));

            for line in e.lines() {
                lines.push(Line::from(vec![
                    Span::raw("  "),
                    Span::styled(line, styles::text()),
                ]));
            }

            lines.push(Line::from(""));

            if !model.is_root || contains_perm_denied(e) {
                lines.push(Line::from(vec![
                    Span::styled("💡 Hint: ", styles::warning()),
                    Span::styled(
                        "This looks like a permissions issue. Try running ",
                        styles::text_muted(),
                    ),
                    Span::styled("ferric", styles::code()),
                    Span::styled(" with ", styles::text_muted()),
                    Span::styled("sudo", styles::code()),
                    Span::styled(".", styles::text_muted()),
                ]));
            }
        }
        None => {
            lines.push(Line::from(Span::styled("● Done", styles::text_dim())));
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled("No result.", styles::text_dim())));
        }
    }

    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled("Press ", styles::text_muted()),
        Span::styled("Esc", styles::highlight()),
        Span::styled(" to go back or ", styles::text_muted()),
        Span::styled("q", styles::highlight()),
        Span::styled(" to quit.", styles::text_muted()),
    ]));

    let paragraph = Paragraph::new(lines).alignment(Alignment::Center);
    paragraph.render(content_area, buf);
}

fn contains_perm_denied(s: &str) -> bool {
    let lower = s.to_ascii_lowercase();
    lower.contains("permission denied") || lower.contains("operation not permitted")
}
