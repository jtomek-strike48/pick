//! Bridge module for sending graph data to the graph window
//!
//! Provides a simple API for Dioxus components to communicate with the egui graph window.

use crate::{send_to_graph_window, GraphMessage};
use pentest_core::evidence::mock::generate_small_engagement;

/// Send sample graph data to the graph window for testing
pub fn send_sample_graph() {
    tracing::info!("Sending sample graph data to graph window");
    let graph = generate_small_engagement();
    send_to_graph_window(GraphMessage::UpdateGraph(graph));
}

/// Send loading state to graph window
pub fn send_loading() {
    send_to_graph_window(GraphMessage::Loading);
}

/// Clear the graph window
pub fn send_clear() {
    send_to_graph_window(GraphMessage::Clear);
}
