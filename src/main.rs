use std::{io, sync::Arc, time::Duration};
mod layout;
use bluetooth::{controller_info, scan_devices};
use data::global_state::{GLOBAL_STATE, UPDATE_UI};
use layout::main_layout;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    widgets::{Block, Widget},
    DefaultTerminal, Frame,
};
use tokio::time;
use widgets::{current_status::CurrentStatusWidget, scanned_devices::DevicesList};

mod bluetooth;
mod data;
mod widgets;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    tokio::spawn(async move { scan_devices().await });
    controller_info().await;
    Ok(())
    // let mut terminal = ratatui::init();
    // let app_result = Bluetui::default().run(&mut terminal).await;
    // ratatui::restore();
    // app_result
}

#[derive(Debug, Default)]
struct Bluetui {
    exit: bool,
}

impl Bluetui {
    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let mut interval = time::interval(Duration::from_millis(1000));
        while !self.exit {
            tokio::select! {
                _ = interval.tick() => {
                    let rec = UPDATE_UI.1.clone();
                    if rec.has_changed().is_ok() {
                        terminal.draw(|frame| self.draw(frame))?;
                    }
                    continue
                },
                _ = self.handle_events() => {
                    if self.exit {
                        break
                    }
                }
            };
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let global = Arc::clone(&GLOBAL_STATE);
        let state = global.read().unwrap();

        let scanned = &state.scanned_devices;

        let area = frame.area();
        let layout = main_layout(area);
        let xd = Block::bordered().title_top(scanned.len().to_string());
        let dx = Block::bordered();
        frame.render_widget(CurrentStatusWidget, layout[0]);
        frame.render_widget(DevicesList, layout[1]);
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => {
                println!("Exiting..."); // Debugging
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
