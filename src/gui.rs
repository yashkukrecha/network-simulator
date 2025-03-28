use eframe::{egui, App};
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::device::Device;

#[derive(Debug, PartialEq)]
enum NodeType {
    Host,
    Switch,
    Router,
}

#[derive(Debug)]
struct Node {
    label: String,
    position: egui::Pos2,
    node_type: NodeType,
    device: Rc<RefCell<dyn Device>>,
}

#[derive(Debug)]
struct Edge {
    from: usize,
    to: usize,
}

pub struct NetworkApp {
    nodes: HashMap<usize, Node>,
    edges: Vec<Edge>,
    next_node_id: usize,
    selected_node_info: String,
    show_selected_node_info: bool,
    selected_host_1: Option<Rc<RefCell<dyn Device>>>,
    selected_host_2: Option<Rc<RefCell<dyn Device>>>,
    show_device_info: bool,
    device_info: String,
    pan_offset: egui::Vec2,
}

impl Default for NetworkApp {
    fn default() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
            next_node_id: 0,
            selected_node_info: String::new(),
            show_selected_node_info: false,
            selected_host_1: None,
            selected_host_2: None,
            show_device_info: false,
            device_info: String::new(),
            pan_offset: egui::Vec2::ZERO,
        }
    }
}

impl NetworkApp {
    // Add a Host Node
    pub fn add_host_node(&mut self, label: &str, x: f32, y: f32, device: Rc<RefCell<dyn Device>>) {
        self.nodes.insert(
            self.next_node_id,
            Node {
                label: label.to_string(),
                position: egui::pos2(x, y),
                node_type: NodeType::Host,
                device,
            },
        );
        self.next_node_id += 1;
    }

    // Add a Switch Node
    pub fn add_switch_node(&mut self, label: &str, x: f32, y: f32, device: Rc<RefCell<dyn Device>>) {
        self.nodes.insert(
            self.next_node_id,
            Node {
                label: label.to_string(),
                position: egui::pos2(x, y),
                node_type: NodeType::Switch,
                device,
            },
        );
        self.next_node_id += 1;
    }

    // Add a Router Node
    pub fn add_router_node(&mut self, label: &str, x: f32, y: f32, device: Rc<RefCell<dyn Device>>) {
        self.nodes.insert(
            self.next_node_id,
            Node {
                label: label.to_string(),
                position: egui::pos2(x, y),
                node_type: NodeType::Router,
                device,
            },
        );
        self.next_node_id += 1;
    }

    // Add an edge between two nodes
    pub fn add_edge(&mut self, from: usize, to: usize) {
        if self.nodes.contains_key(&from) && self.nodes.contains_key(&to) {
            self.edges.push(Edge { from, to });
        }
    }

    fn get_host_nodes(&self) -> Vec<(usize, String, Rc<RefCell<dyn Device>>)> {
        let mut hosts: Vec<(usize, String, Rc<RefCell<dyn Device>>)> = self.nodes
            .iter()
            .filter_map(|(&id, node)| {
                if node.node_type == NodeType::Host {
                    Some((id, node.label.clone(), node.device.clone()))
                } else {
                    None
                }
            })
            .collect();

        // Sort alphabetically by label
        hosts.sort_by(|a, b| a.1.cmp(&b.1));
        hosts
    }

    // Send packet function
    fn send_packet(&mut self) {
        if let (Some(from_device), Some(to_device)) = (self.selected_host_1.clone(), self.selected_host_2.clone()) {
            // Borrow immutably to get IP address to avoid nested mutable borrows.
            let dest_ip = to_device.borrow().get_ip_address();
            from_device.borrow_mut().send_packet(&dest_ip, Vec::new());

            // Print device information
            let info_source = from_device.borrow().get_device_info();
            let info_dest = to_device.borrow().get_device_info();
            self.device_info = format!(
                "Source Host Info:\n{}\n\nDestination Host Info:\n{}",
                info_source, info_dest
            );
            self.show_device_info = true;
        }
    }
}

