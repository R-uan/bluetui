use std::sync::{Arc, LazyLock, RwLock};

use super::{controller::ControllerInfo, device::Device};

pub struct GlobalState {
    pub controller: ControllerInfo,
    pub devices: Vec<Device>,
}

impl Default for GlobalState {
    fn default() -> Self {
        Self {
            devices: Default::default(),
            controller: ControllerInfo::init(),
        }
    }
}

pub static GLOBAL: LazyLock<Arc<RwLock<GlobalState>>> =
    LazyLock::new(|| Arc::new(RwLock::new(GlobalState::default())));
