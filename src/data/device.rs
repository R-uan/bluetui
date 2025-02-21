use std::{
    io::{BufRead, BufReader},
    process::Command,
};

use crate::utils::extract_value;

pub struct Device {
    pub name: String,
    pub mac_addr: String,
    pub status: Option<DeviceStatus>,
}

#[derive(Default)]
pub struct DeviceStatus {
    pub connected: bool,
    pub paired: bool,
    pub trusted: bool,
}

impl ToString for Device {
    fn to_string(&self) -> String {
        let string = format!("Name: {} | MAC: {}", &self.name, &self.mac_addr);
        return string;
    }
}

impl Device {
    pub fn new(input: &str) -> Self {
        let split: Vec<&str> = input.splitn(2, " ").collect();
        return Device {
            name: split[1].to_owned(),
            mac_addr: split[0].to_owned(),
            status: None,
        };
    }

    pub fn collect_status(&mut self) {
        let command = Command::new("bluetoothctl")
            .args(&["info", &self.mac_addr])
            .output()
            .expect("Could not get info");

        let lines_buffer = BufReader::new(&command.stdout[..]);
        let mut device_status = DeviceStatus::default();
        lines_buffer
            .lines()
            .filter_map(Result::ok)
            .for_each(|line| {
                if line.contains("Name") {
                    let name = extract_value(&line).to_owned();
                    self.name = name;
                } else if line.contains("Paired") {
                    device_status.paired = match extract_value(&line) {
                        "yes" => true,
                        "no" => false,
                        _ => false,
                    };
                } else if line.contains("Connected") {
                    device_status.connected = match extract_value(&line) {
                        "yes" => true,
                        "no" => false,
                        _ => false,
                    }
                } else if line.contains("Trusted") {
                    device_status.trusted = match extract_value(&line) {
                        "yes" => true,
                        "no" => false,
                        _ => false,
                    };
                }
            });

        self.status = Some(device_status);
    }
}
