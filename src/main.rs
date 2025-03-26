mod host;
mod router;
mod switch;
mod device;
mod packet;

use std::rc::Rc;
use std::cell::RefCell;
use crate::host::Host;
use crate::router::Router;
use crate::switch::Switch;
use crate::device::Device;
// use crate::packet::Packet;

fn main() {
    // Create a switch with 4 ports
    let switch1 = Rc::new(RefCell::new(Switch::new(4)));

    // Create a router
    let router1 = Rc::new(RefCell::new(Router::new(
        "192.168.1.1".to_string(),
    )));

    // Add router to switch
    let router1_port = switch1.borrow_mut().add_device(Rc::clone(&(router1.clone() as Rc<RefCell<dyn Device>>))).unwrap();

    // Create two hosts
    let host1 = Rc::new(RefCell::new(Host::new(
        "192.168.1.2".to_string(),
        "AA:BB:CC:DD:EE:02".to_string(),
        0,
        Rc::downgrade(&switch1),
    )));
    let host2 = Rc::new(RefCell::new(Host::new(
        "192.168.1.3".to_string(),
        "AA:BB:CC:DD:EE:03".to_string(),
        1,
        Rc::downgrade(&switch1),
    )));

    // Add hosts to switch
    let host1_port = switch1.borrow_mut().add_device(Rc::clone(&(host1.clone() as Rc<RefCell<dyn Device>>))).unwrap();
    let host2_port = switch1.borrow_mut().add_device(Rc::clone(&(host2.clone() as Rc<RefCell<dyn Device>>))).unwrap();

    // Update host ports
    host1.borrow_mut().assign_port(host1_port);
    host2.borrow_mut().assign_port(host2_port);

    // Populate router routing table
    router1.borrow_mut().populate_routing_table(
        "192.168.1.0".to_string(),
        Rc::downgrade(&switch1),
        router1_port,
        "".to_string(),
        "AA:BB:CC:DD:EE:01".to_string(),
    );

    // Populate host routing tables (0 = direct connection through router)
    host1.borrow_mut().populate_routing_table(
        "192.168.1.1".to_string(),
        "192.168.1.0".to_string(),
        0,
    );
    host2.borrow_mut().populate_routing_table(
        "192.168.1.1".to_string(),
        "192.168.1.0".to_string(),
        0,
    );

    // Test packet transmission from host1 to host2
    println!("=== Sending packet from Host 1 to Host 2 ===");
    host1.borrow_mut().send_packet(
        "192.168.1.3",
        b"Hello".to_vec(),
    );
}

/*
fn main() {
    let switch = Rc::new(RefCell::new(Switch::new(4))); // A switch with 4 ports

    let host = Rc::new(RefCell::new(Host::new(
        "192.168.1.2".to_string(),
        "AA:BB:CC:DD:EE:FF".to_string(),
        0, // Temporary port, will be assigned later
        Rc::downgrade(&switch),
    )));

    // Assign the host to a port on the switch
    if let Some(port) = switch.borrow_mut().add_device(Rc::clone(&host) as Rc<RefCell<dyn Device>>) {
        host.borrow_mut().port = port; // Update the host with the assigned port
        println!("Host assigned to port {}", port);
    } else {
        println!("No available ports on the switch");
    }
}
*/