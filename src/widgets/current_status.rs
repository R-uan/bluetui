use std::sync::Arc;

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Paragraph, Widget},
};

use crate::data::global_state::{GLOBAL_STATE, UPDATE_UI};

pub struct CurrentStatusWidget;

impl Widget for CurrentStatusWidget {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let main = Layout::horizontal([
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(10),
        ])
        .split(area);

        let global = Arc::clone(&GLOBAL_STATE);
        let state = global.read().unwrap();
        let ui_state = UPDATE_UI.1.clone();

        let power_status = match state.controller_info.powered {
            true => "power on",
            false => "power off",
        };

        let power_style = match state.controller_info.powered {
            true => Style::new().fg(Color::LightGreen),
            false => Style::new().fg(Color::LightRed),
        };

        Paragraph::new(power_status)
            .alignment(Alignment::Center)
            .style(power_style)
            .render(main[0], buf);

        Paragraph::new(state.scanned_devices.len().to_string())
            .alignment(Alignment::Center)
            .render(main[1], buf);

        Paragraph::new("connection")
            .alignment(Alignment::Center)
            .render(main[2], buf);
    }
}
