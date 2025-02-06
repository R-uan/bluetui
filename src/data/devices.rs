pub struct Devices {
    pub name: String,
    pub mac_addr: String,
}

impl Devices {
    pub fn new(input: &str) -> Option<Self> {
        let info: Vec<&str> = input.splitn(3, " ").collect();
        if info.len() != 3 {
            return None;
        }

        return Some(Devices {
            mac_addr: info[1].to_owned(),
            name: info[2].to_owned(),
        });
    }

    pub fn pair(&self) {
        todo!()
    }
}
