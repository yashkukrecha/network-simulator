#[derive(Debug, Clone)]
struct Packet {
    src_mac: String,
    dest_mac: String,
    src_ip: String,
    dest_ip: String,
    data: Vec<u8>,
    is_arp: bool,
}

impl Packet {
    fn new(
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

    fn rebuild_L3(self, src_mac: String, dest_mac: String) -> Self {
        Self {
            src_mac,
            dest_mac,
            src_ip: self.src_ip,
            dest_ip: self.dest_ip,
            data: self.data,
            is_arp: self.is_arp,
        }
    }
}