use crate::data;
use ratatui::widgets::Widget;
use std::sync::{Arc, LazyLock, Mutex};

#[derive(Default, Debug)]
pub struct CurrentStatus {
    power: bool,
}

pub struct CurrentStatusWidget;

static STATE: LazyLock<Arc<CurrentStatus>> = LazyLock::new(|| Arc::new(CurrentStatus::default()));

impl Widget for CurrentStatusWidget {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        todo!()
    }
}
