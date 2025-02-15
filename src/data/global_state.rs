use std::sync::{Arc, LazyLock, RwLock};

use tokio::sync::watch;

use super::{controler::ControllerInfo, device::Device};

#[derive(Default)]
pub struct GlobalState {
    pub is_scanning: Arc<RwLock<bool>>,
    pub paired_devices: Arc<RwLock<Vec<Device>>>,
    pub scanned_devices: Arc<RwLock<Vec<Device>>>,
    pub controller_info: Arc<RwLock<ControllerInfo>>,
}

pub static GLOBAL_STATE: LazyLock<Arc<RwLock<GlobalState>>> =
    LazyLock::new(|| Arc::new(RwLock::new(GlobalState::default())));

pub static CONNECTED_DEVICE: LazyLock<Arc<RwLock<Option<usize>>>> =
    LazyLock::new(|| Arc::new(RwLock::new(None)));

pub static UPDATE_UI: LazyLock<(watch::Sender<usize>, watch::Receiver<usize>)> =
    LazyLock::new(|| {
        let (sen, rec) = watch::channel(0);
        (sen, rec)
    });
