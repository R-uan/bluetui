use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

use crate::bluetooth::controller::extract_value;

pub struct Device {
    pub name: String,
    pub mac_addr: String,
    pub status: DeviceStatus,
}

#[derive(Default)]
pub struct DeviceStatus {
    pub paired: Option<bool>,
    pub trusted: Option<bool>,
    pub connected: Option<bool>,
}

impl Device {
    ///
    /// Receives raw string directly from the bluetoothctl CLI
    pub fn new(input: &str) -> Option<Self> {
        let info: Vec<&str> = input.splitn(3, " ").collect();
        if info.len() != 3 {
            return None;
        }

        let status = get_device_info(info[1]);
        return Some(Device {
            name: info[2].to_owned(),
            mac_addr: info[1].to_owned(),
            status,
        });
    }

    pub fn pair(&self) {
        todo!()
    }
}

pub fn get_device_info(mac_addr: &str) -> DeviceStatus {
    let mut run = Command::new("bluetoothctl")
        .args(&["info", mac_addr])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let stdout = run.stdout.take().unwrap();
    let output = BufReader::new(stdout);

    let mut status = DeviceStatus::default();

    for line in output.lines() {
        if let Ok(line) = line {
            if line.contains("Paired") {
                status.paired = Some(extract_value(&line) == "yes")
            } else if line.contains("Trusted") {
                status.trusted = Some(extract_value(&line) == "yes")
            } else if line.contains("Connected") {
                status.connected = Some(extract_value(&line) == "yes")
            }
        }
    }

    return status;
}
