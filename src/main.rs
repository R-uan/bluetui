use std::{io, sync::Arc, thread, time::Duration};
mod layout;
use bluetooth::{scan_devices, BluetoothService};
use data::global_state::{GLOBAL_STATE, UPDATE_UI};
use layout::main_layout;
use ratatui::{
    crossterm::{
        event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
        terminal::enable_raw_mode,
    },
    widgets::Block,
    DefaultTerminal, Frame,
};
use tokio::time;

mod bluetooth;
mod data;
mod widgets;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    tokio::spawn(async move { scan_devices().await });
    enable_raw_mode()?;
    let mut terminal = ratatui::init();
    let app_result = Bluetui::default().run(&mut terminal).await;
    ratatui::restore();
    app_result
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
        frame.render_widget(xd, layout[0]);
        frame.render_widget(dx, layout[1]);
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
        // Check if there are any key events available without blocking
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
