#![allow(dead_code)]
use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub fn split_body_footer(area: Rect) -> [Rect; 2] {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(area);
    [chunks[0], chunks[1]]
}

pub fn split_two_columns(area: Rect, left_pct: u16) -> [Rect; 2] {
    let left = left_pct.clamp(1, 99);
    let right = 100 - left;
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(left), Constraint::Percentage(right)])
        .split(area);
    [chunks[0], chunks[1]]
}

pub fn split_header_body_footer(area: Rect) -> [Rect; 3] {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(area);
    [chunks[0], chunks[1], chunks[2]]
}
