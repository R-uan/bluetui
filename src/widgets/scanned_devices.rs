use std::sync::Arc;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::{Text, ToText},
    widgets::{Block, HighlightSpacing, List, ListState, StatefulWidget},
};

use crate::data::global_state::GLOBAL_STATE;
#[derive(Debug, Default)]
pub struct ScannedDevices {
    pub state: ListState,
}

impl ScannedDevices {
    pub fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let global = Arc::clone(&GLOBAL_STATE);
        let state = global.read().unwrap();
        let block = Block::bordered();

        let binding = state.scanned_devices.read().unwrap();
        let entries: Vec<Text<'_>> = binding.iter().map(|s| s.to_text()).collect();

        let list = List::new(entries)
            .highlight_symbol(">")
            .block(block)
            .highlight_spacing(HighlightSpacing::WhenSelected);

        StatefulWidget::render(list, area, buf, &mut self.state);
    }
}
