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
}

/*
EXAMPLE OF USING RC AND ASSIGNING PORT:
let switch = Rc::new(Switch::new(4));
let host = Rc::new(Host {
    arp_table: HashMap::new(),
    routing_table: HashMap::new(),
    incoming_frames: Vec::new(),
    outgoing_frames: Vec::new(),
    ip_address: "192.168.1.10".to_string(),
    mac_address: "AA:BB:CC:DD:EE:FF".to_string(),
    port: 0,
    switch: Rc::clone(&switch),
});

switch.ports[0] = Some(host);
*/