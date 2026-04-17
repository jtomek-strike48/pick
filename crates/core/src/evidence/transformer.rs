//! Transform evidence chain data to Cytoscape.js visualization format
//!
//! This module converts KnowledgeGraph data structures into the JSON format
//! expected by Cytoscape.js for rendering interactive graph visualizations.
//!
//! # Cytoscape.js Format
//!
//! Nodes and edges are transformed with styling information embedded in each element.
//! Styling rules include:
//! - Node color based on type (Evidence=blue, Hypothesis=orange, ExploitAttempt=purple, Finding=red)
//! - Border color based on confidence level (low=red, medium=orange, high=green)
//! - Node shape based on type (ellipse, diamond, hexagon, rectangle)
//! - Edge color based on confidence (gradient from red to green)

#[cfg(test)]
use super::types::NodeType;
use super::types::{EvidenceEdge, EvidenceNode, KnowledgeGraph};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Cytoscape.js graph format
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CytoscapeGraph {
    pub elements: CytoscapeElements,
}

/// Container for nodes and edges
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CytoscapeElements {
    pub nodes: Vec<CytoscapeNode>,
    pub edges: Vec<CytoscapeEdge>,
}

/// Cytoscape.js node format
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CytoscapeNode {
    pub data: CytoscapeNodeData,
    pub style: CytoscapeNodeStyle,
}

/// Node data payload
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CytoscapeNodeData {
    pub id: String,
    pub label: String,
    #[serde(rename = "nodeType")]
    pub node_type: String,
    pub confidence: f32,
    pub description: String,
    pub metadata: serde_json::Value,
}

/// Node styling information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CytoscapeNodeStyle {
    #[serde(rename = "background-color")]
    pub background_color: String,
    #[serde(rename = "border-color")]
    pub border_color: String,
    pub shape: String,
}

/// Cytoscape.js edge format
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CytoscapeEdge {
    pub data: CytoscapeEdgeData,
    pub style: CytoscapeEdgeStyle,
}

/// Edge data payload
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CytoscapeEdgeData {
    pub id: String,
    pub source: String,
    pub target: String,
    pub label: String,
    pub confidence: f32,
}

/// Edge styling information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CytoscapeEdgeStyle {
    #[serde(rename = "line-color")]
    pub line_color: String,
}

/// Transform a KnowledgeGraph into Cytoscape.js format
///
/// This function converts the internal evidence chain representation into
/// the format expected by Cytoscape.js. It applies styling rules based on
/// node types and confidence levels.
///
/// # Styling Rules
///
/// - **Node Colors** (by type):
///   - Evidence: Blue (#3B82F6)
///   - Hypothesis: Orange (#F59E0B)
///   - ExploitAttempt: Purple (#8B5CF6)
///   - Finding: Red (#EF4444)
///
/// - **Border Colors** (by confidence):
///   - Low (< 0.3): Red (#EF4444)
///   - Medium (0.3-0.7): Orange (#F59E0B)
///   - High (0.7-0.9): Green (#10B981)
///   - Very High (>= 0.9): Dark Green (#059669)
///
/// - **Node Shapes** (by type):
///   - Evidence: ellipse
///   - Hypothesis: diamond
///   - ExploitAttempt: hexagon
///   - Finding: rectangle
///
/// - **Edge Colors** (by confidence):
///   - Same gradient as border colors
///
/// # Error Handling
///
/// Invalid nodes (confidence out of bounds, empty IDs) are skipped with warnings.
/// Edges referencing non-existent nodes are also skipped.
///
/// # Examples
///
/// ```
/// use pentest_core::evidence::types::KnowledgeGraph;
/// use pentest_core::evidence::transformer::transform_to_cytoscape;
///
/// let graph = KnowledgeGraph::new();
/// let cytoscape = transform_to_cytoscape(&graph);
/// assert_eq!(cytoscape.elements.nodes.len(), 0);
/// ```
///
/// ```
/// use pentest_core::evidence::mock::generate_small_engagement;
/// use pentest_core::evidence::transformer::transform_to_cytoscape;
///
/// let graph = generate_small_engagement();
/// let cytoscape = transform_to_cytoscape(&graph);
///
/// // Serialize to JSON for frontend
/// let json = serde_json::to_string_pretty(&cytoscape).unwrap();
/// println!("{}", json);
/// ```
pub fn transform_to_cytoscape(graph: &KnowledgeGraph) -> CytoscapeGraph {
    let nodes = graph.nodes.iter().filter_map(transform_node).collect();

    // Build set of valid node IDs for edge validation
    let valid_node_ids: HashSet<&str> = graph.nodes.iter().map(|n| n.id.as_str()).collect();

    let edges = graph
        .edges
        .iter()
        .filter_map(|edge| transform_edge(edge, &valid_node_ids))
        .collect();

    CytoscapeGraph {
        elements: CytoscapeElements { nodes, edges },
    }
}

