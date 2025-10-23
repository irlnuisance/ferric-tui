mod components;

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph, Widget},
};

use crate::app::state::{ActivePanel, Model, Screen};
use components::*;

#[allow(dead_code)]
pub(crate) mod colors {
    use ratatui::style::Color;

    // Primary brand colors
    pub const PRIMARY: Color = Color::Rgb(103, 177, 255);      // Bright blue
    pub const PRIMARY_DIM: Color = Color::Rgb(60, 120, 190);   // Dimmed blue
    pub const SECONDARY: Color = Color::Rgb(255, 184, 108);    // Warm orange
    pub const ACCENT: Color = Color::Rgb(171, 140, 255);       // Purple

    // Semantic status colors
    pub const SUCCESS: Color = Color::Rgb(102, 255, 153);      // Bright green
    pub const SUCCESS_DIM: Color = Color::Rgb(50, 180, 100);   // Forest green
    pub const WARNING: Color = Color::Rgb(255, 220, 100);      // Amber
    pub const DANGER: Color = Color::Rgb(255, 107, 129);       // Soft red
    pub const DANGER_BRIGHT: Color = Color::Rgb(255, 85, 110); // Vibrant red

    // UI element colors
    pub const TEXT_PRIMARY: Color = Color::Rgb(230, 237, 243); // Off-white
    pub const TEXT_SECONDARY: Color = Color::Rgb(150, 160, 175); // Muted gray
    pub const TEXT_DIM: Color = Color::Rgb(100, 110, 125);     // Dimmer gray

    // Interactive states
    pub const SELECTED_BG: Color = Color::Rgb(45, 75, 110);    // Deep blue bg
    pub const SELECTED_FG: Color = Color::Rgb(255, 255, 255);  // White text
    pub const HOVER_BG: Color = Color::Rgb(35, 50, 70);        // Darker blue

    // Background layers
    pub const BG_PRIMARY: Color = Color::Rgb(16, 20, 28);      // Dark navy
    pub const BG_SECONDARY: Color = Color::Rgb(25, 30, 40);    // Lighter navy
    pub const BG_TERTIARY: Color = Color::Rgb(35, 42, 54);     // Panel bg

    // Borders & dividers
    pub const BORDER_ACTIVE: Color = PRIMARY;
    pub const BORDER_INACTIVE: Color = Color::Rgb(60, 70, 85);
    pub const BORDER_FOCUS: Color = ACCENT;
}

#[allow(dead_code)]
pub(crate) mod styles {
    use ratatui::style::{Modifier, Style};
    use super::colors;

    pub fn title() -> Style {
        Style::default()
            .fg(colors::PRIMARY)
            .add_modifier(Modifier::BOLD)
    }

    pub fn subtitle() -> Style {
        Style::default()
            .fg(colors::TEXT_SECONDARY)
            .add_modifier(Modifier::DIM)
    }

    pub fn text() -> Style {
        Style::default().fg(colors::TEXT_PRIMARY)
    }

    pub fn text_muted() -> Style {
        Style::default().fg(colors::TEXT_SECONDARY)
    }

    pub fn text_dim() -> Style {
        Style::default()
            .fg(colors::TEXT_DIM)
            .add_modifier(Modifier::DIM)
    }

    pub fn selected() -> Style {
        Style::default()
            .fg(colors::SELECTED_FG)
            .bg(colors::SELECTED_BG)
            .add_modifier(Modifier::BOLD)
    }

    pub fn selected_marker() -> Style {
        Style::default()
            .fg(colors::ACCENT)
            .add_modifier(Modifier::BOLD)
    }

    pub fn success() -> Style {
        Style::default()
            .fg(colors::SUCCESS)
            .add_modifier(Modifier::BOLD)
    }

    pub fn warning() -> Style {
        Style::default()
            .fg(colors::WARNING)
            .add_modifier(Modifier::BOLD)
    }

