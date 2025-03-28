use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use crate::packet::Packet;
use crate::switch::Switch;

#[derive(Debug)]
pub struct Router {
    arp_table: HashMap<String, String>, // IP address -> MAC address
    // Network address -> (switch, port, IP address of next hop, MAC address of the interface)
    routing_table: HashMap<String, (Weak<RefCell<Switch>>, usize, String, String)>, 
    incoming_packets: Vec<Rc<Packet>>,
    outgoing_packets: Vec<Rc<Packet>>,
    ip_address: String,
}

/*
Example Routing Tables:
If string is empty, then the next hop is the final destination. 
Otherwise, the next hop is the IP address of the next router.

R1:
{
    "192.168.1.0": (switch1, 1, "", MAC1/1),
    "192.168.2.0": (switch2, 1, "", MAC2/1),
    "192.168.3.0": (switch_main, 1, R2, MAC_MAIN/1),
    "192.168.4.0": (switch_main, 1, R2, MAC_MAIN/1),
    "192.168.5.0": (switch_main, 1, R3, MAC_MAIN/1),
}

R2:
{
    "192.168.1.0": (switch_main, 2, R1, MAC_MAIN/2),
    "192.168.2.0": (switch2, 2, "", MAC2/2),
    "192.168.3.0": (switch3, 1, "", MAC3/2),
    "192.168.4.0": (switch4, 1, "", MAC4/2),
    "192.168.5.0": (switch_main, 2, R3, MAC_MAIN/2),
}

R3:
{
    "192.168.1.0": (switch_main, 3, R1, MAC_MAIN/3),
    "192.168.2.0": (switch_main, 3, R2, MAC_MAIN/3),
    "192.168.3.0": (switch_main, 3, R2, MAC_MAIN/3),
    "192.168.4.0": (switch_main, 3, R2, MAC_MAIN/3),
    "192.168.5.0": (switch5, 1, "", MAC5/3),
}
*/

impl Router {
    pub fn new(ip_address: String) -> Self {
        Self {
            arp_table: HashMap::new(),
            routing_table: HashMap::new(),
            incoming_packets: Vec::new(),
            outgoing_packets: Vec::new(),
            ip_address,
        }
    }

    pub fn populate_routing_table(
        &mut self,
        network: String,
        switch: Weak<RefCell<Switch>>,
        port: usize,
        router_ip: String, // "" if directly connected
        local_mac: String
    ) {
        self.routing_table.insert(network, (switch, port, router_ip, local_mac));
    }

    pub fn get_ip_address(&self) -> String { self.ip_address.clone() }

    // Returns an Option<String> that contains the MAC address if successful.
    pub fn send_arp_request(&mut self, dest_ip: &str, src_mac: &str, switch_ref: Weak<RefCell<Switch>>, port: usize) -> Option<String> {
        println!("==============================================");
        println!("ROUTER: {}", self.ip_address);
        println!("Sending ARP request for {}", dest_ip);
        println!("==============================================\n");
        let request = Packet::new(
            src_mac,
            "UNKNOWN",
            &self.ip_address,
            dest_ip,
            Vec::new(),
            true
        );

        let switch_rc = switch_ref.upgrade();
        if switch_rc.is_none() {
            println!("Switch not available");
            return None;
        }
        let binding = switch_rc.unwrap();
        let mut switch = binding.borrow_mut();

        let response = switch.process_arp_request(Rc::new(request), port);
        if let Some(ref resp) = response {
            self.arp_table.insert(dest_ip.to_string(), resp.src_mac.clone());
            Some(resp.src_mac.clone())
        } else {
            None
        }
    }

