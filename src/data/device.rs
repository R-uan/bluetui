use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
    sync::Arc,
};

use ratatui::text::{Text, ToText};

use crate::bluetooth::controller::extract_value;

use super::global_state::{CONNECTED_DEVICE, GLOBAL_STATE};

#[derive(Default, Clone)]
pub struct Device {
    pub name: String,
    pub mac_addr: String,
    pub paired: Option<bool>,
    pub trusted: Option<bool>,
    pub connected: Option<bool>,
}

impl ToText for Device {
    fn to_text(&self) -> Text<'_> {
        let value = format!("{} - {}", self.name, self.mac_addr);
        return Text::from(value);
    }
}

impl Device {
    pub fn add_to_scanned(input: &str) {
        if let Some(device) = Device::new(input) {
            let connected = device.connected.unwrap();
            let global = GLOBAL_STATE.write().unwrap();

            let mut scanned_devices = global.scanned_devices.write().unwrap();
            scanned_devices.push(device);

            if connected {
                *CONNECTED_DEVICE.write().unwrap() = Some(scanned_devices.len() - 1);
            }
        };
    }

    pub fn new(input: &str) -> Option<Self> {
        let info: Vec<&str> = input.splitn(3, " ").collect();
        if info.len() != 3 {
            return None;
        }

        let mac_addr = info[1];

        let mut run = Command::new("bluetoothctl")
            .args(&["info", mac_addr])
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let stdout = run.stdout.take().unwrap();
        let output = BufReader::new(stdout);

        let mut device = Device::default();
        device.mac_addr = mac_addr.to_owned();

        for line in output.lines() {
            if let Ok(line) = line {
                if line.contains("Paired") {
                    device.paired = Some(extract_value(&line) == "yes")
                } else if line.contains("Trusted") {
                    device.trusted = Some(extract_value(&line) == "yes")
                } else if line.contains("Connected") {
                    device.connected = Some(extract_value(&line) == "yes")
                } else if line.contains("Alias") {
                    device.name = extract_value(&line).to_owned();
                }
            }
        }

        return Some(device);
    }

    pub fn extract_mac(input: &str) -> Option<&str> {
        let info: Vec<&str> = input.splitn(3, " ").collect();
        if info.len() != 3 {
            return None;
        }
        return Some(info[1]);
    }

    pub fn update_device_info(&mut self, index: usize) {
        let mut run = Command::new("bluetoothctl")
            .args(&["info", &self.mac_addr])
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let stdout = run.stdout.take().unwrap();
        let output = BufReader::new(stdout);

        for line in output.lines() {
            if let Ok(line) = line {
                if line.contains("Paired") {
                    self.paired = Some(extract_value(&line) == "yes")
                } else if line.contains("Trusted") {
                    self.trusted = Some(extract_value(&line) == "yes")
                } else if line.contains("Connected") {
                    let value = extract_value(&line) == "yes";
                    self.connected = Some(value);
                    if value {
                        *CONNECTED_DEVICE.write().unwrap() = Some(index);
                    }
                } else if line.contains("Alias") {
                    self.name = extract_value(&line).to_owned();
                }
            }
        }
    }
}
