use std::Collections::HashMap;
use std::rc::Rc; // reference counting
use crate::frame::Frame;

#[derive(Debug)]
struct Host {
    arp_table: HashMap<String, String>, // IP address -> MAC address
    routing_table: HashMap<String, Vec<String>>, // Router IP address -> list of destination networks
    incoming_frames: Vec<Rc<Frame>>,
    outgoing_frames: Vec<Rc<Frame>>,
    ip_address: String,
    mac_address: String,
    port: usize,
    switch: Rc<Switch>
}

impl Host {
    fn new(ip_address: String, mac_address: String, port: usize, switch: &Switch) -> Self {
        Self {
            arp_table: HashMap::new(),
            routing_table: HashMap::new(),
            incoming_packet_history: Vec::new(),
            outgoing_packet_history: Vec::new(),
            ip_address,
            mac_address,
            port
            switch
        }
    }

    fn send_arp_request(self, dest_ip: String) -> Frame {
        Frame::new(&self.mac_address, "ff:ff:ff:ff:ff:ff", &self.ip_address, &dest_ip, Vec::new(), true)
    }

    fn receive_arp_response(self, frame: Frame) {
        arp_table.insert(frame.src_ip, frame.src_mac);
    }

    // Destination MAC address will be for the next hop
    // Destination IP address will be for the end-to-end connection
    fn send_frame(self, dest_ip: String, dest_mac: String, data: Vec<u8>) -> Frame {
        let frame = Frame::new(&self.mac_address, &dest_mac, &self.ip_address, &dest_ip, data, false)
        outgoing_packet_history.push(frame);
        frame
    }
}