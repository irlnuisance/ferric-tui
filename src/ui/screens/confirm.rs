use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Style,
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
    match &model.iso_chosen {
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
    match &model.device_chosen {
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

    let verify_status = if model.verify_after_write {
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

    if !model.is_root {
        lines.push(Line::from(vec![
            Span::styled("Press ", styles::text_muted()),
            Span::styled("Ctrl-S", styles::highlight()),
            Span::styled(" to rerun with sudo", styles::text_muted()),
        ]));
    }

    lines.push(Line::from(""));

    let confirm_style = if model.confirm_input == "YES" {
        styles::success()
    } else if !model.confirm_input.is_empty() {
        styles::warning()
    } else {
        styles::text_muted()
    };

    let border_style = if model.confirm_input == "YES" {
        styles::success()
    } else if !model.confirm_input.is_empty() {
        styles::warning()
    } else {
        styles::text_muted()
    };

    let input_display = if model.confirm_input.is_empty() {
        "".to_string()
    } else {
        model.confirm_input.clone()
    };

    let input_with_cursor = format!("{}_", input_display);
    let padding = " ".repeat(4_usize.saturating_sub(input_with_cursor.len()));

    lines.push(Line::from(vec![Span::styled("┌──────┐", border_style)]));
    lines.push(Line::from(vec![
        Span::styled("│ ", border_style),
        Span::styled(&input_with_cursor, confirm_style),
        Span::styled(&padding, Style::default()),
        Span::styled(" │", border_style),
    ]));
    lines.push(Line::from(vec![Span::styled("└──────┘", border_style)]));

    lines.push(Line::from(""));
    if !model.is_root {
        lines.push(Line::from(vec![
            Span::styled("⚠ ", colors::WARNING),
            Span::styled("Not running as root. ", styles::warning()),
            Span::styled("Writing typically requires ", styles::text_muted()),
            Span::styled("sudo", styles::code()),
            Span::styled(".", styles::text_muted()),
        ]));
    }

    if model.iso_chosen.is_none() || model.device_chosen.is_none() {
        lines.push(Line::from(vec![
            Span::styled("⚠ ", colors::WARNING),
            Span::styled(
                "Please choose an ISO and a device first.",
                styles::warning(),
            ),
        ]));
    }

    if model.confirm_input == "YES" {
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
}
