#[derive(Debug, Clone)]
struct Frame<'a> {
    src_mac: &'a String,
    dest_mac: &'a String,
    src_ip: &'a String,
    dest_ip: &'a String,
    data: Vec<u8>,
    is_arp: bool,
}

impl<'a> Frame<'a> {

    // We do not want the frame to own the addresses
    // We want the frame to own the data
    fn new(
        src_mac: &'a String, 
        dest_mac: &'a String, 
        src_ip: &'a String, 
        dest_ip: &'a String, 
        data: Vec<u8>, 
        is_arp: bool) -> Self {
        Self {
            src_mac: src_mac,
            dest_mac: dest_mac,
            src_ip: src_ip,
            dest_ip: dest_ip,
            data,
            is_arp,
        }
    }

    fn rebuild_L3(self, src_mac: &'a String, dest_mac: &'a String) -> Self {
        Self {
            src_mac,
            dest_mac,
            src_ip: self.src_ip,
            dest_ip: self.dest_ip,
            data: self.data,
            is_arp: self.is_arp
        }
    }
}