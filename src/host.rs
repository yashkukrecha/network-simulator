use std::Collections::HashMap;
use std::rc::Rc; // reference counting
use crate::packet::Packet;

#[derive(Debug)]
struct Host {
    arp_table: HashMap<String, String>, // IP address -> MAC address
    routing_table: HashMap<String, Vec<String>>, // Router IP address -> list of destination networks
    incoming_packets: Vec<Rc<Packet>>,
    outgoing_packets: Vec<Rc<Packet>>,
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

    fn send_arp_request(self, dest_ip: String) -> Packet {
        Packet::new(&self.mac_address, "ff:ff:ff:ff:ff:ff", &self.ip_address, &dest_ip, Vec::new(), true)
    }

    fn receive_arp_response(self, packet: Packet) {
        arp_table.insert(packet.src_ip, packet.src_mac);
    }

    // Destination MAC address will be for the next hop
    // Destination IP address will be for the end-to-end connection
    fn send_packet(self, dest_ip: String, dest_mac: String, data: Vec<u8>) -> Packet {
        let packet = Packet::new(&self.mac_address, &dest_mac, &self.ip_address, &dest_ip, data, false)
        outgoing_packet_history.push(packet);
        packet
    }
}