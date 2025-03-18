use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use crate::packet::Packet;
use crate::switch::Switch;
use crate::device::Device;

#[derive(Debug)]
pub struct Host {
    arp_table: HashMap<String, String>, // IP address -> MAC address
    routing_table: HashMap<String, Vec<String>>, // Router IP address -> list of destination networks
    incoming_packets: Vec<Rc<Packet>>,
    outgoing_packets: Vec<Rc<Packet>>,
    ip_address: String,
    mac_address: String,
    port: usize,
    switch: Weak<RefCell<Switch>>,
}

impl Host {
    pub fn new(ip_address: String, mac_address: String, port: usize, switch: Weak<RefCell<Switch>>) -> Self {
        Self {
            arp_table: HashMap::new(),
            routing_table: HashMap::new(),
            incoming_packets: Vec::new(),
            outgoing_packets: Vec::new(),
            ip_address,
            mac_address,
            port, // Temporary port, will be assigned later
            switch,
        }
    }

    pub fn populate_routing_table(&mut self, router_ip: String, network: String) {
        self.routing_table.entry(router_ip).or_insert(Vec::new()).push(network);
    }

    // Returns an Option<String> that contains the MAC address if successful.
    pub fn send_arp_request(&mut self, dest_ip: &str) -> Option<String> {
        let request = Packet::new(
            &self.mac_address,
            "UNKNOWN",
            &self.ip_address,
            dest_ip,
            Vec::new(),
            true
        );

        let switch_rc = self.switch.upgrade();
        if switch_rc.is_none() {
            println!("Switch not available");
            return None;
        }
        let mut switch = switch_rc.unwrap().borrow_mut();

        let response = switch.process_arp_request(Rc::new(request), self.port);
        if let Some(ref resp) = response {
            self.arp_table.insert(dest_ip.to_string(), resp.src_mac.clone());
            Some(resp.src_mac.clone())
        } else {
            None
        }
    }

    pub fn send_packet(&mut self, dest_ip: &str, data: Vec<u8>) {
        // Check if they are in the same subnet, else find the router that can forward it
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
            None => match self.send_arp_request(&hop_dest_ip) {
                Some(mac) => mac,
                None => {
                    println!("No route to {}", dest_ip);
                    return;
                }
            },
        };

        let request = Rc::new(Packet::new(
            &self.mac_address,
            &hop_dest_mac,
            &self.ip_address,
            dest_ip,
            data,
            false
        ));

        let switch_rc = self.switch.upgrade();
        if switch_rc.is_none() {
            println!("Switch not available");
            return;
        }
        let mut switch = switch_rc.unwrap().borrow_mut();

        // Clone so that we maintain ownership of the packet
        self.outgoing_packets.push(Rc::clone(&request));
        if let Some(response) = switch.process_packet(Rc::clone(&request), self.port) {
            self.incoming_packets.push(response);
        }
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

    pub fn receive_packet(&mut self, request: Rc<Packet>) -> Option<Rc<Packet>> {
        // Make sure the packet is intended for this host
        if request.dest_ip != self.ip_address {
            return None;
        }

        // Add to list of incoming packets
        self.incoming_packets.push(Rc::clone(&request));

        // Determine next hop for response using the same logic
        let hop_dest_ip = if self.ip_address.get(..9) == request.src_ip.get(..9) {
            Some(request.src_ip.to_string())
        } else {
            let mut found = None;
            for (router_ip, networks) in &self.routing_table {
                if networks.iter().any(|net| request.src_ip.get(..9) == net.get(..9)) {
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
                        println!("No route to {}", request.src_ip);
                        return None;
                    }
                },
            },
            None => {
                println!("No route to {}", request.src_ip);
                return None;
            }
        };

        let response = Rc::new(Packet::new(
            &self.mac_address,
            &hop_dest_mac,
            &self.ip_address,
            &request.src_ip,
            Vec::new(),
            false
        ));

        // Clone so that we maintain ownership of the packet
        self.outgoing_packets.push(Rc::clone(&response));
        Some(response)
    }
}
