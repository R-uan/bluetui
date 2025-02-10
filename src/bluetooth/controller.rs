use core::str;
use std::{process::Stdio, sync::Arc};

use crate::data::global_state::GLOBAL_STATE;

pub async fn controller_info() {
    let command = tokio::process::Command::new("bluetoothctl")
        .arg("show")
        .stdout(Stdio::piped())
        .output()
        .await
        .unwrap();

    let stdout = str::from_utf8(&command.stdout).unwrap();
    let output_vec: Vec<&str> = stdout.split("\n").collect();

    let global = Arc::clone(&GLOBAL_STATE);
    output_vec.iter().for_each(|f| {
        let mut state = global.write().unwrap();
        if f.contains("Powered") {
            state.controller_info.powered = match extract_value(f) {
                "yes" => true,
                "no" => false,
                _ => false,
            };
        } else if f.contains("PowerState") {
            state.controller_info.power_state = extract_value(f).to_owned();
        } else if f.contains("Pairable") {
            state.controller_info.pairable = match extract_value(f) {
                "yes" => true,
                "no" => false,
                _ => false,
            }
        } else if f.contains("Discoverable") {
            state.controller_info.discoverable = match extract_value(f) {
                "yes" => true,
                "no" => false,
                _ => false,
            }
        } else if f.contains("Name") {
            state.controller_info.name = Some(extract_value(f).to_owned());
        }
    });
}

pub fn extract_value(string: &str) -> &str {
    let split: Vec<&str> = string.splitn(2, ":").collect();
    return split[1];
}