    pub fn danger() -> Style {
        Style::default()
            .fg(colors::DANGER_BRIGHT)
            .add_modifier(Modifier::BOLD)
    }

    pub fn highlight() -> Style {
        Style::default()
            .fg(colors::ACCENT)
            .add_modifier(Modifier::BOLD)
    }

    pub fn code() -> Style {
        Style::default()
            .fg(colors::SECONDARY)
            .add_modifier(Modifier::ITALIC)
    }

    pub fn emphasis() -> Style {
        Style::default()
            .fg(colors::PRIMARY)
            .add_modifier(Modifier::ITALIC)
    }
}

impl Widget for &Model {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let border_color = match self.screen {
            Screen::IsoSearch | Screen::DeviceSelect => colors::BORDER_ACTIVE,
            Screen::Confirm => colors::WARNING,
            Screen::Writing => colors::PRIMARY,
            Screen::Done => {
                if matches!(self.write_result, Some(Ok(_))) {
                    colors::SUCCESS
                } else if matches!(self.write_result, Some(Err(_))) {
                    colors::DANGER
                } else {
                    colors::BORDER_INACTIVE
                }
            }
        };

        let block = Block::bordered()
            .title(" ferric ")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(border_color));

        let inner = block.inner(area);
        block.render(area, buf);

        for y in inner.top()..inner.bottom() {
            for x in inner.left()..inner.right() {
                buf[(x, y)].set_bg(colors::BG_PRIMARY);
            }
        }

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),
                Constraint::Length(1),
            ])
            .split(inner);

        let body_area = layout[0];
        let footer_area = layout[1];

        match self.screen {
            Screen::IsoSearch => render_iso_search(self, body_area, buf),
            Screen::DeviceSelect => render_device_select(self, body_area, buf),
            Screen::Confirm => render_confirm(self, body_area, buf),
            Screen::Writing => render_writing(self, body_area, buf),
            Screen::Done => render_done(self, body_area, buf),
        }

        // Render footer hint
        render_footer(self, footer_area, buf);
    }
}

fn render_iso_search(model: &Model, area: Rect, buf: &mut Buffer) {
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(60),
            Constraint::Percentage(40),
        ])
        .split(area);

    let list_area = columns[0];
    let detail_area = columns[1];

    let mut list_items = vec![];
    for (i, meta) in model.iso_results.iter().enumerate() {
        let name = meta
            .path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("?");
        let size = human_size(meta.size);

        list_items.push(ListItem {
            primary: name.to_string(),
            secondary: Some(format!("({})", size)),
            badges: vec![],
            marker: if i == model.iso_selected {
                Some("▶")
            } else {
                None
            },
        });
    }

    let list_header_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Min(1),
        ])
        .split(list_area);

    let mut header_lines = vec![];
    header_lines.push(Line::from(Span::styled("ISO Search", styles::title())));
    header_lines.push(Line::from(vec![
        Span::styled("Type to filter; ", styles::text_muted()),
        Span::styled("Up/Down", styles::highlight()),
        Span::styled(" to move; ", styles::text_muted()),
        Span::styled("Enter", styles::highlight()),
        Span::styled(" to select", styles::text_muted()),
    ]));
    header_lines.push(Line::from(vec![
        Span::styled("Query: ", styles::text()),
        Span::styled(&model.iso_query, styles::code()),
    ]));
    if model.iso_searching {
        header_lines.push(Line::from(vec![
            Span::styled("● ", colors::PRIMARY),
            Span::styled("Searching...", styles::text_muted()),
        ]));
    } else if model.iso_results.is_empty() {
        header_lines.push(Line::from(Span::styled(
            "No results",
            styles::text_dim(),
        )));
    }

    Paragraph::new(header_lines)
        .alignment(Alignment::Left)
        .render(list_header_layout[0], buf);

    let is_focused = model.active_panel == ActivePanel::IsoList;
    let list_panel = ListPanel {
        title: "Results",
        items: list_items,
        selected: model.iso_selected,
        max_display: 15,
        focused: is_focused,
    };
    list_panel.render(list_header_layout[1], buf);

    let selected_iso = model.iso_results.get(model.iso_selected);
    let mut detail_items = vec![];

    if let Some(iso) = selected_iso {
        detail_items.push(DetailItem {
            label: "Name",
            value: iso
                .path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("?")
                .to_string(),
            style: styles::text(),
        });

        detail_items.push(DetailItem {
            label: "Size",
            value: human_size(iso.size),
            style: styles::emphasis(),
        });

        detail_items.push(DetailItem {
            label: "Path",
            value: iso.path.display().to_string(),
            style: styles::code(),
        });

        if let Some(modified) = iso.modified {
            if let Ok(elapsed) = modified.elapsed() {
                let secs = elapsed.as_secs();
                let time_str = if secs < 3600 {
                    format!("{} minutes ago", secs / 60)
                } else if secs < 86400 {
                    format!("{} hours ago", secs / 3600)
                } else {
                    format!("{} days ago", secs / 86400)
                };
                detail_items.push(DetailItem {
                    label: "Modified",
                    value: time_str,
                    style: styles::text_muted(),
                });
            }
        }
    } else {
        detail_items.push(DetailItem {
            label: "Status",
            value: "No selection".to_string(),
            style: styles::text_dim(),
        });
    }

    let detail_panel = DetailPanel {
        title: "ISO Details",
        items: detail_items,
        synced_focus: is_focused,
    };
    detail_panel.render(detail_area, buf);
}

