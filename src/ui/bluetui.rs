use std::{
    io::{self},
    time::Duration,
};

use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{ListState, Widget},
    DefaultTerminal,
};
use tokio::time;

use crate::{
    data::global_state::UPDATE_UI,
    widgets::{current_status::CurrentStatusWidget, scanned_devices::ScannedDevices},
};
#[derive(Debug)]
pub struct Bluetui {
    exit: bool,
    state: ScannedDevices,
}

impl Bluetui {
    pub async fn run(mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let mut interval = time::interval(Duration::from_millis(1000));
        terminal.clear()?;
        while !self.exit {
            tokio::select! {
                _ = interval.tick() => {
                    let rec = UPDATE_UI.1.clone();
                    if rec.has_changed().is_ok() {
                        terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
                    }
                    continue
                },
                _ = self.handle_events() => {}
            };
        }
        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('j') | KeyCode::Down => self.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.select_previous(),
            KeyCode::Char('p') | KeyCode::Enter => self.pair_device(),
            KeyCode::Char('q') => {
                self.exit();
            }
            _ => {}
        }
    }

    fn pair_device(&mut self) {}

    fn select_next(&mut self) {
        self.state.state.select_next();
    }
    fn select_previous(&mut self) {
        self.state.state.select_previous();
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

impl Widget for &mut Bluetui {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [status_area, devices_area] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Length(20)])
            .areas(area);

        CurrentStatusWidget::render(status_area, buf);
        self.state.render(devices_area, buf);
    }
}

impl Default for Bluetui {
    fn default() -> Self {
        Self {
            exit: false,
            state: Default::default(),
        }
    }
}
