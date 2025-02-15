use std::sync::{Arc, LazyLock, RwLock};

use super::device::Device;

#[derive(Default)]
pub struct GlobalState {
    pub devices: Vec<Device>,
}

pub static GLOBAL: LazyLock<Arc<RwLock<GlobalState>>> =
    LazyLock::new(|| Arc::new(RwLock::new(GlobalState::default())));
