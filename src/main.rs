mod bluetooth;
mod data;
mod layout;
mod ui;
mod widgets;

use bluetooth::{controller::controller_info, devices::scan_devices};
use ui::bluetui::Bluetui;

async fn initialization() {
    tokio::spawn(async move { controller_info().await });
    tokio::spawn(async move { scan_devices().await });
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    tokio::spawn(async move { initialization().await });

    let mut terminal = ratatui::init();
    let app_result = Bluetui::default().run(&mut terminal).await;
    ratatui::restore();

    app_result
}
