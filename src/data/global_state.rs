use std::sync::{Arc, LazyLock, RwLock};

use tokio::sync::watch;

use super::{controler::ControllerInfo, device::Device};

#[derive(Default)]
pub struct GlobalState {
    pub is_scanning: bool,
    pub paired_devices: Vec<Device>,
    pub scanned_devices: Vec<Device>,
    pub controller_info: ControllerInfo,
}

pub static GLOBAL_STATE: LazyLock<Arc<RwLock<GlobalState>>> =
    LazyLock::new(|| Arc::new(RwLock::new(GlobalState::default())));

pub static UPDATE_UI: LazyLock<(watch::Sender<usize>, watch::Receiver<usize>)> =
    LazyLock::new(|| {
        let (sen, rec) = watch::channel(0);
        (sen, rec)
    });
