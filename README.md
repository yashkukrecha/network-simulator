# Network Simulator

A Rust-based network simulator is an educational tool that mimics packet transportation, routing, and other network behaviors, designed to help visualize and test network configurations and behaviors. The project includes a variety of features such as packet forwarding, dynamic routing, error simulation, and a planned GUI for enhanced interactivity.

## Tech Stack

- **Rust**: Core language used for developing the simulator.
- **Cargo**: Rust's package manager and build tool.
- **Egui**: For handling terminal-based interaction and GUI.
- **Tokio**: Asynchronous runtime for handling concurrency, particularly for threads simulating packet forwarding and handling.
- **Serde**: For serializing and deserializing data, especially useful for network packet data structures.

## Features

### Core Features:
- **Packet Simulation**: Simulate the creation, forwarding, and modification of network packets.
- **Routing**:
  - Static routing tables to forward packets.
  - Planned implementation of dynamic routing protocols (e.g., RIP, OSPF).
- **Error Simulation**:
  - Simulate packet loss, corruption, retransmissions, and delays.
  - Configurable bandwidth constraints on routers and switches.
- **TTL (Time-To-Live)**: TTL field in the packet header decreases as the packet travels through routers.
- **TCP/UDP Behavior Simulation**: Support for simple behaviors of TCP and UDP packets for testing.

### GUI Features (Planned):
- **Network Graph**: A graphical representation of the network showing devices, links, and packet flow.
- **Animations**: Visual animations of packets being forwarded through the network.
- **Interactive Nodes**: Clickable network nodes (hosts, switches, routers) to inspect packet history and routing tables.
- **Real-time Updates**: Display packet state changes, routing updates, and error simulations in real time.

### Classes Implemented:
- **Packet**: 
  - Represents a network packet with properties such as source/destination IP, source/destination MAC addresses, and data.
  - Handles ARP requests and responses.
- **Router**:
  - Static routing table.
  - Will support dynamic routing protocols in future updates.
  - Can forward packets based on destination network.
- **Switch**: 
  - Maintains a CAM table (MAC → port).
  - Can forward frames to the correct port.
- **Host**: 
  - Can generate packets.
  - Maintains ARP table (IP → MAC).
  - Sends/receives ARP requests.
- **Device**: 
  - Interface for hosts/routers.

### Future Enhancements:
- **Dynamic Routing**: Implementation of routing protocols such as RIP and OSPF.
- **GUI Integration**: Full graphical user interface with interactive features for network simulation.
- **Protocol Expansion**: More complex protocol simulations like ICMP, ARP, and HTTP.

## Installation

To get started with the simulator, clone the repository and compile the project:

```bash
git clone https://github.com/yashkukrecha/network-simulator.git
cd network-simulator
cargo run
```
Have fun learning about networking!
