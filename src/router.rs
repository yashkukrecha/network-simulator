use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use crate::packet::Packet;
use crate::switch::Switch;
use crate::device::Device;

#[derive(Debug)]
struct Router {
    arp_table: HashMap<String, String>, // IP address -> MAC address
    routing_table: HashMap<String, (Weak<RefCell<Switch>>, usize)>, // Network address -> (switch, port)
    mac_addresses: Vec<String>,
    incoming_packets: Vec<Rc<Packet>>,
    outgoing_packets: Vec<Rc<Packet>>,
    ip_address: String,
}

impl Router {
    fn new(ip_address: String, mac_address: String) -> Self {
        Self {
            arp_table: HashMap::new(),
            routing_table: HashMap::new(),
            mac_addresses: Vec::new(),
            incoming_packets: Vec::new(),
            outgoing_packets: Vec::new(),
            ip_address,
        }
    }

    fn populate_routing_table(&mut self, network: String, mac_address: String, switch: Weak<RefCell<Switch>>, port: usize) {
        self.routing_table.entry(network).insert((switch, port));
        self.mac_addresses.push(mac_address);
    }

    pub fn receive_arp_request(&mut self, packet: Rc<Packet>) -> Option<Rc<Packet>> {
        // If the ARP request is intended for this host, return the MAC value
        if packet.dest_ip == self.ip_address {
            self.arp_table.insert(packet.src_ip.clone(), packet.src_mac.clone());
            return Some(Rc::new(Packet::new(
                &self.mac_address,
                &packet.src_mac,
                &self.ip_address,
                &packet.src_ip,
                Vec::new(),
                true,
            )));
        }
        None
    }

    // TODO: this is a mess
    // Need to figure out routing table
    // Need to figure out how to forward the packet to the next router
    pub fn forward_packet(&mut self, request: Rc<Packet>) -> Option<Rc<Packet>> {
        // Make sure the packet is intended for this router
        if self.mac_addresses.iter().all(|mac| mac != &request.dest_mac) {
            return None;
        }

        // Add to list of incoming packets
        self.incoming_packets.push(Rc::clone(&request));

        // TODO: Determine next hop for response using the same logic
        let hop_dest_ip = if self.ip_address.get(..9) == request.dest_ip.get(..9) {
            Some(request.dest_ip.to_string())
        } else {
            let mut found = None;
            for (networks, (_switch, _port)) in &self.routing_table {
                if networks.iter().any(|net| request.dest_ip.get(..9) == net.get(..9)) {
                    found = Some(router_ip.clone());
                    break;
                }
            }
            found
        };

        let hop_dest_mac = match hop_dest_ip {
            Some(ip) => match self.arp_table.get(&ip) {
                Some(mac) => mac.clone(),
                None => match self.send_arp_request(&ip) {
                    Some(mac) => mac,
                    None => {
                        println!("No route to {}", request.dest_ip);
                        return None;
                    }
                },
            },
            None => {
                println!("No route to {}", request.dest_ip);
                return None;
            }
        };

        // TODO
        let request2 = Rc::new(Packet::new(
            &self.mac_address,
            &hop_dest_mac,
            &self.ip_address,
            &request.src_ip,
            Vec::new(),
            false
        ));

        let switch_rc = self.switch.upgrade();
        if switch_rc.is_none() {
            println!("Switch not available");
            return;
        }
        let mut switch = switch_rc.unwrap().borrow_mut();

        // Clone so that we maintain ownership of the packet
        self.outgoing_packets.push(Rc::clone(&request2));
        if let Some(response) = switch.process_packet(Rc::clone(&request2), self.port) {
            self.incoming_packets.push(response);
        }
    }
}