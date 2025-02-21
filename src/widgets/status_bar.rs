use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    text::{Line, Span, Text},
    widgets::{Block, Padding, Paragraph, Widget},
};

use crate::data::global_state::GLOBAL;
#[derive(Default)]
pub struct StatusBar;

impl StatusBar {
    pub fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let xdd = GLOBAL.read().unwrap();

        let constraints = [Constraint::Percentage(33), Constraint::Percentage(33)];
        let layout = Layout::horizontal(constraints)
            .flex(Flex::SpaceBetween)
            .split(area);

        let tesxt = vec![
            Line::from(format!(" Name: {}", xdd.controller.name)),
            Line::from(format!(" Powered: {}", xdd.controller.powered)),
            Line::from(format!(" Pairable: {}", xdd.controller.pairable)),
        ];

        Paragraph::new(tesxt)
            .alignment(Alignment::Left)
            .block(Block::bordered().title_top("CONTROLLER"))
            .render(layout[0], buf);

        Block::bordered().render(layout[1], buf);
    }
}