fn render_device_select(model: &Model, area: Rect, buf: &mut Buffer) {
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(60),
            Constraint::Percentage(40),
        ])
        .split(area);

    let list_area = columns[0];
    let detail_area = columns[1];

    let mut list_items = vec![];
    for (i, device) in model.devices.iter().enumerate() {
        let primary = format!("{}  {}", device.name, human_size(device.size));

        let mut secondary_parts = vec![];

        if let Some(ref model_name) = device.model {
            secondary_parts.push(model_name.clone());
        }

        let mut badges = vec![];

        if device.removable {
            badges.push(Badge {
                text: "removable".to_string(),
                style: styles::success(),
            });
        }

        if device.mounted {
            badges.push(Badge {
                text: "mounted".to_string(),
                style: styles::warning(),
            });
        }

        let secondary = if secondary_parts.is_empty() {
            None
        } else {
            Some(secondary_parts.join(" "))
        };

        list_items.push(ListItem {
            primary,
            secondary,
            badges,
            marker: if i == model.device_selected {
                Some("▶")
            } else {
                None
            },
        });
    }

    let list_header_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Min(1),
        ])
        .split(list_area);

    let mut header_lines = vec![];
    header_lines.push(Line::from(Span::styled("Device Select", styles::title())));
    header_lines.push(Line::from(vec![
        Span::styled("Up/Down", styles::highlight()),
        Span::styled(" to move; ", styles::text_muted()),
        Span::styled("Enter", styles::highlight()),
        Span::styled(" to select; ", styles::text_muted()),
        Span::styled("r", styles::highlight()),
        Span::styled(" to refresh", styles::text_muted()),
    ]));

    if model.device_refreshing {
        header_lines.push(Line::from(vec![
            Span::styled("● ", colors::PRIMARY),
            Span::styled("Refreshing...", styles::text_muted()),
        ]));
    } else if model.devices.is_empty() {
        header_lines.push(Line::from(Span::styled(
            "No devices found",
            styles::text_dim(),
        )));
    } else {
        header_lines.push(Line::from(vec![
            Span::styled(format!("{} ", model.devices.len()), styles::emphasis()),
            Span::styled(
                if model.devices.len() == 1 { "device" } else { "devices" },
                styles::text_muted(),
            ),
            Span::styled(" found", styles::text_muted()),
        ]));
    }

    header_lines.push(Line::from(""));

    Paragraph::new(header_lines)
        .alignment(Alignment::Left)
        .render(list_header_layout[0], buf);

    let is_focused = model.active_panel == ActivePanel::DeviceList;
    let list_panel = ListPanel {
        title: "Available Devices",
        items: list_items,
        selected: model.device_selected,
        max_display: 12,
        focused: is_focused,
    };
    list_panel.render(list_header_layout[1], buf);

    let selected_device = model.devices.get(model.device_selected);
    let mut detail_items = vec![];

    if let Some(device) = selected_device {
        detail_items.push(DetailItem {
            label: "Device",
            value: device.name.clone(),
            style: styles::text(),
        });

        detail_items.push(DetailItem {
            label: "Path",
            value: device.path.display().to_string(),
            style: styles::code(),
        });

        detail_items.push(DetailItem {
            label: "Size",
            value: human_size(device.size),
            style: styles::emphasis(),
        });

        if let Some(ref model_name) = device.model {
            detail_items.push(DetailItem {
                label: "Model",
                value: model_name.clone(),
                style: styles::text(),
            });
        }

        if let Some(ref tran) = device.tran {
            detail_items.push(DetailItem {
                label: "Transport",
                value: tran.clone(),
                style: styles::text_muted(),
            });
        }

        detail_items.push(DetailItem {
            label: "Removable",
            value: if device.removable { "Yes" } else { "No" }.to_string(),
            style: if device.removable {
                styles::success()
            } else {
                styles::text_dim()
            },
        });

        detail_items.push(DetailItem {
            label: "Mounted",
            value: if device.mounted {
                "Yes ⚠"
            } else {
                "No"
            }
            .to_string(),
            style: if device.mounted {
                styles::warning()
            } else {
                styles::success()
            },
        });
    } else {
        detail_items.push(DetailItem {
            label: "Status",
            value: "No selection".to_string(),
            style: styles::text_dim(),
        });
    }

    let detail_panel = DetailPanel {
        title: "Device Details",
        items: detail_items,
        synced_focus: is_focused,
    };
    detail_panel.render(detail_area, buf);
}

