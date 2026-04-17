//! Mock data generator for evidence chains
//!
//! Provides realistic sample data for development and testing. Includes three
//! sample engagements of varying complexity.

use super::types::*;
use chrono::{Duration, Utc};

/// Generate a small sample engagement (10 nodes)
pub fn generate_small_engagement() -> KnowledgeGraph {
    let base_time = Utc::now() - Duration::hours(2);

    let nodes = vec![
        // Initial nmap scan
        EvidenceNode {
            id: "node-1".to_string(),
            node_type: NodeType::Evidence,
            title: "Nmap port scan completed".to_string(),
            description: "Discovered 3 open ports on target".to_string(),
            confidence: 0.95,
            timestamp: base_time,
            created_by: "pick-agent-1".to_string(),
            engagement_id: "eng-small".to_string(),
            target: Some("192.168.1.10".to_string()),
            metadata: serde_json::json!({
                "tool": "nmap",
                "command": "nmap -sV 192.168.1.10",
                "openPorts": [22, 80, 443],
                "output": "PORT    STATE SERVICE VERSION\n22/tcp  open  ssh     OpenSSH 7.4\n80/tcp  open  http    Apache 2.4.6\n443/tcp open  https   Apache 2.4.6"
            }),
        },
        // Hypothesis: weak SSH creds
        EvidenceNode {
            id: "node-2".to_string(),
            node_type: NodeType::Hypothesis,
            title: "SSH may have weak credentials".to_string(),
            description: "OpenSSH 7.4 often has default or weak passwords".to_string(),
            confidence: 0.70,
            timestamp: base_time + Duration::minutes(1),
            created_by: "ai-planner".to_string(),
            engagement_id: "eng-small".to_string(),
            target: Some("192.168.1.10".to_string()),
            metadata: serde_json::json!({
                "reasoning": "OpenSSH 7.4 is commonly deployed with weak default configs",
                "mitreAttack": "T1110.001"
            }),
        },
        // Exploit attempt: Hydra brute force
        EvidenceNode {
            id: "node-3".to_string(),
            node_type: NodeType::ExploitAttempt,
            title: "Hydra SSH brute force".to_string(),
            description: "Attempting credential brute force against SSH".to_string(),
            confidence: 0.80,
            timestamp: base_time + Duration::minutes(5),
            created_by: "pick-agent-1".to_string(),
            engagement_id: "eng-small".to_string(),
            target: Some("192.168.1.10".to_string()),
            metadata: serde_json::json!({
                "tool": "hydra",
                "username": "admin",
                "attempts": 1000,
                "duration": 45,
                "success": true
            }),
        },
        // Finding: weak SSH password
        EvidenceNode {
            id: "node-4".to_string(),
            node_type: NodeType::Finding,
            title: "Weak SSH credentials confirmed".to_string(),
            description: "Successfully authenticated with admin/password123".to_string(),
            confidence: 1.0,
            timestamp: base_time + Duration::minutes(6),
            created_by: "pick-agent-1".to_string(),
            engagement_id: "eng-small".to_string(),
            target: Some("192.168.1.10".to_string()),
            metadata: serde_json::json!({
                "severity": "HIGH",
                "cvssScore": 8.8,
                "credentials": "admin/password123",
                "remediation": "Enforce strong password policy, disable password auth"
            }),
        },
        // Hypothesis: web server may be vulnerable
        EvidenceNode {
            id: "node-5".to_string(),
            node_type: NodeType::Hypothesis,
            title: "Apache 2.4.6 may be outdated".to_string(),
            description: "Apache 2.4.6 has known CVEs".to_string(),
            confidence: 0.65,
            timestamp: base_time + Duration::minutes(2),
            created_by: "ai-planner".to_string(),
            engagement_id: "eng-small".to_string(),
            target: Some("192.168.1.10".to_string()),
            metadata: serde_json::json!({
                "reasoning": "Apache 2.4.6 released 2013, likely has CVEs",
                "cve": "CVE-2017-15710"
            }),
        },
        // Evidence: Nikto scan
        EvidenceNode {
            id: "node-6".to_string(),
            node_type: NodeType::Evidence,
            title: "Nikto web vulnerability scan".to_string(),
            description: "Found directory listing enabled".to_string(),
            confidence: 0.90,
            timestamp: base_time + Duration::minutes(10),
            created_by: "pick-agent-1".to_string(),
            engagement_id: "eng-small".to_string(),
            target: Some("192.168.1.10".to_string()),
            metadata: serde_json::json!({
                "tool": "nikto",
                "findings": ["Directory listing enabled on /uploads", "Missing security headers"]
            }),
        },
        // Finding: information disclosure
        EvidenceNode {
            id: "node-7".to_string(),
            node_type: NodeType::Finding,
            title: "Information disclosure via directory listing".to_string(),
            description: "/uploads directory lists sensitive files".to_string(),
            confidence: 0.95,
            timestamp: base_time + Duration::minutes(11),
            created_by: "pick-agent-1".to_string(),
            engagement_id: "eng-small".to_string(),
            target: Some("192.168.1.10".to_string()),
            metadata: serde_json::json!({
                "severity": "MEDIUM",
                "cvssScore": 5.3,
                "url": "http://192.168.1.10/uploads",
                "remediation": "Disable directory listing in Apache config"
            }),
        },
    ];

    let edges = vec![
        EvidenceEdge {
            id: "edge-1".to_string(),
            from: "node-1".to_string(),
            to: "node-2".to_string(),
            relationship: EdgeType::Supports,
            confidence: 0.85,
            created_at: base_time + Duration::minutes(1),
        },
        EvidenceEdge {
            id: "edge-2".to_string(),
            from: "node-2".to_string(),
            to: "node-3".to_string(),
            relationship: EdgeType::LeadsTo,
            confidence: 0.80,
            created_at: base_time + Duration::minutes(5),
        },
        EvidenceEdge {
            id: "edge-3".to_string(),
            from: "node-3".to_string(),
            to: "node-4".to_string(),
            relationship: EdgeType::Confirms,
            confidence: 1.0,
            created_at: base_time + Duration::minutes(6),
        },
        EvidenceEdge {
            id: "edge-4".to_string(),
            from: "node-1".to_string(),
            to: "node-5".to_string(),
            relationship: EdgeType::Supports,
            confidence: 0.75,
            created_at: base_time + Duration::minutes(2),
        },
        EvidenceEdge {
            id: "edge-5".to_string(),
            from: "node-5".to_string(),
            to: "node-6".to_string(),
            relationship: EdgeType::LeadsTo,
            confidence: 0.85,
            created_at: base_time + Duration::minutes(10),
        },
        EvidenceEdge {
            id: "edge-6".to_string(),
            from: "node-6".to_string(),
            to: "node-7".to_string(),
            relationship: EdgeType::Confirms,
            confidence: 0.95,
            created_at: base_time + Duration::minutes(11),
        },
    ];

    KnowledgeGraph { nodes, edges }
}

