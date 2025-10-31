use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use crate::app::state::Model;
use crate::ui::{
    components::ProgressWidget,
    icons::Icons,
    theme::{colors, styles},
};

pub fn render(model: &Model, area: Rect, buf: &mut Buffer, icons: &Icons) {
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
        current: model.writing_written,
        total: model.writing_total,
        speed_bps: model.writing_speed_bps,
        bar_width: 40,
        color: colors::SUCCESS,
        icons,
    };
    write_progress.render(layout[0], buf);

    let mut extra_lines = vec![];
    extra_lines.push(Line::from(vec![
        Span::styled("⚠ ", colors::WARNING),
        Span::styled("Please wait ", styles::warning()),
        Span::styled("- do not remove the device.", styles::text()),
    ]));

    if model.verify_after_write {
        extra_lines.push(Line::from(""));
        if model.verifying {
            let verify_progress = ProgressWidget {
                title: "🔍 Verifying".to_string(),
                current: model.verifying_checked,
                total: model.verifying_total,
                speed_bps: model.verifying_speed_bps,
                bar_width: 40,
                color: colors::ACCENT,
                icons,
            };

            let verify_area = Rect {
                x: layout[2].x,
                y: layout[2].y,
                width: layout[2].width,
                height: 10,
            };
            verify_progress.render(verify_area, buf);
        } else if model.write_result == Some(Ok(())) {
            extra_lines.push(Line::from(vec![
                Span::styled("● ", colors::PRIMARY),
                Span::styled("Verification pending...", styles::text_muted()),
            ]));
        }
    }

    let remaining_area = if model.verify_after_write && model.verifying {
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
