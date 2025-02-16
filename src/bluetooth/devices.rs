use std::{
    io::{BufRead, BufReader, Write},
    process::{Command, Stdio},
};

use regex::Regex;

use crate::data::{device::Device, global_state::GLOBAL};

pub fn scan_devices() {
    let mut command = Command::new("bluetoothctl")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Unable to run bluetoothctl");

    let mut stdin = command.stdin.take().expect("Unable to get stdin");
    stdin
        .write_all(b"scan on\n")
        .expect("Unable to write scan on");

    let stdout = command.stdout.take().expect("Unable to get stdout");
    let line_buffer = BufReader::new(stdout);

    line_buffer.lines().filter_map(Result::ok).for_each(|line| {
        if line.contains("Device") {
            let sanitized_line = sanitize_device_line(&line);
            if line.contains("[0;93mCHG") {
            } else if line.contains("[0;92mNEW") {
                let sanitized_line = sanitize_device_line(&line);
                let mut device = Device::new(&sanitized_line);
                device.collect_status();
                GLOBAL.write().unwrap().devices.push(device);
            } else if line.contains("[0;91mDEL") {
                let device = Device::new(&sanitized_line);
                GLOBAL
                    .write()
                    .unwrap()
                    .devices
                    .retain(|d| d.mac_addr != device.mac_addr);
            }
        }
    });
}

fn sanitize_device_line(line: &str) -> String {
    let regex = Regex::new(r"^.*Device ").unwrap();
    return regex.replace(&line, "").to_string();
}

pub fn known_devices() {
    let command = Command::new("bluetoothctl")
        .arg("devices")
        .stdout(Stdio::piped())
        .output()
        .expect("Could not get devices");

    let lines_buffer = BufReader::new(&command.stdout[..]);
    let devices: Vec<Device> = lines_buffer
        .lines()
        .filter_map(Result::ok)
        .filter_map(|line| {
            if line.contains("Device") {
                let sanitized_line = sanitize_device_line(&line);
                let mut device = Device::new(&sanitized_line);
                device.collect_status();
                return Some(device);
            } else {
                None
            }
        })
        .collect();

    GLOBAL.write().unwrap().devices.extend(devices);
}
