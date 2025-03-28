use std::rc::Rc;
use crate::packet::Packet;

pub trait Device: std::fmt::Debug {
    fn receive_arp_request(&mut self, packet: Rc<Packet>) -> Option<Rc<Packet>>;
    fn receive_packet(&mut self, packet: Rc<Packet>) -> Option<Rc<Packet>>;
    fn send_packet(&mut self, dest_ip: &str, data: Vec<u8>);
    fn get_ip_address(&self) -> String;
    fn get_device_info(&self) -> String;
}

use crate::host::Host;
impl Device for Host {
    fn receive_arp_request(&mut self, packet: Rc<Packet>) -> Option<Rc<Packet>> {
        self.receive_arp_request(packet)
    }

    fn receive_packet(&mut self, packet: Rc<Packet>) -> Option<Rc<Packet>> {
        self.receive_packet(packet)
    }

    fn send_packet(&mut self, dest_ip: &str, data: Vec<u8>) {
        self.send_packet(dest_ip, data);
    }

    fn get_ip_address(&self) -> String { self.get_ip_address() }

    fn get_device_info(&self) -> String {
        self.get_host_info()
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

    fn send_packet(&mut self, dest_ip: &str, data: Vec<u8>) {}

    fn get_ip_address(&self) -> String { self.get_ip_address() }

    fn get_device_info(&self) -> String {
        self.get_router_info()
    }
}

use crate::switch::Switch;
impl Device for Switch {
    fn receive_arp_request(&mut self, packet: Rc<Packet>) -> Option<Rc<Packet>> {
        None
    }

    fn receive_packet(&mut self, packet: Rc<Packet>) -> Option<Rc<Packet>> {
        None
    }

    fn send_packet(&mut self, dest_ip: &str, data: Vec<u8>) {}

    fn get_ip_address(&self) -> String { "N/A".to_string() }

    fn get_device_info(&self) -> String {
        self.get_switch_info()
    }
}

