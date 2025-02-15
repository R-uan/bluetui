pub struct Device {
    pub name: String,
    pub mac_addr: String,
}

impl Device {
    pub fn new(input: &str) -> Self {
        let split: Vec<&str> = input.splitn(2, " ").collect();
        return Device {
            name: split[1].to_owned(),
            mac_addr: split[0].to_owned(),
        };
    }
}
