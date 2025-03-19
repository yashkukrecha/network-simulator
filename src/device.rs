use std::rc::Rc;
use crate::packet::Packet;

pub trait Device: std::fmt::Debug {
    fn receive_arp_request(&mut self, packet: Rc<Packet>) -> Option<Rc<Packet>>;
    fn receive_packet(&mut self, packet: Rc<Packet>) -> Option<Rc<Packet>>;
}

use crate::host::Host;
impl Device for Host {
    fn receive_arp_request(&mut self, packet: Rc<Packet>) -> Option<Rc<Packet>> {
        self.receive_arp_request(packet)
    }

    fn receive_packet(&mut self, packet: Rc<Packet>) -> Option<Rc<Packet>> {
        self.receive_packet(packet)
    }
}

use crate::router::Router;
impl Device for Router {
    fn receive_arp_request(&mut self, packet: Rc<Packet>) -> Option<Rc<Packet>> {
        self.receive_arp_request(packet)
    }

    fn receive_packet(&mut self, packet: Rc<Packet>) -> Option<Rc<Packet>> {
        self.forward_packet(packet)
    }
}
