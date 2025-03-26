use eframe::{egui, App};
use std::collections::HashMap;

#[derive(Debug)]
struct Node {
    label: String,
    position: egui::Pos2,
}

#[derive(Debug)]
struct Edge {
    from: usize,
    to: usize,
}

pub struct NetworkApp {
    nodes: HashMap<usize, Node>,
    edges: Vec<Edge>,
    selected_node: Option<String>,
}

impl Default for NetworkApp {
    fn default() -> Self {
        let mut nodes = HashMap::new();
        nodes.insert(0, Node { label: "Host A".to_string(), position: egui::pos2(100.0, 100.0) });
        nodes.insert(1, Node { label: "Switch 1".to_string(), position: egui::pos2(300.0, 100.0) });
        nodes.insert(2, Node { label: "Router 1".to_string(), position: egui::pos2(500.0, 300.0) });
        nodes.insert(3, Node { label: "Host B".to_string(), position: egui::pos2(700.0, 100.0) });

        let edges = vec![
            Edge { from: 0, to: 1 },
            Edge { from: 1, to: 2 },
            Edge { from: 2, to: 3 },
        ];

        Self {
            nodes,
            edges,
            selected_node: None,
        }
    }
}

impl App for NetworkApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let painter = ui.painter();

            // Draw edges (connections)
            for edge in &self.edges {
                if let (Some(from), Some(to)) = (self.nodes.get(&edge.from), self.nodes.get(&edge.to)) {
                    painter.line_segment(
                        [from.position, to.position],
                        egui::Stroke::new(2.0, egui::Color32::LIGHT_BLUE),
                    );
                }
            }

            // Draw nodes
            for node in self.nodes.values() {
                let radius = 20.0;
                let rect = egui::Rect::from_center_size(node.position, egui::vec2(radius * 2.0, radius * 2.0));

                let color = if self.selected_node == Some(node.label.clone()) {
                    egui::Color32::from_rgb(255, 100, 100) // Highlighted color
                } else {
                    egui::Color32::from_rgb(100, 150, 255)
                };

                // Draw circle for node
                painter.circle_filled(node.position, radius, color);

                // Draw label
                painter.text(
                    node.position + egui::vec2(0.0, 25.0),
                    egui::Align2::CENTER_CENTER,
                    &node.label,
                    egui::FontId::proportional(16.0),
                    egui::Color32::WHITE,
                );

                // Handle clicks
                if ui.input(|i| i.pointer.button_clicked(egui::PointerButton::Primary)) {
                    if rect.contains(ui.input(|i| i.pointer.interact_pos().unwrap_or_default())) {
                        self.selected_node = Some(node.label.clone());
                        println!("Selected node: {:?}", node);
                    }
                }
            }
        });
    }
}
