//! Filtering logic for evidence chain graphs
//!
//! This module provides functions to filter KnowledgeGraph nodes and edges
//! based on various criteria defined in EvidenceFilters.

use super::types::{EvidenceFilters, KnowledgeGraph};

/// Apply filters to a knowledge graph, returning a new filtered graph
///
/// # Arguments
///
/// * `graph` - The source knowledge graph to filter
/// * `engagement_id` - Required engagement ID to filter by (exact match)
/// * `filters` - Additional optional filter criteria
///
/// # Returns
///
/// A new KnowledgeGraph containing only nodes and edges that match the filters.
/// If no additional filters are provided (all None), returns all nodes matching
/// the engagement_id. Edges are automatically filtered to only include those
/// where both source and target nodes remain after node filtering.
///
/// # Examples
///
/// ```
/// use pentest_core::evidence::{filters::apply_filters, types::*};
///
/// let graph = KnowledgeGraph::new();
/// let filters = EvidenceFilters::default();
/// let filtered = apply_filters(&graph, "eng-123", &filters);
/// ```
pub fn apply_filters(
    graph: &KnowledgeGraph,
    engagement_id: &str,
    filters: &EvidenceFilters,
) -> KnowledgeGraph {
    // Validate filters - return empty graph for invalid filters
    if let Some(min_confidence) = filters.min_confidence {
        if !(0.0..=1.0).contains(&min_confidence) {
            // Invalid confidence threshold - return empty graph
            return KnowledgeGraph::new();
        }
    }

    // Filter nodes by engagement_id and other criteria
    let filtered_nodes: Vec<_> = graph
        .nodes
        .iter()
        .filter(|node| {
            // Must match engagement_id
            if node.engagement_id != engagement_id {
                return false;
            }

            // Apply target filter (exact or partial match)
            if let Some(ref target_filter) = filters.target {
                if let Some(ref node_target) = node.target {
                    if !node_target.contains(target_filter) {
                        return false;
                    }
                } else {
                    // Node has no target, doesn't match filter
                    return false;
                }
            }

            // Apply minimum confidence filter
            if let Some(min_confidence) = filters.min_confidence {
                if node.confidence < min_confidence {
                    return false;
                }
            }

            // Apply node type filter
            if let Some(ref node_types) = filters.node_types {
                if !node_types.contains(&node.node_type) {
                    return false;
                }
            }

            // Apply time range filter
            if let Some(start_time) = filters.start_time {
                if node.timestamp < start_time {
                    return false;
                }
            }
            if let Some(end_time) = filters.end_time {
                if node.timestamp > end_time {
                    return false;
                }
            }

            true
        })
        .cloned()
        .collect();

    // Build set of remaining node IDs
    let node_ids: std::collections::HashSet<&str> =
        filtered_nodes.iter().map(|n| n.id.as_str()).collect();

    // Filter edges to only include those connecting remaining nodes
    let filtered_edges: Vec<_> = graph
        .edges
        .iter()
        .filter(|edge| node_ids.contains(edge.from.as_str()) && node_ids.contains(edge.to.as_str()))
        .cloned()
        .collect();

    KnowledgeGraph {
        nodes: filtered_nodes,
        edges: filtered_edges,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evidence::mock::{generate_medium_engagement, generate_small_engagement};
    use crate::evidence::types::{EvidenceFilters, NodeType};
    use chrono::Duration;

    #[test]
    fn test_no_filters_returns_full_graph() {
        // Arrange
        let graph = generate_small_engagement();
        let filters = EvidenceFilters::default();

        // Act
        let result = apply_filters(&graph, "eng-small", &filters);

        // Assert
        assert_eq!(result.node_count(), graph.node_count());
        assert_eq!(result.edge_count(), graph.edge_count());
        assert_eq!(result, graph);
    }

    #[test]
    fn test_filter_by_engagement_id_match() {
        // Arrange
        let graph = generate_small_engagement();
        let filters = EvidenceFilters::default();

        // Act
        let result = apply_filters(&graph, "eng-small", &filters);

        // Assert - should return all nodes from the matching engagement
        assert_eq!(result.node_count(), 7);
        assert_eq!(result.edge_count(), 6);
    }

    #[test]
    fn test_filter_by_engagement_id_no_match() {
        // Arrange
        let graph = generate_small_engagement();
        let filters = EvidenceFilters::default();

        // Act
        let result = apply_filters(&graph, "eng-nonexistent", &filters);

        // Assert - should return empty graph
        assert_eq!(result.node_count(), 0);
        assert_eq!(result.edge_count(), 0);
    }

    #[test]
    fn test_filter_by_target_exact_match() {
        // Arrange - medium engagement has multiple targets
        let graph = generate_medium_engagement();
        let filters = EvidenceFilters {
            target: Some("192.168.1.10".to_string()),
            ..Default::default()
        };

        // Act
        let result = apply_filters(&graph, "eng-medium", &filters);

        // Assert - should only return nodes with target "192.168.1.10"
        assert!(result.node_count() > 0);
        assert!(result.node_count() < graph.node_count()); // Should be filtered
        for node in &result.nodes {
            if let Some(ref target) = node.target {
                assert_eq!(target, "192.168.1.10");
            }
        }
    }

    #[test]
    fn test_filter_by_target_partial_match() {
        // Arrange - medium engagement has targets like 192.168.1.10, 192.168.1.20, etc.
        let graph = generate_medium_engagement();
        let filters = EvidenceFilters {
            target: Some("192.168.1".to_string()),
            ..Default::default()
        };

        // Act
        let result = apply_filters(&graph, "eng-medium", &filters);

        // Assert - should match all nodes with target containing "192.168.1"
        assert!(result.node_count() > 0);
        for node in &result.nodes {
            if let Some(ref target) = node.target {
                assert!(target.contains("192.168.1"));
            }
        }
    }

    #[test]
    fn test_filter_by_min_confidence() {
        // Arrange - small engagement has varying confidence levels
        let graph = generate_small_engagement();
        let filters = EvidenceFilters {
            min_confidence: Some(0.8),
            ..Default::default()
        };

        // Act
        let result = apply_filters(&graph, "eng-small", &filters);

        // Assert - should only return nodes with confidence >= 0.8
        assert!(result.node_count() > 0);
        assert!(result.node_count() < graph.node_count()); // Should be filtered
        for node in &result.nodes {
            assert!(node.confidence >= 0.8);
        }
    }

    #[test]
    fn test_filter_by_node_types() {
        // Arrange - small engagement has Evidence, Hypothesis, ExploitAttempt, Finding
        let graph = generate_small_engagement();
        let filters = EvidenceFilters {
            node_types: Some(vec![NodeType::Evidence, NodeType::Finding]),
            ..Default::default()
        };

        // Act
        let result = apply_filters(&graph, "eng-small", &filters);

        // Assert - should only return Evidence and Finding nodes
        assert!(result.node_count() > 0);
        assert!(result.node_count() < graph.node_count()); // Should be filtered
        for node in &result.nodes {
            assert!(node.node_type == NodeType::Evidence || node.node_type == NodeType::Finding);
        }
    }

    #[test]
    fn test_filter_by_time_range() {
        // Arrange - small engagement has nodes created at different times
        let graph = generate_small_engagement();
        let base_time = graph.nodes[0].timestamp;

        let filters = EvidenceFilters {
            start_time: Some(base_time),
            end_time: Some(base_time + Duration::minutes(7)),
            ..Default::default()
        };

        // Act
        let result = apply_filters(&graph, "eng-small", &filters);

        // Assert - should only return nodes within time range
        assert!(result.node_count() > 0);
        assert!(result.node_count() < graph.node_count()); // Should be filtered
        for node in &result.nodes {
            assert!(node.timestamp >= base_time);
            assert!(node.timestamp <= base_time + Duration::minutes(7));
        }
    }

    #[test]
    fn test_combine_multiple_filters() {
        // Arrange - medium engagement with various targets and confidence levels
        let graph = generate_medium_engagement();
        let filters = EvidenceFilters {
            target: Some("192.168.1".to_string()),
            min_confidence: Some(0.75),
            node_types: Some(vec![NodeType::Finding, NodeType::ExploitAttempt]),
            ..Default::default()
        };

        // Act
        let result = apply_filters(&graph, "eng-medium", &filters);

        // Assert - all filters should be applied (AND logic)
        assert!(result.node_count() > 0);
        for node in &result.nodes {
            // Check target filter
            if let Some(ref target) = node.target {
                assert!(target.contains("192.168.1"));
            } else {
                panic!("Node should have target");
            }
            // Check confidence filter
            assert!(node.confidence >= 0.75);
            // Check node type filter
            assert!(
                node.node_type == NodeType::Finding || node.node_type == NodeType::ExploitAttempt
            );
        }
    }

    #[test]
    fn test_edge_filtering_with_nodes() {
        // Arrange - small engagement has connected nodes
        let graph = generate_small_engagement();
        let original_edge_count = graph.edge_count();

        // Filter to only Evidence nodes (should remove many edges)
        let filters = EvidenceFilters {
            node_types: Some(vec![NodeType::Evidence]),
            ..Default::default()
        };

        // Act
        let result = apply_filters(&graph, "eng-small", &filters);

        // Assert - edges should be filtered to only connect remaining nodes
        assert!(result.node_count() > 0);
        assert!(result.edge_count() < original_edge_count); // Should be fewer edges

        // Build set of remaining node IDs
        let node_ids: std::collections::HashSet<&str> =
            result.nodes.iter().map(|n| n.id.as_str()).collect();

        // All edges should connect nodes in the filtered graph
        for edge in &result.edges {
            assert!(node_ids.contains(edge.from.as_str()));
            assert!(node_ids.contains(edge.to.as_str()));
        }
    }

    #[test]
    fn test_edge_removal_when_source_or_target_filtered() {
        // Arrange - small engagement
        let graph = generate_small_engagement();

        // Filter to only Finding nodes (leaves very few nodes)
        let filters = EvidenceFilters {
            node_types: Some(vec![NodeType::Finding]),
            ..Default::default()
        };

        // Act
        let result = apply_filters(&graph, "eng-small", &filters);

        // Assert - should have nodes but no edges (findings don't connect to each other)
        assert!(result.node_count() > 0);

        // Verify all edges connect only to remaining nodes
        let node_ids: std::collections::HashSet<&str> =
            result.nodes.iter().map(|n| n.id.as_str()).collect();

        for edge in &result.edges {
            assert!(node_ids.contains(edge.from.as_str()));
            assert!(node_ids.contains(edge.to.as_str()));
        }
    }

    #[test]
    fn test_invalid_confidence_filter() {
        // Arrange - invalid confidence > 1.0
        let graph = generate_small_engagement();
        let filters = EvidenceFilters {
            min_confidence: Some(1.5),
            ..Default::default()
        };

        // Act
        let result = apply_filters(&graph, "eng-small", &filters);

        // Assert - should return empty graph for invalid filter
        assert_eq!(result.node_count(), 0);
        assert_eq!(result.edge_count(), 0);
    }

    #[test]
    fn test_invalid_confidence_filter_negative() {
        // Arrange - invalid confidence < 0.0
        let graph = generate_small_engagement();
        let filters = EvidenceFilters {
            min_confidence: Some(-0.5),
            ..Default::default()
        };

        // Act
        let result = apply_filters(&graph, "eng-small", &filters);

        // Assert - should return empty graph for invalid filter
        assert_eq!(result.node_count(), 0);
        assert_eq!(result.edge_count(), 0);
    }

    #[test]
    fn test_empty_graph() {
        // Arrange - empty graph
        let graph = KnowledgeGraph::new();
        let filters = EvidenceFilters::default();

        // Act
        let result = apply_filters(&graph, "eng-any", &filters);

        // Assert - should return empty graph
        assert_eq!(result.node_count(), 0);
        assert_eq!(result.edge_count(), 0);
    }

    #[test]
    fn test_original_graph_not_mutated() {
        // Arrange
        let graph = generate_small_engagement();
        let original_node_count = graph.node_count();
        let original_edge_count = graph.edge_count();

        let filters = EvidenceFilters {
            min_confidence: Some(0.9),
            ..Default::default()
        };

        // Act
        let _result = apply_filters(&graph, "eng-small", &filters);

        // Assert - original graph should not be modified
        assert_eq!(graph.node_count(), original_node_count);
        assert_eq!(graph.edge_count(), original_edge_count);
    }
}