impl App for NetworkApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let panel_rect = ui.available_rect_before_wrap();
            // Capture drag events on the entire available area.
            let response = ui.interact(panel_rect, ui.id(), egui::Sense::drag());
            if response.dragged() {
                self.pan_offset += response.drag_delta();
            }
            // Draw input for selecting hosts
            ui.horizontal(|ui| {
                let host_nodes = self.get_host_nodes();

                // Dropdown for Source Host
                ui.label("Select Source Host:");
                egui::ComboBox::from_id_source("host_1")
                    .selected_text(
                        self.selected_host_1
                            .as_ref()
                            .and_then(|selected| {
                                host_nodes
                                    .iter()
                                    .find(|(_, _, device)| Rc::ptr_eq(selected, device))
                                    .map(|(_, label, _)| label.clone())
                            })
                            .unwrap_or_else(|| "None".to_string()),
                    )
                    .show_ui(ui, |ui| {
                        for (_id, label, device) in host_nodes.iter() {
                            let is_selected = self
                                .selected_host_1
                                .as_ref()
                                .map_or(false, |selected| Rc::ptr_eq(selected, device));
                            if ui.selectable_label(is_selected, label).clicked() {
                                self.selected_host_1 = Some(device.clone());
                            }
                        }
                    });

                // Dropdown for Destination Host
                ui.label("Select Destination Host:");
                egui::ComboBox::from_id_source("host_2")
                    .selected_text(
                        self.selected_host_2
                            .as_ref()
                            .and_then(|selected| {
                                host_nodes
                                    .iter()
                                    .find(|(_, _, device)| Rc::ptr_eq(selected, device))
                                    .map(|(_, label, _)| label.clone())
                            })
                            .unwrap_or_else(|| "None".to_string()),
                    )
                    .show_ui(ui, |ui| {
                        for (_id, label, device) in host_nodes.iter() {
                            let is_selected = self
                                .selected_host_2
                                .as_ref()
                                .map_or(false, |selected| Rc::ptr_eq(selected, device));
                            if ui.selectable_label(is_selected, label).clicked() {
                                self.selected_host_2 = Some(device.clone());
                            }
                        }
                    });

                // Send packet button
                if ui.button("Send Packet").clicked() {
                    self.send_packet();
                }
            });

            // Draw edges (connections)
            let painter = ui.painter();
            for edge in &self.edges {
                if let (Some(from), Some(to)) = (self.nodes.get(&edge.from), self.nodes.get(&edge.to)) {
                    painter.line_segment(
                        [from.position + self.pan_offset, to.position + self.pan_offset],
                        egui::Stroke::new(3.0, egui::Color32::from_rgb(179, 179, 179)),
                    );
                }
            }

            // Draw nodes and highlight if selected as host
            for node in self.nodes.values() {
                let radius = match node.node_type {
                    NodeType::Host => 20.0,
                    NodeType::Switch => 10.0,
                    NodeType::Router => 30.0,
                };

                let node_pos = node.position + self.pan_offset;
                let rect = egui::Rect::from_center_size(node_pos, egui::vec2(radius * 2.0, radius * 2.0));

                // Set default color based on node type
                let mut color = if node.node_type == NodeType::Host {
                    egui::Color32::from_rgb(102, 197, 204)
                } else if node.node_type == NodeType::Switch {
                    egui::Color32::from_rgb(246, 207, 113)
                } else {
                    egui::Color32::from_rgb(248, 156, 116)
                };

                // If the node's device is one of the selected hosts, change its color (highlight).
                if let Some(ref sel1) = self.selected_host_1 {
                    if Rc::ptr_eq(sel1, &node.device) {
                        // For example, blend in yellow.
                        color = egui::Color32::YELLOW;
                    }
                }
                if let Some(ref sel2) = self.selected_host_2 {
                    if Rc::ptr_eq(sel2, &node.device) {
                        // For example, blend in magenta.
                        color = egui::Color32::from_rgb(255, 0, 255);
                    }
                }

                // Draw circle for node
                painter.circle_filled(node_pos, radius, color);
                if node.node_type == NodeType::Switch && node.label != "Main Switch" {
                    painter.circle_stroke(
                        node_pos,
                        135.0,
                        egui::Stroke::new(0.5, egui::Color32::from_rgb(179, 179, 179)),
                    );
                }

                // Draw label (position adjusted per type)
                let vec = if node.node_type == NodeType::Host && node.label != "Host B" {
                    egui::vec2(0.0, 30.0)
                } else if node.node_type == NodeType::Switch && node.label != "Main Switch" {
                    egui::vec2(-45.0, 0.0)
                } else if node.node_type == NodeType::Router {
                    egui::vec2(0.0, 40.0)
                } else if node.node_type == NodeType::Switch {
                    egui::vec2(-60.0, -10.0)
                } else {
                    egui::vec2(0.0, -30.0)
                };
                painter.text(
                    node_pos + vec,
                    egui::Align2::CENTER_CENTER,
                    &node.label,
                    egui::FontId::proportional(16.0),
                    egui::Color32::WHITE,
                );

                // Handle node clicks for selection
                if ui.input(|i| i.pointer.button_clicked(egui::PointerButton::Primary)) {
                    if rect.contains(ui.input(|i| i.pointer.interact_pos().unwrap_or_default())) {
                        self.show_selected_node_info = true;
                        self.selected_node_info = node.device.borrow().get_device_info();
                    }
                }
            }
            
            // Show a popup window with device info if the flag is set.
            if self.show_selected_node_info {
                egui::Window::new("Device Info")
                    .open(&mut self.show_selected_node_info)
                    .show(ctx, |ui| {
                        ui.label("Device Information:");
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            ui.add(
                                egui::TextEdit::multiline(&mut self.selected_node_info)
                                    .font(egui::TextStyle::Monospace)
                                    .desired_rows(10)
                            );
                        });
                    });
            }

            // Show a popup window with host info if the flag is set.
            if self.show_device_info {
                egui::Window::new("Host Info")
                    .open(&mut self.show_device_info)
                    .show(ctx, |ui| {
                        ui.label("Host Information:");
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            ui.add(
                                egui::TextEdit::multiline(&mut self.device_info)
                                    .font(egui::TextStyle::Monospace)
                                    .desired_rows(10)
                            );
                        });
                    });
            }       
        });
    }
}