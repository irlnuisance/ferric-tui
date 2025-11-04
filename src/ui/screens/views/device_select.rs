use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::{Cell, Widget},
};

use crate::ui::{
    components::{DetailItem, DetailPanel},
    core::{DeviceSelectProps, UiCtx, View},
    theme::{colors, styles},
    widgets::{DataPanel, InstructionHeader},
};

pub struct DeviceSelectScreen;

impl View for DeviceSelectScreen {
    type Props<'a> = DeviceSelectProps<'a>;

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
        header_lines.push(Line::from(Span::styled("Device Select", styles::title())));
        header_lines.push(Line::from(vec![
            Span::styled("Up/Down", styles::highlight()),
            Span::styled(" to move; ", styles::text_muted()),
            Span::styled("Enter", styles::highlight()),
            Span::styled(" to select; ", styles::text_muted()),
            Span::styled("r", styles::highlight()),
            Span::styled(" to refresh", styles::text_muted()),
        ]));

        if props.refreshing {
            header_lines.push(Line::from(vec![
                Span::styled("● ", colors::PRIMARY),
                Span::styled("Refreshing...", styles::text_muted()),
            ]));
        } else if props.devices.is_empty() {
            header_lines.push(Line::from(Span::styled(
                "No devices found",
                styles::text_dim(),
            )));
        } else {
            header_lines.push(Line::from(vec![
                Span::styled(format!("{} ", props.devices.len()), styles::emphasis()),
                Span::styled(
                    if props.devices.len() == 1 {
                        "device"
                    } else {
                        "devices"
                    },
                    styles::text_muted(),
                ),
                Span::styled(" found", styles::text_muted()),
            ]));
        }

        header_lines.push(Line::from(""));

        let instruction_header = InstructionHeader {
            lines: header_lines,
        };
        instruction_header.render(list_header_layout[0], buf);

        let data_panel = DataPanel {
            title: "Available Devices",
            items: props.devices,
            selected: props.selected,
            columns: vec![
                crate::ui::widgets::ColumnDef::new(Constraint::Percentage(30)),
                crate::ui::widgets::ColumnDef::new(Constraint::Length(9)),
                crate::ui::widgets::ColumnDef::new(Constraint::Percentage(20)),
                crate::ui::widgets::ColumnDef::new(Constraint::Percentage(30)),
            ],
            row_mapper: Box::new(|d| {
                let flags_line = {
                    let mut spans: Vec<Span> = Vec::new();
                    if d.removable {
                        spans.push(Span::styled("removable", styles::success()));
                    }
                    if d.mounted {
                        if !spans.is_empty() {
                            spans.push(Span::raw(" "));
                        }
                        spans.push(Span::styled("mounted", styles::warning()));
                    }
                    if spans.is_empty() {
                        Line::from(Span::styled("-", styles::text_dim()))
                    } else {
                        Line::from(spans)
                    }
                };
                let model_name = d.model.clone().unwrap_or_default();
                vec![
                    Cell::from(d.name.clone()),
                    Cell::from(d.size.to_string()),
                    Cell::from(flags_line),
                    Cell::from(model_name),
                ]
            }),
            focused: props.focused,
        };

        data_panel.render(list_header_layout[1], buf);

        let selected_device = props.devices.get(props.selected);
        let mut detail_items = vec![];

        if let Some(device) = selected_device {
            detail_items.push(DetailItem {
                label: "Device",
                value: device.name.clone(),
                style: styles::text(),
            });

            detail_items.push(DetailItem {
                label: "Path",
                value: device.path.to_string(),
                style: styles::code(),
            });

            detail_items.push(DetailItem {
                label: "Size",
                value: device.size.to_string(),
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
                value: if device.mounted { "Yes ⚠" } else { "No" }.to_string(),
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
            synced_focus: props.focused,
        };
        detail_panel.render(detail_area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_select_screen_empty() {
        let screen = DeviceSelectScreen;
        let props = DeviceSelectProps {
            devices: &[],
            selected: 0,
            refreshing: false,
            focused: true,
        };
        let ctx = UiCtx::new();
        let area = Rect::new(0, 0, 120, 40);
        let mut buf = Buffer::empty(area);

        screen.render(&props, &ctx, area, &mut buf);
    }

    #[test]
    fn test_device_select_screen_refreshing() {
        let screen = DeviceSelectScreen;
        let props = DeviceSelectProps {
            devices: &[],
            selected: 0,
            refreshing: true,
            focused: true,
        };
        let ctx = UiCtx::new();
        let area = Rect::new(0, 0, 120, 40);
        let mut buf = Buffer::empty(area);

        screen.render(&props, &ctx, area, &mut buf);
    }
}
