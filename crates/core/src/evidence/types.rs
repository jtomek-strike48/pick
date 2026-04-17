//! Core types for evidence chain visualization

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A complete evidence chain graph containing nodes and edges
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KnowledgeGraph {
    /// All nodes in the graph
    pub nodes: Vec<EvidenceNode>,
    /// All edges connecting nodes
    pub edges: Vec<EvidenceEdge>,
}

/// A single node in the evidence chain
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvidenceNode {
    /// Unique identifier for this node
    pub id: String,

    /// Type of node (Evidence, Hypothesis, ExploitAttempt, Finding)
    #[serde(rename = "nodeType")]
    pub node_type: NodeType,

    /// Short title for the node
    pub title: String,

    /// Detailed description
    pub description: String,

    /// Confidence level (0.0 to 1.0)
    pub confidence: f32,

    /// When this node was created
    pub timestamp: DateTime<Utc>,

    /// Who/what created this node (user, agent, tool)
    #[serde(rename = "createdBy")]
    pub created_by: String,

    /// Which engagement this belongs to
    #[serde(rename = "engagementId")]
    pub engagement_id: String,

    /// Target IP or hostname (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,

    /// Additional metadata specific to node type
    pub metadata: serde_json::Value,
}

/// Type of evidence node
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NodeType {
    /// Raw observation from tool execution
    Evidence,

    /// AI-generated theory about potential vulnerability
    Hypothesis,

    /// Action taken to test a hypothesis
    ExploitAttempt,

    /// Confirmed vulnerability
    Finding,
}

impl NodeType {
    /// Get human-readable label for this node type
    pub fn label(&self) -> &'static str {
        match self {
            Self::Evidence => "Evidence",
            Self::Hypothesis => "Hypothesis",
            Self::ExploitAttempt => "Exploit Attempt",
            Self::Finding => "Finding",
        }
    }

    /// Get color for this node type (used in visualization)
    pub fn color(&self) -> &'static str {
        match self {
            Self::Evidence => "#3B82F6",       // Blue
            Self::Hypothesis => "#F59E0B",     // Orange
            Self::ExploitAttempt => "#8B5CF6", // Purple
            Self::Finding => "#EF4444",        // Red
        }
    }

    /// Get shape for this node type (Cytoscape.js shape)
    pub fn shape(&self) -> &'static str {
        match self {
            Self::Evidence => "ellipse",
            Self::Hypothesis => "diamond",
            Self::ExploitAttempt => "hexagon",
            Self::Finding => "rectangle",
        }
    }
}

/// A directed edge connecting two nodes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvidenceEdge {
    /// Unique identifier for this edge
    pub id: String,

    /// Source node ID
    pub from: String,

    /// Target node ID
    pub to: String,

    /// Type of relationship
    pub relationship: EdgeType,

    /// Confidence in this relationship (0.0 to 1.0)
    pub confidence: f32,

    /// When this edge was created
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
}

/// Type of relationship between nodes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EdgeType {
    /// Evidence supports a hypothesis
    Supports,

    /// Hypothesis leads to an exploit attempt
    LeadsTo,

    /// Exploit attempt confirms a finding
    Confirms,

    /// Generic reference between nodes
    References,
}

impl EdgeType {
    /// Get human-readable label for this edge type
    pub fn label(&self) -> &'static str {
        match self {
            Self::Supports => "supports",
            Self::LeadsTo => "leads to",
            Self::Confirms => "confirms",
            Self::References => "references",
        }
    }
}

/// Filters for querying evidence chains
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EvidenceFilters {
    /// Filter by target IP or hostname
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,

    /// Minimum confidence level (0.0 to 1.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "minConfidence")]
    pub min_confidence: Option<f32>,

    /// Filter by node types
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nodeTypes")]
    pub node_types: Option<Vec<NodeType>>,

    /// Start of time range
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "startTime")]
    pub start_time: Option<DateTime<Utc>>,

    /// End of time range
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "endTime")]
    pub end_time: Option<DateTime<Utc>>,
}

/// GraphQL query variables
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceChainQuery {
    /// Engagement ID to query
    #[serde(rename = "engagementId")]
    pub engagement_id: String,

    /// Optional filters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<EvidenceFilters>,
}

/// GraphQL response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceChainResponse {
    pub data: Option<EvidenceChainData>,
    pub errors: Option<Vec<GraphQLError>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceChainData {
    #[serde(rename = "evidenceChain")]
    pub evidence_chain: KnowledgeGraph,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLError {
    pub message: String,
    pub extensions: Option<HashMap<String, serde_json::Value>>,
}

impl EvidenceNode {
    /// Validate this node's data
    pub fn validate(&self) -> Result<(), String> {
        // Confidence must be between 0.0 and 1.0
        if self.confidence < 0.0 || self.confidence > 1.0 {
            return Err(format!(
                "Confidence {} out of bounds [0.0, 1.0]",
                self.confidence
            ));
        }

        // ID cannot be empty
        if self.id.is_empty() {
            return Err("Node ID cannot be empty".to_string());
        }

        // Title cannot be empty
        if self.title.is_empty() {
            return Err("Node title cannot be empty".to_string());
        }

        // Engagement ID cannot be empty
        if self.engagement_id.is_empty() {
            return Err("Engagement ID cannot be empty".to_string());
        }

        Ok(())
    }

    /// Get color based on confidence level
    pub fn confidence_color(&self) -> &'static str {
        match self.confidence {
            c if c < 0.3 => "#EF4444", // Red (low confidence)
            c if c < 0.7 => "#F59E0B", // Orange (medium confidence)
            c if c < 0.9 => "#10B981", // Green (high confidence)
            _ => "#059669",            // Dark green (very high confidence)
        }
    }
}

