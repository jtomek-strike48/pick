//! Proof-of-concept: egui_graphs rendering Knowledge Graph
//!
//! Run with: RUSTUP_TOOLCHAIN= cargo run --example egui_graph_poc
//!
//! This demonstrates that egui_graphs can replace Cytoscape.js for graph visualization,
//! eliminating JavaScript bridge issues in the Dioxus desktop app.

use eframe::{run_native, App, CreationContext, Frame, NativeOptions};
use egui_graphs::{Graph, GraphView, SettingsInteraction, SettingsNavigation, SettingsStyle};
use pentest_core::evidence::mock::generate_small_engagement;
use pentest_core::evidence::types::{KnowledgeGraph, NodeType};
use petgraph::stable_graph::{NodeIndex, StableGraph};
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct NodeData {
    label: String,
    node_type: NodeType,
    description: String,
    confidence: f32,
}

#[derive(Clone, Debug)]
struct EdgeData {
    label: String,
}

struct KnowledgeGraphApp {
    graph: Graph<NodeData, EdgeData>,
    loading: bool,
    error: Option<String>,
}

impl KnowledgeGraphApp {
    fn new(_cc: &CreationContext<'_>) -> Self {
        // Load data directly (no async needed)
        let kg = generate_small_engagement();
        let node_count = kg.nodes.len();
        let edge_count = kg.edges.len();
        println!("Loaded {} nodes, {} edges", node_count, edge_count);

        let graph = Self::convert_to_graph(kg);

        Self {
            graph,
            loading: false,
            error: None,
        }
    }

    fn convert_to_graph(kg: KnowledgeGraph) -> Graph<NodeData, EdgeData> {
        let mut graph: StableGraph<NodeData, EdgeData> = StableGraph::new();
        let mut node_map: HashMap<String, NodeIndex> = HashMap::new();

        // Add nodes
        for node in &kg.nodes {
            let data = NodeData {
                label: node.title.clone(),
                node_type: node.node_type.clone(),
                description: node.description.clone(),
                confidence: node.confidence,
            };

            let idx = graph.add_node(data);
            node_map.insert(node.id.clone(), idx);
        }

        // Add edges
        for edge in &kg.edges {
            if let (Some(&from_idx), Some(&to_idx)) =
                (node_map.get(&edge.from), node_map.get(&edge.to))
            {
                let data = EdgeData {
                    label: format!("{:?}", edge.relationship),
                };
                graph.add_edge(from_idx, to_idx, data);
            }
        }

        // Create graph and color nodes
        let mut g = Graph::from(&graph);

        // Collect node indices first to avoid borrow checker issues
        let indices: Vec<_> = g.nodes_iter().map(|(idx, _)| idx).collect();

        // Color and label nodes based on type
        for idx in indices {
            if let Some(n) = g.node_mut(idx) {
                let color = match &n.payload().node_type {
                    NodeType::Evidence => egui::Color32::from_rgb(52, 152, 219), // Blue
                    NodeType::Hypothesis => egui::Color32::from_rgb(241, 196, 15), // Yellow
                    NodeType::ExploitAttempt => egui::Color32::from_rgb(230, 126, 34), // Orange
                    NodeType::Finding => egui::Color32::from_rgb(231, 76, 60),     // Red
                };
                n.set_color(color);
                n.set_label(n.payload().label.clone());
            }
        }

        g
    }
}

impl App for KnowledgeGraphApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut Frame) {
        egui::Panel::top("header").show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Knowledge Graph POC - egui_graphs");
                ui.separator();
                ui.label("Pan: drag | Zoom: scroll | Select: click");

                // Legend
                ui.separator();
                ui.label("Legend:");
                ui.colored_label(egui::Color32::from_rgb(52, 152, 219), "● Evidence");
                ui.colored_label(egui::Color32::from_rgb(241, 196, 15), "● Hypothesis");
                ui.colored_label(egui::Color32::from_rgb(230, 126, 34), "● Exploit");
                ui.colored_label(egui::Color32::from_rgb(231, 76, 60), "● Finding");
            });
        });

        if let Some(ref error) = self.error {
            egui::CentralPanel::default().show_inside(ui, |ui| {
                ui.centered_and_justified(|ui| {
                    ui.label(format!("Error: {}", error));
                });
            });
            return;
        }

        if self.loading {
            egui::CentralPanel::default().show_inside(ui, |ui| {
                ui.centered_and_justified(|ui| {
                    ui.label("Loading graph data...");
                });
            });
            return;
        }

        // Show node details if selected
        if let Some(selected) = self.graph.selected_nodes().first() {
            if let Some(node) = self.graph.node(*selected) {
                let data = node.payload();

                egui::SidePanel::right("details").min_size(300.0).show_inside(ui, |ui| {
                    ui.heading("Node Details");
                    ui.separator();
                    ui.label(format!("Title: {}", data.label));
                    ui.label(format!("Type: {:?}", data.node_type));
                    ui.label(format!("Confidence: {:.0}%", data.confidence * 100.0));
                    ui.separator();
                    ui.label("Description:");
                    ui.label(&data.description);
                });
            }
        }

        egui::CentralPanel::default().show_inside(ui, |ui| {
            let view = &mut GraphView::<_, _, _, _, egui_graphs::DefaultNodeShape, egui_graphs::DefaultEdgeShape>::new(&mut self.graph)
                .with_navigations(&SettingsNavigation::default())
                .with_interactions(&SettingsInteraction::default())
                .with_styles(&SettingsStyle::default());
            ui.add(view);
        });
    }
}

fn main() {
    let options = NativeOptions::default();

    run_native(
        "Knowledge Graph POC - egui_graphs",
        options,
        Box::new(|cc| Ok(Box::new(KnowledgeGraphApp::new(cc)))),
    )
    .unwrap();
}
