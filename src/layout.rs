use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub fn main_layout(area: Rect) -> Vec<Rect> {
    return Layout::default()
        .direction(Direction::Vertical)
        .constraints(Constraint::from_percentages([7, 93]))
        .split(area)
        .to_vec();
}