    pub fn receive_arp_request(&mut self, packet: Rc<Packet>) -> Option<Rc<Packet>> {
        // Only process ARP request if it is intended for this router
        if packet.dest_ip != self.ip_address {
            return None;
        }

        // Find the local MAC address based on the packet's source IP address
        let mut local_mac : Option<String> = None;
        for (network, (_, _, _, loc_mac)) in &self.routing_table {
            if packet.src_ip.get(..9) == network.get(..9) {
                local_mac = Some(loc_mac.clone());
                break;
            }
        }

        // If the ARP request is intended for this host, return the MAC value
        if let Some(mac) = local_mac {
            self.arp_table.insert(packet.src_ip.clone(), packet.src_mac.clone());
            println!("==============================================");
            println!("ROUTER: {}", self.ip_address);
            println!("Received ARP request from {}", packet.src_ip);
            println!("ARP Table: {:#?}", self.arp_table);
            println!("==============================================\n");
            return Some(Rc::new(Packet::new(
                &mac,
                &packet.src_mac,
                &packet.dest_ip,
                &packet.src_ip,
                Vec::new(),
                true
            )));
        }
        None
    }

    pub fn forward_packet(&mut self, request: Rc<Packet>) -> Option<Rc<Packet>> {
        // Make sure the packet is intended for this router
        if !self.routing_table.values().any(|(_, _, _, local_mac)| local_mac == &request.dest_mac) {
            return None;
        }

        // Add to list of incoming packets
        self.incoming_packets.push(Rc::clone(&request));
        println!("==============================================");
        println!("ROUTER: {}", self.ip_address);
        println!("Received packet: {request:#?}");
        println!("==============================================\n");

        // Get (next_hop_ip, corresponding_switch, port, MAC)
        let mut hop_info: Option<(String, Weak<RefCell<Switch>>, usize, String)> = None;
        for (network, (sw, port, router_ip, local_mac)) in &self.routing_table {
            if request.dest_ip.get(..9) == network.get(..9) {
                if router_ip == "" {
                    hop_info = Some((request.dest_ip.clone(), sw.clone(), *port, local_mac.clone()));
                } else {
                    hop_info = Some((router_ip.clone(), sw.clone(), *port, local_mac.clone()));
                }
                break;
            }
        }

        let (hop_ip, hop_switch, hop_port, local_mac) = match hop_info {
            Some(info) => info,
            None => {
                println!("No route to {}", request.dest_ip);
                return None;
            }
        };

        // Obtain next hop's MAC address using the provided switch and port
        let hop_dest_mac = match self.arp_table.get(&hop_ip) {
            Some(mac) => mac.clone(),
            None => match self.send_arp_request(&hop_ip, &local_mac, hop_switch.clone(), hop_port) {
                Some(mac) => mac,
                None => {
                    println!("No route to {}", request.dest_ip);
                    return None;
                }
            },
        };

        // Rebuild the packet with updated L3 headers so that the correct switch processes it
        let modified_packet = request.rebuild_L3(local_mac, hop_dest_mac);
        let modified_packet = Rc::new(modified_packet);

        // Use the switch reference from the routing table entry
        let switch_rc = hop_switch.upgrade();
        if switch_rc.is_none() {
            println!("Switch not available");
            return None;
        }
        let binding = switch_rc.unwrap();
        let mut switch = binding.borrow_mut();

        // Add to outgoing packets and send the packet through the correct port
        self.outgoing_packets.push(Rc::clone(&modified_packet));
        println!("==============================================");
        println!("ROUTER: {}", self.ip_address);
        println!("Forwarding packet for {}", request.dest_ip);
        println!("==============================================\n");
        if let Some(response) = switch.process_packet(Rc::clone(&modified_packet), hop_port) {
            self.incoming_packets.push(Rc::clone(&response));
            println!("==============================================");
            println!("ROUTER: {}", self.ip_address);
            println!("Forwarding packet for {}", response.dest_ip);
            println!("==============================================\n");

            // make sure to replace the source MAC and destination MAC
            let modified_response = response.rebuild_L3(request.dest_mac.clone(), request.src_mac.clone());
            let modified_response = Rc::new(modified_response);
            self.outgoing_packets.push(Rc::clone(&modified_response));
            return Some(modified_response);
        }
        None
    }

    pub fn get_router_info(&self) -> String {
        format!(
            "======================================\nROUTER: {}\nARP Table: {:#?}\nOutgoing Packets: {:#?}\nIncoming Packets: {:#?}\n======================================\n",
            self.ip_address, self.arp_table, self.outgoing_packets, self.incoming_packets
        )
    }
}