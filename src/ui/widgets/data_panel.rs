use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::Style,
    widgets::{Block, BorderType, Cell, Row, StatefulWidget, Table, TableState, Widget},
};

use crate::ui::theme::colors;

pub struct DataPanel<'a, T> {
    pub title: &'a str,
    pub items: &'a [T],
    pub selected: usize,
    pub columns: Vec<ColumnDef>,
    pub row_mapper: Box<dyn Fn(&T) -> Vec<Cell<'a>> + 'a>,
    pub focused: bool,
}

pub struct ColumnDef {
    pub constraint: Constraint,
}

impl ColumnDef {
    pub fn new(constraint: Constraint) -> Self {
        Self { constraint }
    }
}

impl<'a, T> Widget for DataPanel<'a, T> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (border_color, border_type) = if self.focused {
            (colors::BORDER_FOCUS, BorderType::Double)
        } else {
            (colors::BORDER_INACTIVE, BorderType::Rounded)
        };

        let block = Block::bordered()
            .title(self.title)
            .border_type(border_type)
            .border_style(Style::default().fg(border_color));

        let rows: Vec<Row> = self
            .items
            .iter()
            .map(|item| Row::new((self.row_mapper)(item)))
            .collect();

        let constraints: Vec<Constraint> = self.columns.iter().map(|c| c.constraint).collect();

        let table = Table::new(rows, constraints)
            .block(block)
            .highlight_symbol("▶ ");

        let mut table_state = TableState::default();
        let selected = if self.items.is_empty() {
            None
        } else {
            Some(self.selected.min(self.items.len() - 1))
        };
        table_state.select(selected);

        StatefulWidget::render(table, area, buf, &mut table_state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone)]
    struct TestItem {
        name: String,
        value: u32,
    }

    #[test]
    fn test_column_def_new() {
        let col = ColumnDef::new(Constraint::Percentage(50));
        assert!(matches!(col.constraint, Constraint::Percentage(50)));
    }

    #[test]
    fn test_column_def_with_length_constraint() {
        let col = ColumnDef::new(Constraint::Length(10));
        assert!(matches!(col.constraint, Constraint::Length(10)));
    }

    #[test]
    fn test_data_panel_empty_items() {
        let items: Vec<TestItem> = vec![];
        let panel = DataPanel {
            title: "Empty",
            items: &items,
            selected: 0,
            columns: vec![ColumnDef::new(Constraint::Percentage(100))],
            row_mapper: Box::new(|item| vec![Cell::from(item.name.clone())]),
            focused: false,
        };

        let area = Rect::new(0, 0, 40, 10);
        let mut buf = Buffer::empty(area);
        panel.render(area, &mut buf);
    }

    #[test]
    fn test_data_panel_selection_clamping() {
        let items = vec![
            TestItem {
                name: "Item 1".to_string(),
                value: 1,
            },
            TestItem {
                name: "Item 2".to_string(),
                value: 2,
            },
        ];

        let panel = DataPanel {
            title: "Test",
            items: &items,
            selected: 99,
            columns: vec![ColumnDef::new(Constraint::Percentage(100))],
            row_mapper: Box::new(|item| vec![Cell::from(item.name.clone())]),
            focused: false,
        };

        let area = Rect::new(0, 0, 40, 10);
        let mut buf = Buffer::empty(area);
        panel.render(area, &mut buf);
    }

    #[test]
    fn test_data_panel_focused_vs_unfocused() {
        let items = vec![TestItem {
            name: "Item".to_string(),
            value: 1,
        }];

        let focused_panel = DataPanel {
            title: "Focused",
            items: &items,
            selected: 0,
            columns: vec![ColumnDef::new(Constraint::Percentage(100))],
            row_mapper: Box::new(|item| vec![Cell::from(item.name.clone())]),
            focused: true,
        };

        let unfocused_panel = DataPanel {
            title: "Unfocused",
            items: &items,
            selected: 0,
            columns: vec![ColumnDef::new(Constraint::Percentage(100))],
            row_mapper: Box::new(|item| vec![Cell::from(item.name.clone())]),
            focused: false,
        };

        let area = Rect::new(0, 0, 40, 10);
        let mut buf1 = Buffer::empty(area);
        let mut buf2 = Buffer::empty(area);

        focused_panel.render(area, &mut buf1);
        unfocused_panel.render(area, &mut buf2);
    }
}
