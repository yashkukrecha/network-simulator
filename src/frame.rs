#[derive(Debug, Clone)]
pub struct Frame {
    pub src_mac: String,
    pub dest_mac: String,
    pub src_ip: String,
    pub dest_ip: String,
    pub data: Vec<u8>,
    pub is_arp: bool,
}

impl Frame {
    pub fn new(src_mac: &str, dest_mac: &str, src_ip: &str, dest_ip: &str, data: Vec<u8>, is_arp: bool) -> Self {
        Self {
            src_mac: src_mac.to_string(),
            dest_mac: dest_mac.to_string(),
            src_ip: src_ip.to_string(),
            dest_ip: dest_ip.to_string(),
            data,
            is_arp,
        }
    }
}