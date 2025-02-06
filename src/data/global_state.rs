use std::sync::{Arc, LazyLock, RwLock};

use tokio::sync::watch;

use crate::bluetooth::BluetoothService;

use super::devices::Devices;

#[derive(Default)]
pub struct GlobalState {
    pub scanning: bool,
    pub paired_devices: Vec<Devices>,
    pub scanned_devices: Vec<Devices>,
}

pub static GLOBAL_STATE: LazyLock<Arc<RwLock<GlobalState>>> =
    LazyLock::new(|| Arc::new(RwLock::new(GlobalState::default())));

pub static UPDATE_UI: LazyLock<(watch::Sender<usize>, watch::Receiver<usize>)> =
    LazyLock::new(|| {
        let (sen, rec) = watch::channel(0);
        (sen, rec)
    });