/// Generate a medium sample engagement (50 nodes)
pub fn generate_medium_engagement() -> KnowledgeGraph {
    let base_time = Utc::now() - Duration::hours(6);
    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    // Network discovery
    nodes.push(EvidenceNode {
        id: "m-node-1".to_string(),
        node_type: NodeType::Evidence,
        title: "Network discovery scan".to_string(),
        description: "Discovered 5 live hosts on network".to_string(),
        confidence: 0.98,
        timestamp: base_time,
        created_by: "pick-agent-1".to_string(),
        engagement_id: "eng-medium".to_string(),
        target: Some("192.168.1.0/24".to_string()),
        metadata: serde_json::json!({
            "tool": "nmap",
            "liveHosts": ["192.168.1.10", "192.168.1.20", "192.168.1.30", "192.168.1.40", "192.168.1.50"]
        }),
    });

    // Generate hypotheses and findings for each host
    let hosts = vec![
        ("192.168.1.10", "SSH weak creds", 8.8),
        ("192.168.1.20", "SMB null session", 6.5),
        ("192.168.1.30", "MySQL default creds", 9.1),
        ("192.168.1.40", "RDP exposed", 7.5),
        ("192.168.1.50", "Apache directory listing", 5.3),
    ];

    for (idx, (host, vuln, cvss)) in hosts.iter().enumerate() {
        let offset = Duration::minutes((idx * 10) as i64);

        // Hypothesis
        let hyp_id = format!("m-node-{}", idx * 10 + 2);
        nodes.push(EvidenceNode {
            id: hyp_id.clone(),
            node_type: NodeType::Hypothesis,
            title: format!("{} on {}", vuln, host),
            description: format!("Suspected vulnerability: {}", vuln),
            confidence: 0.70,
            timestamp: base_time + offset,
            created_by: "ai-planner".to_string(),
            engagement_id: "eng-medium".to_string(),
            target: Some(host.to_string()),
            metadata: serde_json::json!({ "reasoning": format!("{} commonly vulnerable", host) }),
        });

        edges.push(EvidenceEdge {
            id: format!("m-edge-{}", idx * 3 + 1),
            from: "m-node-1".to_string(),
            to: hyp_id.clone(),
            relationship: EdgeType::Supports,
            confidence: 0.80,
            created_at: base_time + offset,
        });

        // Exploit attempt
        let exploit_id = format!("m-node-{}", idx * 10 + 3);
        nodes.push(EvidenceNode {
            id: exploit_id.clone(),
            node_type: NodeType::ExploitAttempt,
            title: format!("Testing {} on {}", vuln, host),
            description: format!("Exploit attempt for {}", vuln),
            confidence: 0.75,
            timestamp: base_time + offset + Duration::minutes(3),
            created_by: "pick-agent-1".to_string(),
            engagement_id: "eng-medium".to_string(),
            target: Some(host.to_string()),
            metadata: serde_json::json!({ "tool": "metasploit" }),
        });

        edges.push(EvidenceEdge {
            id: format!("m-edge-{}", idx * 3 + 2),
            from: hyp_id.clone(),
            to: exploit_id.clone(),
            relationship: EdgeType::LeadsTo,
            confidence: 0.75,
            created_at: base_time + offset + Duration::minutes(3),
        });

        // Finding
        let finding_id = format!("m-node-{}", idx * 10 + 4);
        nodes.push(EvidenceNode {
            id: finding_id.clone(),
            node_type: NodeType::Finding,
            title: format!("{} confirmed on {}", vuln, host),
            description: format!("Vulnerability {} confirmed", vuln),
            confidence: 0.95,
            timestamp: base_time + offset + Duration::minutes(5),
            created_by: "pick-agent-1".to_string(),
            engagement_id: "eng-medium".to_string(),
            target: Some(host.to_string()),
            metadata: serde_json::json!({
                "severity": if *cvss > 8.0 { "CRITICAL" } else if *cvss > 6.0 { "HIGH" } else { "MEDIUM" },
                "cvssScore": cvss
            }),
        });

        edges.push(EvidenceEdge {
            id: format!("m-edge-{}", idx * 3 + 3),
            from: exploit_id.clone(),
            to: finding_id.clone(),
            relationship: EdgeType::Confirms,
            confidence: 0.95,
            created_at: base_time + offset + Duration::minutes(5),
        });
    }

    // Add some cross-references between findings (lateral movement)
    nodes.push(EvidenceNode {
        id: "m-node-100".to_string(),
        node_type: NodeType::Hypothesis,
        title: "Lateral movement possible".to_string(),
        description: "Multiple compromised hosts enable lateral movement".to_string(),
        confidence: 0.85,
        timestamp: base_time + Duration::hours(1),
        created_by: "ai-planner".to_string(),
        engagement_id: "eng-medium".to_string(),
        target: None,
        metadata: serde_json::json!({ "mitreAttack": "T1021" }),
    });

    edges.push(EvidenceEdge {
        id: "m-edge-100".to_string(),
        from: "m-node-4".to_string(),
        to: "m-node-100".to_string(),
        relationship: EdgeType::References,
        confidence: 0.85,
        created_at: base_time + Duration::hours(1),
    });

    edges.push(EvidenceEdge {
        id: "m-edge-101".to_string(),
        from: "m-node-14".to_string(),
        to: "m-node-100".to_string(),
        relationship: EdgeType::References,
        confidence: 0.85,
        created_at: base_time + Duration::hours(1),
    });

    KnowledgeGraph { nodes, edges }
}

