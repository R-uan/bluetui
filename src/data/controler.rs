#[derive(Default)]
pub struct ControllerInfo {
    pub name: Option<String>,
    pub powered: bool,
    pub pairable: bool,
    pub discoverable: bool,
    pub power_state: String,
}
