use std::collections::HashMap;
use std::rc::Rc;
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
    fn new(ip_address: String, mac_address: String, port: usize, switch: Rc<Switch>) -> Self {
        Self {
            arp_table: HashMap::new(),
            routing_table: HashMap::new(),
            incoming_packets: Vec::new(),
            outgoing_packets: Vec::new(),
            ip_address,
            mac_address,
            port,
            switch,
        }
    }

    fn arp_request(&mut self, dest_ip: &str) -> String {
        let request = Packet::new(
            &self.mac_address,
            "UNKNOWN",
            &self.ip_address,
            dest_ip,
            Vec::new(),
            true
        );

        let response = self.switch.process_arp_request(Rc::new(request), self.port);
        self.arp_table.insert(dest_ip.to_string(), response.src_mac.clone());
        response.src_mac.clone()
    }

    fn send_packet(&mut self, dest_ip: &str, data: Vec<u8>) {
        // Check if they are in the same subnet, otherwise find the router that can forward the packet
        let hop_dest_ip = if self.ip_address.get(..9) == dest_ip.get(..9) {
            dest_ip.to_string()
        } else {
            let mut found = None;
            for (router_ip, networks) in &self.routing_table {
                if networks.iter().any(|net| dest_ip.get(..9) == net.get(..9)) {
                    found = Some(router_ip.clone());
                    break;
                }
            }
            if let Some(router_ip) = found {
                router_ip
            } else {
                println!("No route to {}", dest_ip);
                return;
            }
        };

        // Check the ARP table if the destination MAC address exists, else send an ARP request
        let hop_dest_mac = match self.arp_table.get(&hop_dest_ip) {
            Some(mac) => mac.clone(),
            None => self.arp_request(&hop_dest_ip),
        };

        let packet = Rc::new(Packet::new(
            &self.mac_address,
            &hop_dest_mac,
            &self.ip_address,
            dest_ip,
            data,
            false
        ));

        // Need to clone so that you maintain ownership of packet
        self.outgoing_packets.push(Rc::clone(&packet));
        let response = self.switch.process_packet(Rc::clone(&packet), self.port);
        self.incoming_packets.push(response);
    }
}