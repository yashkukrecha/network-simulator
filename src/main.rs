mod host;
mod router;
mod switch;
mod device;
mod packet;
mod gui;

use std::rc::Rc;
use std::cell::RefCell;
use crate::host::Host;
use crate::router::Router;
use crate::switch::Switch;
use crate::device::Device;
use crate::gui::NetworkApp;

/**
* Routers: 
*   - MAC addresses: AA:BB:CC:DD:EE:{subnet : 6 (connected to main switch)}{router_number}
*   - IP addresses: 192.168.{router_number}.1
* Hosts:
*   - MAC addresses: AA:BB:CC:DD:EE:{subnet}{host_letter}
*   - IP addresses: 192.168.{subnet}.{host_letter}
* Subnet:
*   - IP Address: 192.168.{subnet}.0
*/

fn main() {
    // Create all switches
    let switch1 = Rc::new(RefCell::new(Switch::new(3)));
    let switch2 = Rc::new(RefCell::new(Switch::new(4)));
    let switch3 = Rc::new(RefCell::new(Switch::new(2)));
    let switch4 = Rc::new(RefCell::new(Switch::new(4)));
    let switch5 = Rc::new(RefCell::new(Switch::new(2)));
    let switch_main = Rc::new(RefCell::new(Switch::new(3)));

    // Create routers
    let router1 = Rc::new(RefCell::new(Router::new("192.168.1.1".to_string())));
    let router2 = Rc::new(RefCell::new(Router::new("192.168.2.1".to_string())));
    let router3 = Rc::new(RefCell::new(Router::new("192.168.3.1".to_string())));

    // Add router 1 to switches and populate routing tables
    let router1_port = switch1.borrow_mut().add_device(Rc::clone(&(router1.clone() as Rc<RefCell<dyn Device>>))).unwrap();
    router1.borrow_mut().populate_routing_table(
        "192.168.1.0".to_string(),
        Rc::downgrade(&switch1),
        router1_port,
        "".to_string(),
        "AA:BB:CC:DD:EE:11".to_string(),
    );
    let router1_port = switch2.borrow_mut().add_device(Rc::clone(&(router1.clone() as Rc<RefCell<dyn Device>>))).unwrap();
    router1.borrow_mut().populate_routing_table(
        "192.168.2.0".to_string(),
        Rc::downgrade(&switch2),
        router1_port,
        "".to_string(),
        "AA:BB:CC:DD:EE:21".to_string(),
    );
    let router1_port = switch_main.borrow_mut().add_device(Rc::clone(&(router1.clone() as Rc<RefCell<dyn Device>>))).unwrap();
    router1.borrow_mut().populate_routing_table(
        "192.168.3.0".to_string(),
        Rc::downgrade(&switch_main),
        router1_port,
        "192.168.2.1".to_string(),
        "AA:BB:CC:DD:EE:61".to_string(),
    );
    router1.borrow_mut().populate_routing_table(
        "192.168.4.0".to_string(),
        Rc::downgrade(&switch_main),
        router1_port,
        "192.168.2.1".to_string(),
        "AA:BB:CC:DD:EE:61".to_string(),
    );
    router1.borrow_mut().populate_routing_table(
        "192.168.5.0".to_string(),
        Rc::downgrade(&switch_main),
        router1_port,
        "192.168.3.1".to_string(),
        "AA:BB:CC:DD:EE:61".to_string(),
    );

    // Add router 2 to switches and populate routing tables
    let router2_port = switch_main.borrow_mut().add_device(Rc::clone(&(router2.clone() as Rc<RefCell<dyn Device>>))).unwrap();
    router2.borrow_mut().populate_routing_table(
        "192.168.1.0".to_string(),
        Rc::downgrade(&switch_main),
        router2_port,
        "192.168.1.1".to_string(),
        "AA:BB:CC:DD:EE:62".to_string(),
    );
    router2.borrow_mut().populate_routing_table(
        "192.168.5.0".to_string(),
        Rc::downgrade(&switch_main),
        router2_port,
        "192.168.3.1".to_string(),
        "AA:BB:CC:DD:EE:62".to_string(),
    );
    let router2_port = switch2.borrow_mut().add_device(Rc::clone(&(router2.clone() as Rc<RefCell<dyn Device>>))).unwrap();
    router2.borrow_mut().populate_routing_table(
        "192.168.2.0".to_string(),
        Rc::downgrade(&switch2),
        router2_port,
        "".to_string(),
        "AA:BB:CC:DD:EE:22".to_string(),
    );
    let router2_port = switch3.borrow_mut().add_device(Rc::clone(&(router2.clone() as Rc<RefCell<dyn Device>>))).unwrap();
    router2.borrow_mut().populate_routing_table(
        "192.168.3.0".to_string(),
        Rc::downgrade(&switch3),
        router2_port,
        "".to_string(),
        "AA:BB:CC:DD:EE:32".to_string(),
    );
    let router2_port = switch4.borrow_mut().add_device(Rc::clone(&(router2.clone() as Rc<RefCell<dyn Device>>))).unwrap();
    router2.borrow_mut().populate_routing_table(
        "192.168.4.0".to_string(),
        Rc::downgrade(&switch4),
        router2_port,
        "".to_string(),
        "AA:BB:CC:DD:EE:42".to_string(),
    );

    // Add router 3 to switches and populate routing tables
    let router3_port = switch_main.borrow_mut().add_device(Rc::clone(&(router3.clone() as Rc<RefCell<dyn Device>>))).unwrap();
    router3.borrow_mut().populate_routing_table(
        "192.168.1.0".to_string(),
        Rc::downgrade(&switch_main),
        router3_port,
        "192.168.1.1".to_string(),
        "AA:BB:CC:DD:EE:63".to_string(),
    );
    router3.borrow_mut().populate_routing_table(
        "192.168.2.0".to_string(),
        Rc::downgrade(&switch_main),
        router3_port,
        "192.168.2.1".to_string(),
        "AA:BB:CC:DD:EE:63".to_string(),
    );
    router3.borrow_mut().populate_routing_table(
        "192.168.3.0".to_string(),
        Rc::downgrade(&switch_main),
        router3_port,
        "192.168.2.1".to_string(),
        "AA:BB:CC:DD:EE:63".to_string(),
    );
    router3.borrow_mut().populate_routing_table(
        "192.168.4.0".to_string(),
        Rc::downgrade(&switch_main),
        router3_port,
        "192.168.2.1".to_string(),
        "AA:BB:CC:DD:EE:63".to_string(),
    );
    let router3_port = switch5.borrow_mut().add_device(Rc::clone(&(router3.clone() as Rc<RefCell<dyn Device>>))).unwrap();    
    router3.borrow_mut().populate_routing_table(
        "192.168.5.0".to_string(),
        Rc::downgrade(&switch5),
        router3_port,
        "".to_string(),
        "AA:BB:CC:DD:EE:53".to_string(),
    );
    

    // Create two hosts
    let host_a = Rc::new(RefCell::new(Host::new(
        "192.168.1.A".to_string(),
        "AA:BB:CC:DD:EE:1A".to_string(),
        0,
        Rc::downgrade(&switch1),
    )));
    let host_b = Rc::new(RefCell::new(Host::new(
        "192.168.1.B".to_string(),
        "AA:BB:CC:DD:EE:1B".to_string(),
        0,
        Rc::downgrade(&switch1),
    )));
    let host_c = Rc::new(RefCell::new(Host::new(
        "192.168.2.C".to_string(),
        "AA:BB:CC:DD:EE:2C".to_string(),
        0,
        Rc::downgrade(&switch2),
    )));
    let host_d = Rc::new(RefCell::new(Host::new(
        "192.168.2.D".to_string(),
        "AA:BB:CC:DD:EE:2D".to_string(),
        0,
        Rc::downgrade(&switch2),
    )));
    let host_e = Rc::new(RefCell::new(Host::new(
        "192.168.3.E".to_string(),
        "AA:BB:CC:DD:EE:3E".to_string(),
        0,
        Rc::downgrade(&switch3),
    )));
    let host_f = Rc::new(RefCell::new(Host::new(
        "192.168.4.F".to_string(),
        "AA:BB:CC:DD:EE:4F".to_string(),
        0,
        Rc::downgrade(&switch4),
    )));
    let host_g = Rc::new(RefCell::new(Host::new(
        "192.168.4.G".to_string(),
        "AA:BB:CC:DD:EE:4G".to_string(),
        0,
        Rc::downgrade(&switch4),
    )));
    let host_h = Rc::new(RefCell::new(Host::new(
        "192.168.4.H".to_string(),
        "AA:BB:CC:DD:EE:4H".to_string(),
        0,
        Rc::downgrade(&switch4),
    )));
    let host_i = Rc::new(RefCell::new(Host::new(
        "192.168.5.I".to_string(),
        "AA:BB:CC:DD:EE:5I".to_string(),
        0,
        Rc::downgrade(&switch5),
    )));

    // Add hosts to switch
    let host_a_port = switch1.borrow_mut().add_device(Rc::clone(&(host_a.clone() as Rc<RefCell<dyn Device>>))).unwrap();
    let host_b_port = switch1.borrow_mut().add_device(Rc::clone(&(host_b.clone() as Rc<RefCell<dyn Device>>))).unwrap();
    let host_c_port = switch2.borrow_mut().add_device(Rc::clone(&(host_c.clone() as Rc<RefCell<dyn Device>>))).unwrap();
    let host_d_port = switch2.borrow_mut().add_device(Rc::clone(&(host_d.clone() as Rc<RefCell<dyn Device>>))).unwrap();
    let host_e_port = switch3.borrow_mut().add_device(Rc::clone(&(host_e.clone() as Rc<RefCell<dyn Device>>))).unwrap();
    let host_f_port = switch4.borrow_mut().add_device(Rc::clone(&(host_f.clone() as Rc<RefCell<dyn Device>>))).unwrap();
    let host_g_port = switch4.borrow_mut().add_device(Rc::clone(&(host_g.clone() as Rc<RefCell<dyn Device>>))).unwrap();
    let host_h_port = switch4.borrow_mut().add_device(Rc::clone(&(host_h.clone() as Rc<RefCell<dyn Device>>))).unwrap();
    let host_i_port = switch5.borrow_mut().add_device(Rc::clone(&(host_i.clone() as Rc<RefCell<dyn Device>>))).unwrap();

    // Update host ports
    host_a.borrow_mut().assign_port(host_a_port);
    host_b.borrow_mut().assign_port(host_b_port);
    host_c.borrow_mut().assign_port(host_c_port);
    host_d.borrow_mut().assign_port(host_d_port);
    host_e.borrow_mut().assign_port(host_e_port);
    host_f.borrow_mut().assign_port(host_f_port);
    host_g.borrow_mut().assign_port(host_g_port);
    host_h.borrow_mut().assign_port(host_h_port);
    host_i.borrow_mut().assign_port(host_i_port);

    // Populate host routing tables (0 = direct connection through router)
    let vec_r1 = vec![("192.168.1.0".to_string(), 0), 
        ("192.168.2.0".to_string(), 0), 
        ("192.168.3.0".to_string(), 1), 
        ("192.168.4.0".to_string(), 1), 
        ("192.168.5.0".to_string(), 1)];

    let vec_r2 = vec![("192.168.1.0".to_string(), 1), 
        ("192.168.2.0".to_string(), 0), 
        ("192.168.3.0".to_string(), 0), 
        ("192.168.4.0".to_string(), 0), 
        ("192.168.5.0".to_string(), 1)];

    let vec_r3 = vec![("192.168.1.0".to_string(), 1), 
        ("192.168.2.0".to_string(), 1), 
        ("192.168.3.0".to_string(), 1), 
        ("192.168.4.0".to_string(), 1), 
        ("192.168.5.0".to_string(), 0)];

    host_a.borrow_mut().populate_routing_table("192.168.1.1".to_string(), vec_r1.clone());
    host_b.borrow_mut().populate_routing_table("192.168.1.1".to_string(), vec_r1.clone());

    host_c.borrow_mut().populate_routing_table("192.168.1.1".to_string(), vec_r1.clone());
    host_c.borrow_mut().populate_routing_table("192.168.2.1".to_string(), vec_r2.clone());
    host_d.borrow_mut().populate_routing_table("192.168.1.1".to_string(), vec_r1.clone());
    host_d.borrow_mut().populate_routing_table("192.168.2.1".to_string(), vec_r2.clone());

    host_e.borrow_mut().populate_routing_table("192.168.2.1".to_string(), vec_r2.clone());

    host_f.borrow_mut().populate_routing_table("192.168.2.1".to_string(), vec_r2.clone());
    host_g.borrow_mut().populate_routing_table("192.168.2.1".to_string(), vec_r2.clone());
    host_h.borrow_mut().populate_routing_table("192.168.2.1".to_string(), vec_r2.clone());

    host_i.borrow_mut().populate_routing_table("192.168.3.1".to_string(), vec_r3.clone());

    // WORKING: host-to-host and host-to-router-to-host communication

    let mut app = NetworkApp::default();
    app.add_host_node("Host A", 150.0, 500.0, Rc::clone(&(host_a.clone() as Rc<RefCell<dyn Device>>)));
    app.add_host_node("Host B", 150.0, 400.0, Rc::clone(&(host_b.clone() as Rc<RefCell<dyn Device>>)));
    app.add_host_node("Host C", 500.0, 700.0, Rc::clone(&(host_c.clone() as Rc<RefCell<dyn Device>>)));
    app.add_host_node("Host D", 600.0, 700.0, Rc::clone(&(host_d.clone() as Rc<RefCell<dyn Device>>)));
    app.add_host_node("Host E", 1000.0, 675.0, Rc::clone(&(host_e.clone() as Rc<RefCell<dyn Device>>)));
    app.add_host_node("Host F", 1000.0, 300.0, Rc::clone(&(host_f.clone() as Rc<RefCell<dyn Device>>)));
    app.add_host_node("Host G", 900.0, 200.0, Rc::clone(&(host_g.clone() as Rc<RefCell<dyn Device>>)));
    app.add_host_node("Host H", 950.0, 250.0, Rc::clone(&(host_h.clone() as Rc<RefCell<dyn Device>>)));
    app.add_host_node("Host I", 625.0, 100.0, Rc::clone(&(host_i.clone() as Rc<RefCell<dyn Device>>)));

    app.add_switch_node("Switch 1", 200.0, 450.0, Rc::clone(&(switch1.clone() as Rc<RefCell<dyn Device>>)));
    app.add_switch_node("Switch 2", 550.0, 650.0, Rc::clone(&(switch2.clone() as Rc<RefCell<dyn Device>>)));
    app.add_switch_node("Switch 3", 950.0, 637.5, Rc::clone(&(switch3.clone() as Rc<RefCell<dyn Device>>)));
    app.add_switch_node("Switch 4", 900.0, 300.0, Rc::clone(&(switch4.clone() as Rc<RefCell<dyn Device>>)));
    app.add_switch_node("Switch 5", 550.0, 100.0, Rc::clone(&(switch5.clone() as Rc<RefCell<dyn Device>>)));
    app.add_switch_node("Main Switch", 550.0, 400.0, Rc::clone(&(switch_main.clone() as Rc<RefCell<dyn Device>>)));

    app.add_router_node("Router 1", 400.0, 450.0, Rc::clone(&(router1.clone() as Rc<RefCell<dyn Device>>)));
    app.add_router_node("Router 2", 700.0, 450.0, Rc::clone(&(router2.clone() as Rc<RefCell<dyn Device>>)));
    app.add_router_node("Router 3", 550.0, 300.0, Rc::clone(&(router3.clone() as Rc<RefCell<dyn Device>>)));

    // Edges between hosts and switches
    app.add_edge(0, 9);
    app.add_edge(1, 9);
    app.add_edge(2, 10);
    app.add_edge(3, 10);
    app.add_edge(4, 11);
    app.add_edge(5, 12);
    app.add_edge(6, 12);
    app.add_edge(7, 12);
    app.add_edge(8, 13);
    
    // Edges between switches and routers
    app.add_edge(15, 9);
    app.add_edge(15, 10);
    app.add_edge(15, 14);
    app.add_edge(16, 10);
    app.add_edge(16, 11);
    app.add_edge(16, 12);
    app.add_edge(16, 14);
    app.add_edge(17, 13);
    app.add_edge(17, 14);

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Network Simulator",
        native_options,
        Box::new(|_cc| Box::new(app)),
    );
}