impl EvidenceEdge {
    /// Validate this edge's data
    pub fn validate(&self) -> Result<(), String> {
        // Confidence must be between 0.0 and 1.0
        if self.confidence < 0.0 || self.confidence > 1.0 {
            return Err(format!(
                "Edge confidence {} out of bounds [0.0, 1.0]",
                self.confidence
            ));
        }

        // IDs cannot be empty
        if self.id.is_empty() || self.from.is_empty() || self.to.is_empty() {
            return Err("Edge IDs cannot be empty".to_string());
        }

        // Cannot connect to self
        if self.from == self.to {
            return Err(format!("Edge cannot connect node {} to itself", self.from));
        }

        Ok(())
    }
}

impl KnowledgeGraph {
    /// Create an empty graph
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    /// Validate the entire graph
    pub fn validate(&self) -> Result<(), String> {
        // Validate all nodes
        for node in &self.nodes {
            node.validate()?;
        }

        // Validate all edges
        for edge in &self.edges {
            edge.validate()?;
        }

        // Verify all edges reference valid nodes
        let node_ids: std::collections::HashSet<&str> =
            self.nodes.iter().map(|n| n.id.as_str()).collect();

        for edge in &self.edges {
            if !node_ids.contains(edge.from.as_str()) {
                return Err(format!(
                    "Edge {} references unknown node {}",
                    edge.id, edge.from
                ));
            }
            if !node_ids.contains(edge.to.as_str()) {
                return Err(format!(
                    "Edge {} references unknown node {}",
                    edge.id, edge.to
                ));
            }
        }

        Ok(())
    }

    /// Get node count
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Get edge count
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Find a node by ID
    pub fn find_node(&self, id: &str) -> Option<&EvidenceNode> {
        self.nodes.iter().find(|n| n.id == id)
    }
}

impl Default for KnowledgeGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_validation() {
        let mut node = EvidenceNode {
            id: "test-1".to_string(),
            node_type: NodeType::Evidence,
            title: "Test Node".to_string(),
            description: "Test description".to_string(),
            confidence: 0.8,
            timestamp: Utc::now(),
            created_by: "test".to_string(),
            engagement_id: "eng-1".to_string(),
            target: None,
            metadata: serde_json::json!({}),
        };

        // Valid node
        assert!(node.validate().is_ok());

        // Invalid confidence
        node.confidence = 1.5;
        assert!(node.validate().is_err());
        node.confidence = 0.8;

        // Empty ID
        node.id = "".to_string();
        assert!(node.validate().is_err());
    }

    #[test]
    fn test_edge_validation() {
        let mut edge = EvidenceEdge {
            id: "edge-1".to_string(),
            from: "node-1".to_string(),
            to: "node-2".to_string(),
            relationship: EdgeType::Supports,
            confidence: 0.9,
            created_at: Utc::now(),
        };

        // Valid edge
        assert!(edge.validate().is_ok());

        // Self-reference
        edge.to = "node-1".to_string();
        assert!(edge.validate().is_err());
    }

    #[test]
    fn test_graph_validation() {
        let node1 = EvidenceNode {
            id: "node-1".to_string(),
            node_type: NodeType::Evidence,
            title: "Node 1".to_string(),
            description: "Desc".to_string(),
            confidence: 0.9,
            timestamp: Utc::now(),
            created_by: "test".to_string(),
            engagement_id: "eng-1".to_string(),
            target: None,
            metadata: serde_json::json!({}),
        };

        let node2 = EvidenceNode {
            id: "node-2".to_string(),
            ..node1.clone()
        };

        let edge = EvidenceEdge {
            id: "edge-1".to_string(),
            from: "node-1".to_string(),
            to: "node-2".to_string(),
            relationship: EdgeType::Supports,
            confidence: 0.9,
            created_at: Utc::now(),
        };

        let mut graph = KnowledgeGraph {
            nodes: vec![node1, node2],
            edges: vec![edge],
        };

        // Valid graph
        assert!(graph.validate().is_ok());

        // Edge referencing non-existent node
        graph.edges[0].to = "node-3".to_string();
        assert!(graph.validate().is_err());
    }

    #[test]
    fn test_node_type_colors() {
        assert_eq!(NodeType::Evidence.color(), "#3B82F6");
        assert_eq!(NodeType::Hypothesis.color(), "#F59E0B");
        assert_eq!(NodeType::ExploitAttempt.color(), "#8B5CF6");
        assert_eq!(NodeType::Finding.color(), "#EF4444");
    }

    #[test]
    fn test_confidence_colors() {
        let node = EvidenceNode {
            id: "test-1".to_string(),
            node_type: NodeType::Evidence,
            title: "Test".to_string(),
            description: "Test".to_string(),
            confidence: 0.2,
            timestamp: Utc::now(),
            created_by: "test".to_string(),
            engagement_id: "eng-1".to_string(),
            target: None,
            metadata: serde_json::json!({}),
        };

        assert_eq!(node.confidence_color(), "#EF4444"); // Low confidence - red
    }
}
