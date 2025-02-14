use std::sync::Arc;

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    style::{Color, Style},
    widgets::{Paragraph, Widget},
};

use crate::data::global_state::{CONNECTED_DEVICE, GLOBAL_STATE};

pub struct CurrentStatusWidget;

impl CurrentStatusWidget {
    pub fn render(area: Rect, buf: &mut Buffer) {
        let main = Layout::horizontal([
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(10),
        ])
        .flex(Flex::SpaceAround)
        .split(area);

        let global = Arc::clone(&GLOBAL_STATE);
        let state = global.read().unwrap();
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

        let cl = Arc::clone(&CONNECTED_DEVICE);
        let binding = cl.read().unwrap();
        let xdx = binding.as_ref();

        let caralho = if let Some(_) = xdx {
            "Connected"
        } else {
            "None"
        };

        Paragraph::new(caralho)
            .alignment(Alignment::Center)
            .render(main[2], buf);
    }
}