/// Transform a single node into Cytoscape.js format
///
/// Returns None if the node is invalid (confidence out of bounds, validation fails)
fn transform_node(node: &EvidenceNode) -> Option<CytoscapeNode> {
    // Validate node first
    if let Err(e) = node.validate() {
        eprintln!("Warning: Skipping invalid node {}: {}", node.id, e);
        return None;
    }

    Some(CytoscapeNode {
        data: CytoscapeNodeData {
            id: node.id.clone(),
            label: node.title.clone(),
            node_type: format!("{:?}", node.node_type).to_uppercase(),
            confidence: node.confidence,
            description: node.description.clone(),
            metadata: node.metadata.clone(),
        },
        style: CytoscapeNodeStyle {
            background_color: node.node_type.color().to_string(),
            border_color: node.confidence_color().to_string(),
            shape: node.node_type.shape().to_string(),
        },
    })
}

/// Transform a single edge into Cytoscape.js format
///
/// Returns None if the edge is invalid or references non-existent nodes
fn transform_edge(edge: &EvidenceEdge, valid_node_ids: &HashSet<&str>) -> Option<CytoscapeEdge> {
    // Validate edge first
    if let Err(e) = edge.validate() {
        eprintln!("Warning: Skipping invalid edge {}: {}", edge.id, e);
        return None;
    }

    // Check if both nodes exist
    if !valid_node_ids.contains(edge.from.as_str()) {
        eprintln!(
            "Warning: Skipping edge {} - source node {} not found",
            edge.id, edge.from
        );
        return None;
    }

    if !valid_node_ids.contains(edge.to.as_str()) {
        eprintln!(
            "Warning: Skipping edge {} - target node {} not found",
            edge.id, edge.to
        );
        return None;
    }

    Some(CytoscapeEdge {
        data: CytoscapeEdgeData {
            id: edge.id.clone(),
            source: edge.from.clone(),
            target: edge.to.clone(),
            label: edge.relationship.label().to_string(),
            confidence: edge.confidence,
        },
        style: CytoscapeEdgeStyle {
            line_color: confidence_to_edge_color(edge.confidence),
        },
    })
}

