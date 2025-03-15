use std::Collections::HashMap;
use crate::frame::Frame;

pub struct Host {
    pub incoming_packet_history: Vec<Frame>,
    pub outgoing_packet_history: Vec<Frame>,
    pub arp_table: HashMap<String, String>,
    pub ip_address: String,
    pub mac_address: String
}

impl Host {
    pub fn new(ip_address: String, mac_address: String) -> Self {
        Self {
            incoming_packet_history: Vec::new(),
            outgoing_packet_history: Vec::new(),
            arp_table: HashMap::new(),
            ip_address,
            mac_address
        }
    }

    pub fn send_arp_request(self, dest_ip: String) -> Frame {
        Frame::new(&self.mac_address, "ff:ff:ff:ff:ff:ff", &self.ip_address, &dest_ip, Vec::new(), true)
    }

    pub fn receive_arp_response(self, frame: Frame) {
        arp_table.insert(frame.src_ip, frame.src_mac);
    }

    // Destination MAC address will be for the next hop
    // Destination IP address will be for the end-to-end connection
    pub fn send_frame(self, dest_ip: String, dest_mac: String, data: Vec<u8>) -> Frame {
        let frame = Frame::new(&self.mac_address, &dest_mac, &self.ip_address, &dest_ip, data, false)
        outgoing_packet_history.push(frame);
        frame
    }
}