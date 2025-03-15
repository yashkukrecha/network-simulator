use std::Collections::HashMap;
use crate::frame::Frame;

pub struct Host {
    pub incoming_packet_history: Vec<Frame>,
    pub outgoing_packet_history: Vec<Frame>,
    pub arp_table: HashMap<String, String>,
}

impl Host {
    pub fn new() -> Self {
        Self {
            incoming_packet_history: Vec::new(),
            outgoing_packet_history: Vec::new(),
            arp_table: HashMap::new(),
        }
    }
}