fn render_confirm(model: &Model, area: Rect, buf: &mut Buffer) {
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
            Constraint::Percentage(15),
            Constraint::Min(1),
            Constraint::Percentage(15),
        ])
        .split(centered_layout[1]);

    let content_area = vertical_layout[1];

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
                Span::styled(format!("{}", p.display()), styles::code()),
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
                Span::styled(format!("{}", p.display()), styles::danger()),
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

    lines.push(Line::from(vec![
        Span::styled("┌──────┐", border_style),
    ]));
    lines.push(Line::from(vec![
        Span::styled("│ ", border_style),
        Span::styled(&input_with_cursor, confirm_style),
        Span::styled(&padding, Style::default()),
        Span::styled(" │", border_style),
    ]));
    lines.push(Line::from(vec![
        Span::styled("└──────┘", border_style),
    ]));

    lines.push(Line::from(""));
    if !model.is_root {
        lines.push(Line::from(vec![
            Span::styled("⚠ ", colors::WARNING),
            Span::styled("Not running as root. ", styles::warning()),
            Span::styled(
                "Writing typically requires ",
                styles::text_muted(),
            ),
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
            Span::styled(
                "Enter is disabled until you type YES.",
                styles::text_dim(),
            ),
        ]));
    }

    let paragraph = Paragraph::new(lines).alignment(Alignment::Center);
    paragraph.render(content_area, buf);
}

