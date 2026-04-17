//! Knowledge Graph Window - egui-based graph visualization
//!
//! Runs in a separate window alongside the main Dioxus app.
//! Communicates via channels for graph updates.

use eframe::{egui, App};
use egui_graphs::{Graph, GraphView, SettingsInteraction, SettingsNavigation, SettingsStyle};
use pentest_core::evidence::types::{KnowledgeGraph, NodeType};
use petgraph::stable_graph::{NodeIndex, StableGraph};
use std::collections::HashMap;
use std::sync::mpsc::{Receiver, TryRecvError};

#[derive(Clone, Debug)]
struct NodeData {
    label: String,
    node_type: NodeType,
    description: String,
    confidence: f32,
}

#[derive(Clone, Debug)]
struct EdgeData {
    #[allow(dead_code)]
    label: String,
}

/// Message types sent from Dioxus app to graph window
#[derive(Debug)]
pub enum GraphMessage {
    /// Update the graph with new data
    UpdateGraph(KnowledgeGraph),
    /// Clear the graph
    Clear,
    /// Show loading state
    Loading,
}

/// Knowledge Graph window state
pub struct GraphWindow {
    /// Graph data
    graph: Option<Graph<NodeData, EdgeData>>,
    /// Channel receiver for updates from Dioxus
    receiver: Receiver<GraphMessage>,
    /// Loading state
    loading: bool,
    /// Error message
    error: Option<String>,
    /// Selected node for detail panel
    selected_node: Option<NodeIndex>,
}

impl GraphWindow {
    /// Create a new graph window
    pub fn new(receiver: Receiver<GraphMessage>) -> Self {
        Self {
            graph: None,
            receiver,
            loading: false,
            error: None,
            selected_node: None,
        }
    }

    /// Convert KnowledgeGraph to egui_graphs Graph
    fn convert_to_graph(kg: &KnowledgeGraph) -> Graph<NodeData, EdgeData> {
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

        let mut g = Graph::from(&graph);

        // Color and label nodes based on type
        let indices: Vec<_> = g.nodes_iter().map(|(idx, _)| idx).collect();
        for idx in indices {
            if let Some(n) = g.node_mut(idx) {
                let color = match &n.payload().node_type {
                    NodeType::Evidence => egui::Color32::from_rgb(52, 152, 219),     // Blue
                    NodeType::Hypothesis => egui::Color32::from_rgb(241, 196, 15),   // Yellow
                    NodeType::ExploitAttempt => egui::Color32::from_rgb(230, 126, 34), // Orange
                    NodeType::Finding => egui::Color32::from_rgb(231, 76, 60),       // Red
                };
                n.set_color(color);
                n.set_label(n.payload().label.clone());
            }
        }

        g
    }

    /// Check for updates from the main app
    /// Returns true if any messages were received (requires repaint)
    fn check_messages(&mut self) -> bool {
        let mut updated = false;
        loop {
            match self.receiver.try_recv() {
                Ok(msg) => {
                    updated = true;
                    match msg {
                        GraphMessage::UpdateGraph(kg) => {
                            tracing::info!(
                                "Graph window received update: {} nodes, {} edges",
                                kg.nodes.len(),
                                kg.edges.len()
                            );
                            self.graph = Some(Self::convert_to_graph(&kg));
                            self.loading = false;
                            self.error = None;
                        }
                        GraphMessage::Clear => {
                            self.graph = None;
                            self.selected_node = None;
                        }
                        GraphMessage::Loading => {
                            self.loading = true;
                            self.error = None;
                        }
                    }
                }
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => {
                    self.error = Some("Connection to main app lost".to_string());
                    break;
                }
            }
        }
        updated
    }
}

impl App for GraphWindow {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Check for messages from Dioxus app and only repaint if needed
        if self.check_messages() {
            ctx.request_repaint();
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Top panel with title and legend
        egui::Panel::top("header").show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Knowledge Graph");
                ui.separator();

                if let Some(ref graph) = self.graph {
                    // Node count
                    ui.label(format!("Nodes: {}", graph.nodes_iter().count()));
                    ui.separator();
                    ui.label(format!("Edges: {}", graph.edges_iter().count()));
                    ui.separator();
                }

                // Legend
                ui.label("Legend:");
                ui.colored_label(egui::Color32::from_rgb(52, 152, 219), "● Evidence");
                ui.colored_label(egui::Color32::from_rgb(241, 196, 15), "● Hypothesis");
                ui.colored_label(egui::Color32::from_rgb(230, 126, 34), "● Exploit");
                ui.colored_label(egui::Color32::from_rgb(231, 76, 60), "● Finding");
            });
        });

        // Side panel for node details if selected
        if let Some(idx) = self.selected_node {
            if let Some(ref graph) = self.graph {
                if let Some(node) = graph.node(idx) {
                    let data = node.payload();

                    egui::Panel::right("details")
                        .min_size(300.0)
                        .show_inside(ui, |ui| {
                            ui.heading("Node Details");
                            ui.separator();

                            ui.label(format!("Title: {}", data.label));
                            ui.label(format!("Type: {:?}", data.node_type));
                            ui.label(format!("Confidence: {:.0}%", data.confidence * 100.0));
                            ui.separator();

                            ui.label("Description:");
                            ui.label(&data.description);

                            ui.separator();

                            if ui.button("Close").clicked() {
                                self.selected_node = None;
                            }
                        });
                }
            }
        }

        // Main panel with graph or status
        egui::CentralPanel::default().show_inside(ui, |ui| {
            if self.loading {
                ui.centered_and_justified(|ui| {
                    ui.spinner();
                    ui.label("Loading graph data...");
                });
            } else if let Some(ref error) = self.error {
                ui.centered_and_justified(|ui| {
                    ui.colored_label(egui::Color32::RED, format!("Error: {}", error));
                });
            } else if let Some(ref mut graph) = self.graph {
                // Render graph
                let view = &mut GraphView::<
                    _,
                    _,
                    _,
                    _,
                    egui_graphs::DefaultNodeShape,
                    egui_graphs::DefaultEdgeShape,
                >::new(graph)
                .with_navigations(&SettingsNavigation::default())
                .with_interactions(&SettingsInteraction::default())
                .with_styles(&SettingsStyle::default());

                ui.add(view);

                // Update selected node based on graph selection
                if let Some(selected) = graph.selected_nodes().first() {
                    self.selected_node = Some(*selected);
                }
            } else {
                ui.centered_and_justified(|ui| {
                    ui.label("No graph data loaded");
                    ui.label("Select an engagement from the main window");
                });
            }
        });
    }
}