/// Generate a large sample engagement (100 nodes)
pub fn generate_large_engagement() -> KnowledgeGraph {
    let base_time = Utc::now() - Duration::days(1);
    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    // Initial reconnaissance
    for i in 0..20 {
        nodes.push(EvidenceNode {
            id: format!("l-node-{}", i),
            node_type: NodeType::Evidence,
            title: format!("Recon scan {}", i),
            description: format!("Reconnaissance scan {} completed", i),
            confidence: 0.90 + (i as f32 * 0.01) % 0.1,
            timestamp: base_time + Duration::minutes(i * 5),
            created_by: "pick-agent-1".to_string(),
            engagement_id: "eng-large".to_string(),
            target: Some(format!("10.0.{}.0/24", i)),
            metadata: serde_json::json!({ "tool": "nmap" }),
        });
    }

    // Hypotheses
    for i in 20..50 {
        let parent = format!("l-node-{}", i % 20);
        let current = format!("l-node-{}", i);

        nodes.push(EvidenceNode {
            id: current.clone(),
            node_type: NodeType::Hypothesis,
            title: format!("Hypothesis {}", i - 20),
            description: format!("Potential vulnerability {}", i - 20),
            confidence: 0.60 + ((i as f32 * 13.0) % 30.0) / 100.0,
            timestamp: base_time + Duration::minutes(i * 5),
            created_by: "ai-planner".to_string(),
            engagement_id: "eng-large".to_string(),
            target: Some(format!("10.0.{}.1", i % 20)),
            metadata: serde_json::json!({}),
        });

        edges.push(EvidenceEdge {
            id: format!("l-edge-{}", i - 20),
            from: parent,
            to: current,
            relationship: EdgeType::Supports,
            confidence: 0.75,
            created_at: base_time + Duration::minutes(i * 5),
        });
    }

    // Exploit attempts
    for i in 50..80 {
        let parent = format!("l-node-{}", i - 30);
        let current = format!("l-node-{}", i);

        nodes.push(EvidenceNode {
            id: current.clone(),
            node_type: NodeType::ExploitAttempt,
            title: format!("Exploit attempt {}", i - 50),
            description: format!("Testing exploit {}", i - 50),
            confidence: 0.70,
            timestamp: base_time + Duration::minutes(i * 5),
            created_by: "pick-agent-1".to_string(),
            engagement_id: "eng-large".to_string(),
            target: Some(format!("10.0.{}.1", i % 20)),
            metadata: serde_json::json!({ "tool": "metasploit" }),
        });

        edges.push(EvidenceEdge {
            id: format!("l-edge-{}", i - 20),
            from: parent,
            to: current,
            relationship: EdgeType::LeadsTo,
            confidence: 0.70,
            created_at: base_time + Duration::minutes(i * 5),
        });
    }

    // Findings
    for i in 80..100 {
        let parent = format!("l-node-{}", i - 30);
        let current = format!("l-node-{}", i);

        nodes.push(EvidenceNode {
            id: current.clone(),
            node_type: NodeType::Finding,
            title: format!("Finding {}", i - 80),
            description: format!("Confirmed vulnerability {}", i - 80),
            confidence: 0.90 + (i as f32 * 0.01) % 0.1,
            timestamp: base_time + Duration::minutes(i * 5),
            created_by: "pick-agent-1".to_string(),
            engagement_id: "eng-large".to_string(),
            target: Some(format!("10.0.{}.1", i % 20)),
            metadata: serde_json::json!({
                "severity": if i % 3 == 0 { "CRITICAL" } else if i % 2 == 0 { "HIGH" } else { "MEDIUM" },
                "cvssScore": 5.0 + ((i as f32 * 7.0) % 5.0)
            }),
        });

        edges.push(EvidenceEdge {
            id: format!("l-edge-{}", i - 20),
            from: parent,
            to: current,
            relationship: EdgeType::Confirms,
            confidence: 0.90,
            created_at: base_time + Duration::minutes(i * 5),
        });
    }

    KnowledgeGraph { nodes, edges }
}

