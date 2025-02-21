use std::thread::{self};

use bluetooth::devices::{known_devices, scan_devices};
use bluetui::Bluetui;

mod bluetooth;
mod bluetui;
mod utils;
mod widgets;

mod data;
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    thread::spawn(move || known_devices());
    thread::spawn(move || scan_devices());

    let mut terminal = ratatui::init();
    let app = Bluetui::default().run(&mut terminal).await;
    ratatui::restore();

    app
}
