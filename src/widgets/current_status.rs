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

        let state = GLOBAL_STATE.read().unwrap();
        let power_status = match state.controller_info.read().unwrap().powered {
            true => "power on",
            false => "power off",
        };

        let power_style = match state.controller_info.read().unwrap().powered {
            true => Style::new().fg(Color::LightGreen),
            false => Style::new().fg(Color::LightRed),
        };

        Paragraph::new(power_status)
            .alignment(Alignment::Center)
            .style(power_style)
            .render(main[0], buf);

        Paragraph::new(state.scanned_devices.read().unwrap().len().to_string())
            .alignment(Alignment::Center)
            .render(main[1], buf);

        let device = if let Some(index) = *CONNECTED_DEVICE.read().unwrap() {
            Some(GLOBAL_STATE.read().unwrap().scanned_devices.read().unwrap()[index].clone())
        } else {
            None
        };

        let caralho = if let Some(d) = device {
            &format!("Connected: {}", d.name)
        } else {
            "None"
        };

        Paragraph::new(caralho)
            .alignment(Alignment::Center)
            .render(main[2], buf);
    }
}