fn render_writing(model: &Model, area: Rect, buf: &mut Buffer) {
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
            extra_lines.push(Line::from(Span::styled(
                "Verifying...",
                styles::title(),
            )));

            let vt = model.verifying_total;
            let vc = model.verifying_checked;
            let vp = if vt > 0 {
                (vc as f64 / vt as f64) * 100.0
            } else {
                0.0
            };

            let bar_width: usize = 40;
            let v_filled = ((vp / 100.0) * bar_width as f64) as usize;
            let v_empty = bar_width.saturating_sub(v_filled);
            let v_bar = format!(
                "{}{}",
                "█".repeat(v_filled),
                "░".repeat(v_empty)
            );

            extra_lines.push(Line::from(vec![
                Span::styled("Verify: ", styles::text()),
                Span::styled(format!("{:>5.1}%", vp), styles::highlight()),
            ]));
            extra_lines.push(Line::from(Span::styled(
                v_bar,
                Style::default().fg(colors::ACCENT),
            )));

            extra_lines.push(Line::from(vec![
                Span::styled(format!("{}  ", human_size(vc)), styles::text()),
                Span::styled("/ ", styles::text_dim()),
                Span::styled(
                    format!("  {}", human_size(vt)),
                    styles::text_muted(),
                ),
            ]));

            let vbps = model.verifying_speed_bps;
            if vbps > 0.0 {
                extra_lines.push(Line::from(vec![
                    Span::styled("Verify speed: ", styles::text()),
                    Span::styled(
                        format!("{}/s", human_size(vbps as u64)),
                        styles::code(),
                    ),
                ]));
            }
        } else if model.write_result == Some(Ok(())) {
            extra_lines.push(Line::from(vec![
                Span::styled("● ", colors::PRIMARY),
                Span::styled(
                    "Verification pending...",
                    styles::text_muted(),
                ),
            ]));
        }
    }

    Paragraph::new(extra_lines)
        .alignment(Alignment::Center)
        .render(layout[2], buf);
}

fn render_done(model: &Model, area: Rect, buf: &mut Buffer) {
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

    let mut lines = vec![];

    match &model.write_result {
        Some(Ok(())) => {
            lines.push(Line::from(Span::styled("✓ Done", styles::success())));
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::styled("✓ ", colors::SUCCESS),
                Span::styled(
                    "Write completed successfully!",
                    styles::success(),
                ),
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
            // should not happen
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

fn render_footer(model: &Model, area: Rect, buf: &mut Buffer) {
    let hint = nav_cycle_hint(model);
    let paragraph = Paragraph::new(Line::from(Span::styled(hint, styles::text_dim())))
        .alignment(Alignment::Center);
    paragraph.render(area, buf);
}

fn human_size(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;
    let b = bytes as f64;
    if b >= GB {
        format!("{:.1} GiB", b / GB)
    } else if b >= MB {
        format!("{:.1} MiB", b / MB)
    } else if b >= KB {
        format!("{:.0} KiB", b / KB)
    } else {
        format!("{} B", bytes)
    }
}

fn nav_cycle_hint(m: &Model) -> String {
    match m.screen {
        Screen::IsoSearch => {
            let focus_label = match m.active_panel {
                ActivePanel::IsoList => "ISO Search",
                ActivePanel::DeviceList => "Device Select",
                ActivePanel::ConfirmInput => "Confirm",
            };
            format!("Focus: {} | Tab/Shift-Tab: Switch Panels | q: Quit", focus_label)
        }
        Screen::DeviceSelect => {
            let focus_label = match m.active_panel {
                ActivePanel::IsoList => "ISO Search",
                ActivePanel::DeviceList => "Device Select",
                ActivePanel::ConfirmInput => "Confirm",
            };
            format!("Focus: {} | Tab/Shift-Tab: Switch Panels | r: Refresh | q: Quit", focus_label)
        }
        Screen::Confirm => {
            format!("Focus: Confirm | Type YES then Enter | Esc: Back | q: Quit")
        }
        Screen::Writing => {
            "Writing in progress... Please wait | Do NOT remove device".to_string()
        }
        Screen::Done => {
            "Complete | Esc: Back | q: Quit".to_string()
        }
    }
}

fn contains_perm_denied(s: &str) -> bool {
    let lower = s.to_ascii_lowercase();
    lower.contains("permission denied")
        || lower.contains("operation not permitted")
}
