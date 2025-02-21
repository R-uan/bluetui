use std::sync::{Arc, RwLock};

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, HighlightSpacing, List, ListState, StatefulWidget},
};

use crate::data::global_state::GLOBAL;

pub struct DevicesList {
    pub list_state: Arc<RwLock<ListState>>,
}

impl DevicesList {
    pub fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let mut state = self.list_state.write().unwrap();

        let clone_global = Arc::clone(&GLOBAL);
        let global_state = clone_global.read().unwrap();
        let entries: Vec<String> = global_state.devices.iter().map(|d| d.to_string()).collect();

        let list = List::new(entries)
            .highlight_symbol(".")
            .highlight_spacing(HighlightSpacing::Always)
            .block(Block::bordered());

        StatefulWidget::render(list, area, buf, &mut state);
    }
}

impl Default for DevicesList {
    fn default() -> Self {
        Self {
            list_state: Default::default(),
        }
    }
}
