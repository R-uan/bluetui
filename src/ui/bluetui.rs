use std::{
    io::{self, Write},
    sync::Arc,
    time::Duration,
};

use ratatui::{
    crossterm::{
        event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
        terminal::{Clear, ClearType},
        ExecutableCommand,
    },
    DefaultTerminal, Frame,
};
use tokio::time;

use crate::{
    data::global_state::{GLOBAL_STATE, UPDATE_UI},
    layout::main_layout,
    widgets::{current_status::CurrentStatusWidget, scanned_devices::DevicesList},
};
#[derive(Debug, Default)]

pub struct Bluetui {
    exit: bool,
}

impl Bluetui {
    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let mut interval = time::interval(Duration::from_millis(1000));
        terminal.clear()?;
        while !self.exit {
            tokio::select! {
                _ = interval.tick() => {
                    let rec = UPDATE_UI.1.clone();
                    if rec.has_changed().is_ok() {
                        terminal.clear()?;
                        terminal.draw(|frame| self.draw(frame))?;
                    }
                    continue
                },
                _ = self.handle_events() => {}
            };
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let area = frame.area();
        let layout = main_layout(area);

        frame.render_widget(CurrentStatusWidget, layout[0]);
        frame.render_widget(DevicesList, layout[1]);
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => {
                self.exit();
            }
            _ => {}
        }
    }

    async fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(key_event) = event::read()? {
                if key_event.kind == KeyEventKind::Press {
                    self.handle_key_event(key_event);
                }
            }
        }
        Ok(())
    }
}
