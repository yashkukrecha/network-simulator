#[derive(Debug)]
struct Switch {
    mac_table: HashMap<String, usize>,
    ports: Vec<Option<Rc<dyn Device>>>,
    packets: Vec<Rc<Packet>>,
}

impl Switch {
    fn new(port_count: usize) -> Self {
        Self {
            mac_table: HashMap::new(),
            ports: vec![None; port_count],
            packets: Vec::new(),
        }
    }

    fn process_arp_request(&self, packet: Rc<Packet>, port: usize) -> Rc<Packet> {
        // Your logic...
        packet // Just returning the same packet as an example
    }

    fn process_packet(&self, packet: Rc<Packet>, port: usize) -> Rc<Packet> {
        // Your logic...
        packet // Just returning the same packet as an example
    }
}