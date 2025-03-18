use std::rc::Rc;
use std::cell::RefCell;
use crate::packet::Packet;

pub trait Device {
    fn get_mac_address(&self) -> &str;
    fn get_ip_address(&self) -> &str;
    fn receive_arp_request(&mut self, packet: Rc<Packet>) -> Option<Rc<Packet>>;
    fn receive_packet(&mut self, packet: Rc<Packet>) -> Option<Rc<Packet>>;
}

use crate::host::Host;
impl Device for Host {
    fn get_mac_address(&self) -> &str {
        &self.mac_address
    }

    fn get_ip_address(&self) -> &str {
        &self.ip_address
    }

    fn receive_arp_request(&mut self, packet: Rc<Packet>) -> Option<Rc<Packet>> {
        self.receive_arp_request(packet)
    }

    fn receive_packet(&mut self, packet: Rc<Packet>) -> Option<Rc<Packet>> {
        self.receive_packet(packet)
    }
}

use crate::router::Router;
impl Device for Router {
    fn get_mac_address(&self) -> &str {
        &self.mac_address
    }
    
    fn get_ip_address(&self) -> &str {
        &self.ip_address
    }

    fn receive_arp_request(&mut self, _packet: Rc<Packet>) -> Option<Rc<Packet>> {
        // Dummy implementation: routers could implement ARP reply logic
        None
    }

    fn receive_packet(&mut self, _packet: Rc<Packet>) -> Option<Rc<Packet>> {
        // Dummy implementation: routers could implement packet forwarding logic
        None
    }
}
