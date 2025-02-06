use core::str;
use std::{
    process::{self, Stdio},
    sync::Arc,
};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    sync::mpsc,
};

use crate::data::{
    devices::Devices,
    global_state::{GLOBAL_STATE, UPDATE_UI},
};

pub struct BluetoothService {
    sender: mpsc::Sender<String>,
}

impl BluetoothService {
    pub fn new() -> Self {
        let (tx, mut rx) = mpsc::channel::<String>(5);
        tokio::spawn(async move {
            loop {
                match rx.recv().await {
                    Some(command) => {
                        let args = command.split(" ");
                        let out = process::Command::new("bluetoothctl")
                            .args(args)
                            .output()
                            .expect("Cry");
                        let output = str::from_utf8(&out.stdout).expect("msg");

                        let test: Vec<&str> = output.split("\n").collect();
                        println!("output {:}", test[0]);
                    }
                    _ => {}
                };
            }
        });

        Self { sender: tx }
    }

    pub async fn fetch_devices(&self) {
        let paired_devices = process::Command::new("bluetoothctl")
            .arg("devices")
            .output()
            .expect("Unable to read known devices");

        let global = Arc::clone(&GLOBAL_STATE);
        let mut state = global.write().unwrap();

        let found_devices: Vec<&str> = str::from_utf8(&paired_devices.stdout)
            .expect("msg")
            .split("\n")
            .collect();

        let devices: Vec<Devices> = found_devices
            .iter()
            .filter_map(|s| Devices::new(s))
            .collect();

        state.paired_devices = devices;
    }

    pub async fn exec(command: &str) {
        let args: Vec<&str> = command.split(" ").collect();
        todo!()
    }

    pub async fn send_command(&self, command: String) {
        self.sender.send(command).await.unwrap();
    }
}

pub async fn scan_devices() {
    let global = Arc::clone(&GLOBAL_STATE);

    {
        let mut state = global.write().unwrap();
        state.scanning = true;
    }

    let mut process = tokio::process::Command::new("bluetoothctl")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Bluetoothctl interactive mode wasn't initialized properly (⊙ _ ⊙ )");

    let mut stdin = process
        .stdin
        .take()
        .expect("Could not get bluetooth stdin pipeline (⊙ _ ⊙ )");

    stdin
        .write_all(b"scan on\n")
        .await
        .expect("Could not send scan command through bluetoothctl stdin pipeline (⊙ _ ⊙ )");

    let stdout = process.stdout.take().expect("Could not get stdout");
    let reader = tokio::io::BufReader::new(stdout);
    let mut lines = reader.lines();

    loop {
        if let Some(line) = lines.next_line().await.expect("Cry in lines") {
            if line.contains("NEW") && line.contains("Device") == true {
                let start = line.find("Device").unwrap();
                let format = &line[start..line.len()];

                if let Some(device) = Devices::new(format) {
                    let mut state = global.write().unwrap();
                    state.scanned_devices.push(device);
                    let (sen, _) = &*UPDATE_UI;
                    let _ = sen.send(state.scanned_devices.len());
                    // println!("{:}", state.scanned_devices.len());
                }
            } else if line.contains("CHG") && line.contains("Device") {
                let start = line.find("Device").unwrap();
                let format = &line[start..line.len()];
                if let Some(device) = Devices::new(format) {
                    let mut state = global.write().unwrap();
                    state.scanned_devices.push(device);
                    let (sen, _) = &*UPDATE_UI;
                    let _ = sen.send(state.scanned_devices.len());
                }
            }
        }
        // println!("> > > > {line}");
    }
}
