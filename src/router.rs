#[derive(Debug)]
struct Router {
    arp_table: HashMap<String, String>, // IP address -> MAC address
    routing_table: HashMap<String, (Rc<Switch>, usize)>, // Network address -> (switch, port)
    incoming_packets: Vec<Rc<Packet>>,
    outgoing_packets: Vec<Rc<Packet>>,
    ip_address: String,
    mac_address: String,
}

impl Router {
    fn new(ip_address: String, mac_address: String) -> Self {
        Self {
            arp_table: HashMap::new(),
            routing_table: HashMap::new(),
            incoming_packets: Vec::new(),
            outgoing_packets: Vec::new(),
            ip_address,
            mac_address,
        }
    }
}