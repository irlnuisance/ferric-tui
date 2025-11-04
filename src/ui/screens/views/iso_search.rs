use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::{Cell, Widget},
};

use crate::ui::{
    components::{DetailItem, DetailPanel},
    core::{IsoSearchProps, UiCtx, View},
    theme::{colors, styles},
    widgets::{DataPanel, InstructionHeader},
};

pub struct IsoSearchScreen;

impl View for IsoSearchScreen {
    type Props<'a> = IsoSearchProps<'a>;

    fn render<'a>(&self, props: &Self::Props<'a>, _ctx: &UiCtx, area: Rect, buf: &mut Buffer) {
        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
            .split(area);

        let list_area = columns[0];
        let detail_area = columns[1];

        let list_header_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(5), Constraint::Min(1)])
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
            Span::styled(props.query, styles::code()),
        ]));
        if props.searching {
            header_lines.push(Line::from(vec![
                Span::styled("● ", colors::PRIMARY),
                Span::styled("Searching...", styles::text_muted()),
            ]));
        } else if props.results.is_empty() {
            header_lines.push(Line::from(Span::styled("No results", styles::text_dim())));
        }

        let instruction_header = InstructionHeader {
            lines: header_lines,
        };
        instruction_header.render(list_header_layout[0], buf);

        let data_panel = DataPanel {
            title: "Results",
            items: props.results,
            selected: props.selected,
            columns: vec![
                crate::ui::widgets::ColumnDef::new(Constraint::Percentage(70)),
                crate::ui::widgets::ColumnDef::new(Constraint::Percentage(30)),
            ],
            row_mapper: Box::new(|meta| {
                let name = meta
                    .path
                    .as_path()
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("?");
                let size = meta.size.to_string();
                vec![Cell::from(name.to_string()), Cell::from(size)]
            }),
            focused: props.focused,
        };

        data_panel.render(list_header_layout[1], buf);

        let selected_iso = props.results.get(props.selected);
        let mut detail_items = vec![];

        if let Some(iso) = selected_iso {
            detail_items.push(DetailItem {
                label: "Name",
                value: iso
                    .path
                    .as_path()
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("?")
                    .to_string(),
                style: styles::text(),
            });

            detail_items.push(DetailItem {
                label: "Size",
                value: iso.size.to_string(),
                style: styles::emphasis(),
            });

            detail_items.push(DetailItem {
                label: "Path",
                value: iso.path.to_string(),
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
            synced_focus: props.focused,
        };
        detail_panel.render(detail_area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iso_search_screen_empty() {
        let screen = IsoSearchScreen;
        let props = IsoSearchProps {
            query: "",
            searching: false,
            results: &[],
            selected: 0,
            focused: true,
        };
        let ctx = UiCtx::new();
        let area = Rect::new(0, 0, 120, 40);
        let mut buf = Buffer::empty(area);

        screen.render(&props, &ctx, area, &mut buf);
    }

    #[test]
    fn test_iso_search_screen_searching() {
        let screen = IsoSearchScreen;
        let props = IsoSearchProps {
            query: "ubuntu",
            searching: true,
            results: &[],
            selected: 0,
            focused: true,
        };
        let ctx = UiCtx::new();
        let area = Rect::new(0, 0, 120, 40);
        let mut buf = Buffer::empty(area);

        screen.render(&props, &ctx, area, &mut buf);
    }
}
