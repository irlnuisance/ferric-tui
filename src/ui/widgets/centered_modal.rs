use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub struct CenteredModal {
    pub horizontal_margin_pct: u16,
    pub vertical_margin_pct: u16,
}

impl Default for CenteredModal {
    fn default() -> Self {
        Self {
            horizontal_margin_pct: 20,
            vertical_margin_pct: 20,
        }
    }
}

impl CenteredModal {
    pub fn compute_area(&self, area: Rect) -> Rect {
        let content_pct = 100 - (2 * self.horizontal_margin_pct);

        let horizontal_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(self.horizontal_margin_pct),
                Constraint::Percentage(content_pct),
                Constraint::Percentage(self.horizontal_margin_pct),
            ])
            .split(area);

        let vertical_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(self.vertical_margin_pct),
                Constraint::Min(1),
                Constraint::Percentage(self.vertical_margin_pct),
            ])
            .split(horizontal_layout[1]);

        vertical_layout[1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_centered_modal_default() {
        let modal = CenteredModal::default();
        assert_eq!(modal.horizontal_margin_pct, 20);
        assert_eq!(modal.vertical_margin_pct, 20);
    }

    #[test]
    fn test_centered_modal_compute_area() {
        let modal = CenteredModal::default();
        let area = Rect::new(0, 0, 100, 100);
        let content_area = modal.compute_area(area);

        assert_eq!(content_area.width, 60);
    }

    #[test]
    fn test_centered_modal_custom_margins() {
        let modal = CenteredModal {
            horizontal_margin_pct: 30,
            vertical_margin_pct: 15,
        };

        let area = Rect::new(0, 0, 100, 100);
        let content_area = modal.compute_area(area);

        assert_eq!(content_area.width, 40);
    }
}
