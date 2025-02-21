use std::{io::Result, time::Duration};

use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    widgets::Widget,
    DefaultTerminal,
};

use crate::{
    data::global_state::GLOBAL,
    widgets::{devices_list::DevicesList, status_bar::StatusBar},
};

#[derive(Default)]
pub struct Bluetui {
    pub exit: bool,
    pub status_bar: StatusBar,
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
            KeyCode::Char('j') | KeyCode::Down => self.next_device(),
            KeyCode::Char('k') | KeyCode::Up => self.previous_device(),
            KeyCode::Char('q') | KeyCode::Esc => self.exit_app(),
            KeyCode::Enter => self.select_device(),
            _ => {}
        }
    }
}

impl Widget for &mut Bluetui {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout =
            Layout::vertical([Constraint::Percentage(10), Constraint::Percentage(90)]).split(area);

        self.status_bar.render(layout[0], buf);
        self.device_list.render(layout[1], buf);
    }
}

impl Bluetui {
    fn exit_app(&mut self) {
        self.exit = true;
    }

    fn next_device(&mut self) {
        self.device_list.list_state.write().unwrap().select_next();
    }

    fn previous_device(&mut self) {
        self.device_list
            .list_state
            .write()
            .unwrap()
            .select_previous();
    }

    fn select_device(&mut self) {
        let a = self
            .device_list
            .list_state
            .read()
            .unwrap()
            .selected()
            .unwrap();

        let xdd = GLOBAL.read().unwrap();
        println!("{}", xdd.devices[a].mac_addr);
    }
}
