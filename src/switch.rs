use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::packet::Packet;
use crate::device::Device;

#[derive(Debug)]
pub struct Switch {
    mac_table: HashMap<String, usize>, // MAC -> port
    ports: Vec<Option<Rc<RefCell<dyn Device>>>>,
    packets: Vec<Rc<Packet>>, // Will be used in multithreading
}

impl Switch {
    pub fn new(port_count: usize) -> Self {
        Self {
            mac_table: HashMap::new(),
            ports: vec![None; port_count],
            packets: Vec::new(),
        }
    }

    pub fn add_device(&mut self, device: Rc<RefCell<dyn Device>>) -> Option<usize> {
        for (i, port) in self.ports.iter_mut().enumerate() {
            if port.is_none() {
                *port = Some(device);
                return Some(i);
            }
        }
        None
    }

    pub fn process_arp_request(&mut self, packet: Rc<Packet>, port: usize) -> Option<Rc<Packet>> {
        // Add to MAC table
        self.mac_table.entry(packet.src_mac.clone()).or_insert(port);

        // For all other occupied ports except the one that sent the request, send the request
        for (i, device) in self.ports.iter().enumerate() {
            if let Some(dev) = device {
                if i != port {
                    if let Some(response) = dev.borrow_mut().receive_arp_request(Rc::clone(&packet)) {
                        return Some(response);
                    }
                }
            }
        }

        // None of them worked
        None
    }

    pub fn process_packet(&mut self, packet: Rc<Packet>, port: usize) -> Option<Rc<Packet>> {
        // Add to MAC table
        self.mac_table.entry(packet.src_mac.clone()).or_insert(port);

        // Check if destination is in MAC table
        if let Some(&target_port) = self.mac_table.get(&packet.dest_mac) {
            if let Some(device) = &self.ports[target_port] {
                return device.borrow_mut().receive_packet(Rc::clone(&packet));
            }
        } else {
            // Flood to all ports except the incoming one
            for (i, device) in self.ports.iter().enumerate() {
                if i != port {
                    if let Some(dev) = device {
                        if let Some(response) = dev.borrow_mut().receive_packet(Rc::clone(&packet)) {
                            return Some(response);
                        }
                    }
                }
            }
        }
        
        None
    }
}
