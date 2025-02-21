use std::{
    io::{BufRead, BufReader},
    process::Command,
};

use crate::utils::extract_value;

#[derive(Default)]
pub struct ControllerInfo {
    pub mac_addr: String,
    pub name: String,
    pub powered: bool,
    pub discoverable: bool,
    pub pairable: bool,
}

impl ControllerInfo {
    pub fn init() -> Self {
        let command = Command::new("bluetoothctl")
            .arg("show")
            .output()
            .expect("Couldn't get controller info");

        let output_buffer = BufReader::new(&command.stdout[..]);

        let mut controller = ControllerInfo::default();

        output_buffer
            .lines()
            .filter_map(Result::ok)
            .for_each(|line| {
                if line.contains("Controller ") {
                    let split: Vec<&str> = line.splitn(3, " ").collect();
                    controller.mac_addr = split[1].to_owned();
                } else if line.contains("Name") {
                    let value = extract_value(&line);
                    controller.name = value.to_owned();
                } else if line.contains("Powered") {
                    controller.powered = match extract_value(&line) {
                        "yes" => true,
                        "no" => false,
                        _ => false,
                    }
                } else if line.contains("Discoverable") {
                    controller.discoverable = match extract_value(&line) {
                        "yes" => true,
                        "no" => false,
                        _ => false,
                    }
                } else if line.contains("Pairable") {
                    controller.pairable = match extract_value(&line) {
                        "yes" => true,
                        "no" => false,
                        _ => false,
                    }
                }
            });

        // GLOBAL.write().unwrap().controller = controller;
        return controller;
    }
}