/// Mock API client that returns sample data
pub struct MockEvidenceClient;

impl MockEvidenceClient {
    pub fn new() -> Self {
        Self
    }

    /// Fetch evidence chain by engagement ID
    pub async fn fetch_evidence_chain(
        &self,
        engagement_id: &str,
        _filters: Option<EvidenceFilters>,
    ) -> Result<KnowledgeGraph, String> {
        // Simulate network delay
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

        match engagement_id {
            "eng-small" => Ok(generate_small_engagement()),
            "eng-medium" => Ok(generate_medium_engagement()),
            "eng-large" => Ok(generate_large_engagement()),
            _ => Err(format!("Engagement {} not found", engagement_id)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_small_engagement() {
        let graph = generate_small_engagement();
        assert_eq!(graph.node_count(), 7);
        assert_eq!(graph.edge_count(), 6);
        assert!(graph.validate().is_ok());
    }

    #[test]
    fn test_generate_medium_engagement() {
        let graph = generate_medium_engagement();
        assert!(graph.node_count() > 10);
        assert!(graph.validate().is_ok());
    }

    #[test]
    fn test_generate_large_engagement() {
        let graph = generate_large_engagement();
        assert_eq!(graph.node_count(), 100);
        assert!(graph.validate().is_ok());
    }

    #[tokio::test]
    async fn test_mock_client() {
        let client = MockEvidenceClient::new();
        let result = client.fetch_evidence_chain("eng-small", None).await;
        assert!(result.is_ok());

        let graph = result.unwrap();
        assert_eq!(graph.node_count(), 7);
    }

    #[tokio::test]
    async fn test_mock_client_not_found() {
        let client = MockEvidenceClient::new();
        let result = client.fetch_evidence_chain("eng-invalid", None).await;
        assert!(result.is_err());
    }
}
