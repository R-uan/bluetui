use std::sync::Arc;

use ratatui::widgets::{Block, List, Widget};

use crate::data::global_state::GLOBAL_STATE;

pub struct DevicesList;

impl Widget for DevicesList {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let global = Arc::clone(&GLOBAL_STATE);
        let state = global.read().unwrap();

        let paired_devices = &state.scanned_devices;

        let items: Vec<String> = paired_devices
            .iter()
            .map(|pd| {
                let name = &pd.name;
                let mac_addr = &pd.mac_addr;
                println!("{name}");
                format!("{name} {mac_addr}")
            })
            .collect();

        List::new(items).highlight_symbol(">").render(area, buf);
    }
}
