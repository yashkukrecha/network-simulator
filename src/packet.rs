#[derive(Debug, Clone)]
pub struct Packet {
    pub src_mac: String,
    pub dest_mac: String,
    pub src_ip: String,
    pub dest_ip: String,
    pub data: Vec<u8>,
    pub is_arp: bool,
}

impl Packet {
    pub fn new(
        src_mac: &str,
        dest_mac: &str,
        src_ip: &str,
        dest_ip: &str,
        data: Vec<u8>,
        is_arp: bool
    ) -> Self {
        Self {
            src_mac: src_mac.to_string(),
            dest_mac: dest_mac.to_string(),
            src_ip: src_ip.to_string(),
            dest_ip: dest_ip.to_string(),
            data,
            is_arp,
        }
    }

    pub fn rebuild_L3(&mut self, src_mac: String, dest_mac: String) -> Self {
        Self {
            src_mac,
            dest_mac,
            src_ip: self.src_ip.clone(),
            dest_ip: self.dest_ip.clone(),
            data: self.data.clone(),
            is_arp: self.is_arp.clone(),
        }
    }
}
