use ratatui::widgets::{Block, List, Widget};

pub struct DevicesList;

impl Widget for DevicesList {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let items: Vec<&str> = vec!["aaaaaa", "bbbbbb"];
        List::new(items).highlight_symbol(">").render(area, buf);
    }
}
