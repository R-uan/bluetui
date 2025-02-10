use std::{
    io::{BufRead, BufReader, Write},
    process::{Command, Stdio},
    sync::Arc,
};

use crate::data::{
    device::Device,
    global_state::{GLOBAL_STATE, UPDATE_UI},
};

pub async fn scan_devices() {
    let global_variable = Arc::clone(&GLOBAL_STATE);
    let mut run_process = Command::new("bluetoothctl")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Bluetoothctl interactive mode wasn't initialized properly (⊙ _ ⊙ )");

    let mut stdin = run_process
        .stdin
        .take()
        .expect("Could not get bluetooth stdin pipeline (⊙ _ ⊙ )");

    stdin
        .write_all(b"scan on\n")
        .expect("Could not send command 'scan on' to bluetoothctl stdin pipeline (⊙ _ ⊙ )");

    let stdout = run_process
        .stdout
        .take()
        .expect("Could not get output from bluetoothctl 'scan on' command pipeline (⊙ _ ⊙ )");

    let stdout_reader = BufReader::new(stdout);
    let stdout_data = stdout_reader.lines();

    for line in stdout_data {
        if let Ok(line) = line {
            if line.contains("NEW") && line.contains("Device") == true {
                let start = line.find("Device").unwrap();
                let format = &line[start..line.len()];

                if let Some(device) = Device::new(format) {
                    let mut state = global_variable.write().unwrap();
                    state.scanned_devices.push(device);
                    let (sen, _) = &*UPDATE_UI;
                    let _ = sen.send(state.scanned_devices.len());
                }
            } else if line.contains("CHG") && line.contains("Device") {
                // let start = line.find("Device").unwrap();
                // let format = &line[start..line.len()];

                // if let Some(device) = Device::new(format) {
                //     let mut state = global_variable.write().unwrap();
                //     state.scanned_devices.push(device);
                //     let (sen, _) = &*UPDATE_UI;
                //     let _ = sen.send(state.scanned_devices.len());
                // }
            }
        }
    }
}
