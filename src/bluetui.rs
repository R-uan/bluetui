use std::{io::Result, time::Duration};

use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::Rect,
    widgets::Widget,
    DefaultTerminal,
};

use crate::widgets::devices_list::DevicesList;

#[derive(Default)]
pub struct Bluetui {
    pub exit: bool,
    pub device_list: DevicesList,
}

impl Bluetui {
    pub async fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            self.handle_events().await?;
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
        }
        Ok(())
    }

    async fn handle_events(&mut self) -> Result<()> {
        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(key_event) = event::read()? {
                if key_event.kind == KeyEventKind::Press {
                    self.handle_key_event(key_event);
                }
            }
        }

        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('j') | KeyCode::Down => self.next_(),
            KeyCode::Char('k') | KeyCode::Up => {}
            KeyCode::Char('q') | KeyCode::Esc => self.exit(),
            _ => {}
        }
    }
}

impl Widget for &mut Bluetui {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.device_list.render(area, buf);
    }
}

impl Bluetui {
    fn exit(&mut self) {
        self.exit = true;
    }

    fn next_(&mut self) {
        self.device_list.list_state.write().unwrap().select_next();
    }
}
