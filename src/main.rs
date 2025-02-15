use std::{
    thread::{self, sleep},
    time::Duration,
};

use bluetooth::devices::{known_devices, scan_devices};
use data::global_state::GLOBAL;

mod bluetooth;

mod data;
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    thread::spawn(move || known_devices());
    thread::spawn(move || scan_devices());

    loop {
        let global = GLOBAL.read().unwrap().devices.len();
        println!("{}", global);
        sleep(Duration::from_secs(1));
    }

    Ok(())
}