/// Map confidence value to edge color
fn confidence_to_edge_color(confidence: f32) -> String {
    match confidence {
        c if c < 0.3 => "#EF4444".to_string(), // Red (low confidence)
        c if c < 0.7 => "#F59E0B".to_string(), // Orange (medium confidence)
        c if c < 0.9 => "#10B981".to_string(), // Green (high confidence)
        _ => "#059669".to_string(),            // Dark green (very high confidence)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_empty_graph_transformation() {
        // RED: Write test first
        let graph = KnowledgeGraph::new();
        let result = transform_to_cytoscape(&graph);

        assert_eq!(result.elements.nodes.len(), 0);
        assert_eq!(result.elements.edges.len(), 0);
    }

    #[test]
    fn test_single_node_transformation_evidence_high_confidence() {
        // RED: Test for single Evidence node with high confidence (0.95)
        let node = EvidenceNode {
            id: "node-1".to_string(),
            node_type: NodeType::Evidence,
            title: "Nmap scan completed".to_string(),
            description: "Discovered open ports".to_string(),
            confidence: 0.95,
            timestamp: Utc::now(),
            created_by: "pick-agent-1".to_string(),
            engagement_id: "eng-1".to_string(),
            target: Some("192.168.1.10".to_string()),
            metadata: serde_json::json!({
                "tool": "nmap",
                "ports": [22, 80, 443]
            }),
        };

        let graph = KnowledgeGraph {
            nodes: vec![node],
            edges: vec![],
        };

        let result = transform_to_cytoscape(&graph);

        // Should have exactly one node, no edges
        assert_eq!(result.elements.nodes.len(), 1);
        assert_eq!(result.elements.edges.len(), 0);

        let cyto_node = &result.elements.nodes[0];

        // Verify data fields
        assert_eq!(cyto_node.data.id, "node-1");
        assert_eq!(cyto_node.data.label, "Nmap scan completed");
        assert_eq!(cyto_node.data.node_type, "EVIDENCE");
        assert_eq!(cyto_node.data.confidence, 0.95);
        assert_eq!(cyto_node.data.description, "Discovered open ports");

        // Verify metadata is preserved
        assert_eq!(cyto_node.data.metadata["tool"], "nmap");
        assert_eq!(
            cyto_node.data.metadata["ports"],
            serde_json::json!([22, 80, 443])
        );

        // Verify styling
        // Evidence nodes should be blue (#3B82F6)
        assert_eq!(cyto_node.style.background_color, "#3B82F6");
        // High confidence (0.95) should be dark green border (#059669)
        assert_eq!(cyto_node.style.border_color, "#059669");
        // Evidence nodes should be ellipse shape
        assert_eq!(cyto_node.style.shape, "ellipse");
    }

    #[test]
    fn test_single_edge_transformation() {
        // RED: Test for single edge between two nodes
        use super::super::types::EdgeType;

        let node1 = EvidenceNode {
            id: "node-1".to_string(),
            node_type: NodeType::Evidence,
            title: "Nmap scan".to_string(),
            description: "Port scan results".to_string(),
            confidence: 0.95,
            timestamp: Utc::now(),
            created_by: "pick-agent-1".to_string(),
            engagement_id: "eng-1".to_string(),
            target: Some("192.168.1.10".to_string()),
            metadata: serde_json::json!({}),
        };

        let node2 = EvidenceNode {
            id: "node-2".to_string(),
            node_type: NodeType::Hypothesis,
            title: "SSH weak creds hypothesis".to_string(),
            description: "SSH may have weak credentials".to_string(),
            confidence: 0.70,
            timestamp: Utc::now(),
            created_by: "ai-planner".to_string(),
            engagement_id: "eng-1".to_string(),
            target: Some("192.168.1.10".to_string()),
            metadata: serde_json::json!({}),
        };

        let edge = EvidenceEdge {
            id: "edge-1".to_string(),
            from: "node-1".to_string(),
            to: "node-2".to_string(),
            relationship: EdgeType::Supports,
            confidence: 0.85,
            created_at: Utc::now(),
        };

        let graph = KnowledgeGraph {
            nodes: vec![node1, node2],
            edges: vec![edge],
        };

        let result = transform_to_cytoscape(&graph);

        // Should have 2 nodes and 1 edge
        assert_eq!(result.elements.nodes.len(), 2);
        assert_eq!(result.elements.edges.len(), 1);

        let cyto_edge = &result.elements.edges[0];

        // Verify edge data fields
        assert_eq!(cyto_edge.data.id, "edge-1");
        assert_eq!(cyto_edge.data.source, "node-1");
        assert_eq!(cyto_edge.data.target, "node-2");
        assert_eq!(cyto_edge.data.label, "supports");
        assert_eq!(cyto_edge.data.confidence, 0.85);

        // Verify edge styling
        // Confidence 0.85 should map to light green (#10B981)
        assert_eq!(cyto_edge.style.line_color, "#10B981");
    }

    #[test]
    fn test_complete_small_graph_from_mock() {
        // RED: Test complete transformation using mock data (7 nodes, 6 edges)
        use super::super::mock::generate_small_engagement;

        let graph = generate_small_engagement();
        let result = transform_to_cytoscape(&graph);

        // Verify correct number of elements
        assert_eq!(result.elements.nodes.len(), 7);
        assert_eq!(result.elements.edges.len(), 6);

        // Verify first node (Evidence type)
        let node1 = &result.elements.nodes[0];
        assert_eq!(node1.data.id, "node-1");
        assert_eq!(node1.data.label, "Nmap port scan completed");
        assert_eq!(node1.data.node_type, "EVIDENCE");
        assert_eq!(node1.style.background_color, "#3B82F6"); // Blue
        assert_eq!(node1.style.shape, "ellipse");

        // Verify second node (Hypothesis type)
        let node2 = &result.elements.nodes[1];
        assert_eq!(node2.data.id, "node-2");
        assert_eq!(node2.data.node_type, "HYPOTHESIS");
        assert_eq!(node2.style.background_color, "#F59E0B"); // Orange
        assert_eq!(node2.style.shape, "diamond");

        // Verify third node (ExploitAttempt type)
        let node3 = &result.elements.nodes[2];
        assert_eq!(node3.data.id, "node-3");
        assert_eq!(node3.data.node_type, "EXPLOITATTEMPT");
        assert_eq!(node3.style.background_color, "#8B5CF6"); // Purple
        assert_eq!(node3.style.shape, "hexagon");

        // Verify fourth node (Finding type)
        let node4 = &result.elements.nodes[3];
        assert_eq!(node4.data.id, "node-4");
        assert_eq!(node4.data.node_type, "FINDING");
        assert_eq!(node4.style.background_color, "#EF4444"); // Red
        assert_eq!(node4.style.shape, "rectangle");

        // Verify edge relationships
        let edge1 = &result.elements.edges[0];
        assert_eq!(edge1.data.source, "node-1");
        assert_eq!(edge1.data.target, "node-2");
        assert_eq!(edge1.data.label, "supports");

        // Verify all nodes have metadata preserved
        assert!(node1.data.metadata["tool"].is_string());
        assert!(node1.data.metadata["openPorts"].is_array());
    }

    #[test]
    fn test_invalid_node_skipped_gracefully() {
        // RED: Test that invalid nodes (confidence out of bounds) are skipped
        let valid_node = EvidenceNode {
            id: "node-1".to_string(),
            node_type: NodeType::Evidence,
            title: "Valid node".to_string(),
            description: "This is valid".to_string(),
            confidence: 0.95,
            timestamp: Utc::now(),
            created_by: "pick-agent-1".to_string(),
            engagement_id: "eng-1".to_string(),
            target: None,
            metadata: serde_json::json!({}),
        };

        let invalid_node = EvidenceNode {
            id: "node-2".to_string(),
            node_type: NodeType::Hypothesis,
            title: "Invalid node".to_string(),
            description: "This has invalid confidence".to_string(),
            confidence: 1.5, // Out of bounds!
            timestamp: Utc::now(),
            created_by: "pick-agent-1".to_string(),
            engagement_id: "eng-1".to_string(),
            target: None,
            metadata: serde_json::json!({}),
        };

        let graph = KnowledgeGraph {
            nodes: vec![valid_node, invalid_node],
            edges: vec![],
        };

        let result = transform_to_cytoscape(&graph);

        // Should only have 1 node (invalid one skipped)
        assert_eq!(result.elements.nodes.len(), 1);
        assert_eq!(result.elements.nodes[0].data.id, "node-1");
    }

    #[test]
    fn test_edge_referencing_nonexistent_node_skipped() {
        // RED: Test that edges referencing non-existent nodes are skipped
        use super::super::types::EdgeType;

        let node1 = EvidenceNode {
            id: "node-1".to_string(),
            node_type: NodeType::Evidence,
            title: "Valid node".to_string(),
            description: "This exists".to_string(),
            confidence: 0.95,
            timestamp: Utc::now(),
            created_by: "pick-agent-1".to_string(),
            engagement_id: "eng-1".to_string(),
            target: None,
            metadata: serde_json::json!({}),
        };

        // Edge references non-existent node-2
        let invalid_edge = EvidenceEdge {
            id: "edge-1".to_string(),
            from: "node-1".to_string(),
            to: "node-2".to_string(), // This node doesn't exist!
            relationship: EdgeType::Supports,
            confidence: 0.85,
            created_at: Utc::now(),
        };

        let graph = KnowledgeGraph {
            nodes: vec![node1],
            edges: vec![invalid_edge],
        };

        let result = transform_to_cytoscape(&graph);

        // Should have 1 node but 0 edges (invalid edge skipped)
        assert_eq!(result.elements.nodes.len(), 1);
        assert_eq!(result.elements.edges.len(), 0);
    }

    #[test]
    fn test_all_node_types_styling() {
        // RED: Test that all node types have correct colors and shapes
        let nodes = vec![
            EvidenceNode {
                id: "node-evidence".to_string(),
                node_type: NodeType::Evidence,
                title: "Evidence node".to_string(),
                description: "Test".to_string(),
                confidence: 0.9,
                timestamp: Utc::now(),
                created_by: "test".to_string(),
                engagement_id: "eng-1".to_string(),
                target: None,
                metadata: serde_json::json!({}),
            },
            EvidenceNode {
                id: "node-hypothesis".to_string(),
                node_type: NodeType::Hypothesis,
                title: "Hypothesis node".to_string(),
                description: "Test".to_string(),
                confidence: 0.9,
                timestamp: Utc::now(),
                created_by: "test".to_string(),
                engagement_id: "eng-1".to_string(),
                target: None,
                metadata: serde_json::json!({}),
            },
            EvidenceNode {
                id: "node-exploit".to_string(),
                node_type: NodeType::ExploitAttempt,
                title: "Exploit node".to_string(),
                description: "Test".to_string(),
                confidence: 0.9,
                timestamp: Utc::now(),
                created_by: "test".to_string(),
                engagement_id: "eng-1".to_string(),
                target: None,
                metadata: serde_json::json!({}),
            },
            EvidenceNode {
                id: "node-finding".to_string(),
                node_type: NodeType::Finding,
                title: "Finding node".to_string(),
                description: "Test".to_string(),
                confidence: 0.9,
                timestamp: Utc::now(),
                created_by: "test".to_string(),
                engagement_id: "eng-1".to_string(),
                target: None,
                metadata: serde_json::json!({}),
            },
        ];

        let graph = KnowledgeGraph {
            nodes,
            edges: vec![],
        };

        let result = transform_to_cytoscape(&graph);

        assert_eq!(result.elements.nodes.len(), 4);

        // Evidence: blue, ellipse
        let evidence = &result.elements.nodes[0];
        assert_eq!(evidence.data.node_type, "EVIDENCE");
        assert_eq!(evidence.style.background_color, "#3B82F6");
        assert_eq!(evidence.style.shape, "ellipse");

        // Hypothesis: orange, diamond
        let hypothesis = &result.elements.nodes[1];
        assert_eq!(hypothesis.data.node_type, "HYPOTHESIS");
        assert_eq!(hypothesis.style.background_color, "#F59E0B");
        assert_eq!(hypothesis.style.shape, "diamond");

        // ExploitAttempt: purple, hexagon
        let exploit = &result.elements.nodes[2];
        assert_eq!(exploit.data.node_type, "EXPLOITATTEMPT");
        assert_eq!(exploit.style.background_color, "#8B5CF6");
        assert_eq!(exploit.style.shape, "hexagon");

        // Finding: red, rectangle
        let finding = &result.elements.nodes[3];
        assert_eq!(finding.data.node_type, "FINDING");
        assert_eq!(finding.style.background_color, "#EF4444");
        assert_eq!(finding.style.shape, "rectangle");
    }

    #[test]
    fn test_all_confidence_levels_border_colors() {
        // RED: Test that confidence levels map to correct border colors
        let nodes = vec![
            EvidenceNode {
                id: "node-low".to_string(),
                node_type: NodeType::Evidence,
                title: "Low confidence".to_string(),
                description: "Test".to_string(),
                confidence: 0.2, // Low confidence
                timestamp: Utc::now(),
                created_by: "test".to_string(),
                engagement_id: "eng-1".to_string(),
                target: None,
                metadata: serde_json::json!({}),
            },
            EvidenceNode {
                id: "node-medium".to_string(),
                node_type: NodeType::Evidence,
                title: "Medium confidence".to_string(),
                description: "Test".to_string(),
                confidence: 0.5, // Medium confidence
                timestamp: Utc::now(),
                created_by: "test".to_string(),
                engagement_id: "eng-1".to_string(),
                target: None,
                metadata: serde_json::json!({}),
            },
            EvidenceNode {
                id: "node-high".to_string(),
                node_type: NodeType::Evidence,
                title: "High confidence".to_string(),
                description: "Test".to_string(),
                confidence: 0.8, // High confidence
                timestamp: Utc::now(),
                created_by: "test".to_string(),
                engagement_id: "eng-1".to_string(),
                target: None,
                metadata: serde_json::json!({}),
            },
            EvidenceNode {
                id: "node-very-high".to_string(),
                node_type: NodeType::Evidence,
                title: "Very high confidence".to_string(),
                description: "Test".to_string(),
                confidence: 0.95, // Very high confidence
                timestamp: Utc::now(),
                created_by: "test".to_string(),
                engagement_id: "eng-1".to_string(),
                target: None,
                metadata: serde_json::json!({}),
            },
        ];

        let graph = KnowledgeGraph {
            nodes,
            edges: vec![],
        };

        let result = transform_to_cytoscape(&graph);

        assert_eq!(result.elements.nodes.len(), 4);

        // Low confidence (< 0.3): red border
        assert_eq!(result.elements.nodes[0].style.border_color, "#EF4444");

        // Medium confidence (0.3-0.7): orange border
        assert_eq!(result.elements.nodes[1].style.border_color, "#F59E0B");

        // High confidence (0.7-0.9): green border
        assert_eq!(result.elements.nodes[2].style.border_color, "#10B981");

        // Very high confidence (>= 0.9): dark green border
        assert_eq!(result.elements.nodes[3].style.border_color, "#059669");
    }

    #[test]
    fn test_all_confidence_levels_edge_colors() {
        // RED: Test that edge confidence levels map to correct colors
        use super::super::types::EdgeType;

        let nodes = vec![
            EvidenceNode {
                id: "node-1".to_string(),
                node_type: NodeType::Evidence,
                title: "Node 1".to_string(),
                description: "Test".to_string(),
                confidence: 0.9,
                timestamp: Utc::now(),
                created_by: "test".to_string(),
                engagement_id: "eng-1".to_string(),
                target: None,
                metadata: serde_json::json!({}),
            },
            EvidenceNode {
                id: "node-2".to_string(),
                node_type: NodeType::Hypothesis,
                title: "Node 2".to_string(),
                description: "Test".to_string(),
                confidence: 0.9,
                timestamp: Utc::now(),
                created_by: "test".to_string(),
                engagement_id: "eng-1".to_string(),
                target: None,
                metadata: serde_json::json!({}),
            },
            EvidenceNode {
                id: "node-3".to_string(),
                node_type: NodeType::ExploitAttempt,
                title: "Node 3".to_string(),
                description: "Test".to_string(),
                confidence: 0.9,
                timestamp: Utc::now(),
                created_by: "test".to_string(),
                engagement_id: "eng-1".to_string(),
                target: None,
                metadata: serde_json::json!({}),
            },
            EvidenceNode {
                id: "node-4".to_string(),
                node_type: NodeType::Finding,
                title: "Node 4".to_string(),
                description: "Test".to_string(),
                confidence: 0.9,
                timestamp: Utc::now(),
                created_by: "test".to_string(),
                engagement_id: "eng-1".to_string(),
                target: None,
                metadata: serde_json::json!({}),
            },
        ];

        let edges = vec![
            EvidenceEdge {
                id: "edge-low".to_string(),
                from: "node-1".to_string(),
                to: "node-2".to_string(),
                relationship: EdgeType::Supports,
                confidence: 0.2, // Low
                created_at: Utc::now(),
            },
            EvidenceEdge {
                id: "edge-medium".to_string(),
                from: "node-2".to_string(),
                to: "node-3".to_string(),
                relationship: EdgeType::LeadsTo,
                confidence: 0.5, // Medium
                created_at: Utc::now(),
            },
            EvidenceEdge {
                id: "edge-high".to_string(),
                from: "node-3".to_string(),
                to: "node-4".to_string(),
                relationship: EdgeType::Confirms,
                confidence: 0.8, // High
                created_at: Utc::now(),
            },
        ];

        let graph = KnowledgeGraph { nodes, edges };

        let result = transform_to_cytoscape(&graph);

        assert_eq!(result.elements.edges.len(), 3);

        // Low confidence: red
        assert_eq!(result.elements.edges[0].style.line_color, "#EF4444");

        // Medium confidence: orange
        assert_eq!(result.elements.edges[1].style.line_color, "#F59E0B");

        // High confidence: green
        assert_eq!(result.elements.edges[2].style.line_color, "#10B981");
    }

    #[test]
    fn test_metadata_preservation() {
        // Test that all node metadata is fully preserved in transformation
        let complex_metadata = serde_json::json!({
            "tool": "nmap",
            "command": "nmap -sV -A 192.168.1.10",
            "openPorts": [22, 80, 443, 3306],
            "services": {
                "22": "ssh",
                "80": "http",
                "443": "https",
                "3306": "mysql"
            },
            "output": "PORT    STATE SERVICE VERSION\n22/tcp  open  ssh     OpenSSH 7.4",
            "timestamp": "2026-04-14T10:30:00Z",
            "duration": 42.5,
            "discovered": true,
            "null_value": null
        });

        let node = EvidenceNode {
            id: "node-complex".to_string(),
            node_type: NodeType::Evidence,
            title: "Complex metadata test".to_string(),
            description: "Testing metadata preservation".to_string(),
            confidence: 0.95,
            timestamp: Utc::now(),
            created_by: "pick-agent-1".to_string(),
            engagement_id: "eng-1".to_string(),
            target: Some("192.168.1.10".to_string()),
            metadata: complex_metadata.clone(),
        };

        let graph = KnowledgeGraph {
            nodes: vec![node],
            edges: vec![],
        };

        let result = transform_to_cytoscape(&graph);

        assert_eq!(result.elements.nodes.len(), 1);

        let cyto_node = &result.elements.nodes[0];

        // Verify all metadata fields are preserved exactly
        assert_eq!(cyto_node.data.metadata["tool"], "nmap");
        assert_eq!(
            cyto_node.data.metadata["command"],
            "nmap -sV -A 192.168.1.10"
        );
        assert_eq!(
            cyto_node.data.metadata["openPorts"],
            serde_json::json!([22, 80, 443, 3306])
        );
        assert_eq!(cyto_node.data.metadata["services"]["22"], "ssh");
        assert_eq!(cyto_node.data.metadata["duration"], 42.5);
        assert_eq!(cyto_node.data.metadata["discovered"], true);
        assert!(cyto_node.data.metadata["null_value"].is_null());
    }

    #[test]
    fn test_json_serialization() {
        // Test that the output can be serialized to JSON for Cytoscape.js
        let node = EvidenceNode {
            id: "node-1".to_string(),
            node_type: NodeType::Evidence,
            title: "Test node".to_string(),
            description: "Test description".to_string(),
            confidence: 0.95,
            timestamp: Utc::now(),
            created_by: "test".to_string(),
            engagement_id: "eng-1".to_string(),
            target: None,
            metadata: serde_json::json!({ "key": "value" }),
        };

        let graph = KnowledgeGraph {
            nodes: vec![node],
            edges: vec![],
        };

        let result = transform_to_cytoscape(&graph);

        // Serialize to JSON
        let json = serde_json::to_string(&result).expect("Should serialize to JSON");

        // Verify it contains expected keys
        assert!(json.contains("\"elements\""));
        assert!(json.contains("\"nodes\""));
        assert!(json.contains("\"edges\""));
        assert!(json.contains("\"data\""));
        assert!(json.contains("\"style\""));
        assert!(json.contains("\"background-color\""));
        assert!(json.contains("\"border-color\""));

        // Verify it can be deserialized back
        let deserialized: CytoscapeGraph =
            serde_json::from_str(&json).expect("Should deserialize from JSON");
        assert_eq!(deserialized.elements.nodes.len(), 1);
        assert_eq!(deserialized.elements.edges.len(), 0);
    }
}
