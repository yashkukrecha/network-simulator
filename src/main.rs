fn main() {
    println!("Hello, world!");
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