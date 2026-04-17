//! Evidence Chain Visualization Module
//!
//! This module provides types and client functionality for fetching and displaying
//! evidence chains from StrikeKit. Evidence chains represent the causal relationships
//! between observations, hypotheses, exploit attempts, and confirmed findings during
//! a penetration test engagement.
//!
//! # Architecture
//!
//! - `types.rs`: Core data structures (nodes, edges, graphs)
//! - `client.rs`: API client for fetching evidence from StrikeKit
//! - `mock.rs`: Mock data generator for development and testing
//! - `transformer.rs`: Transforms API data to visualization format (Cytoscape.js)
//! - `filters.rs`: Filtering logic for evidence graphs
//!
//! # StrikeKit API Contract
//!
//! ## GraphQL Endpoint
//!
//! ```text
//! POST https://{strikekit-host}/graphql
//! Authorization: Bearer {matrix-token}
//! Content-Type: application/json
//! ```
//!
//! ## Query: Get Evidence Chain
//!
//! ```graphql
//! query GetEvidenceChain($engagementId: ID!, $filters: EvidenceFilters) {
//!   evidenceChain(engagementId: $engagementId, filters: $filters) {
//!     nodes {
//!       id
//!       nodeType
//!       title
//!       description
//!       confidence
//!       timestamp
//!       createdBy
//!       engagementId
//!       target
//!       metadata
//!     }
//!     edges {
//!       id
//!       from
//!       to
//!       relationship
//!       confidence
//!       createdAt
//!     }
//!   }
//! }
//! ```
//!
//! ## Input: EvidenceFilters
//!
//! ```json
//! {
//!   "target": "192.168.1.10",           // Optional: filter by IP/hostname
//!   "minConfidence": 0.5,               // Optional: minimum confidence level
//!   "nodeTypes": ["EVIDENCE", "FINDING"], // Optional: filter by node types
//!   "startTime": "2026-04-01T00:00:00Z", // Optional: time range start
//!   "endTime": "2026-04-15T23:59:59Z"    // Optional: time range end
//! }
//! ```
//!
//! ## Response Structure
//!
//! ```json
//! {
//!   "data": {
//!     "evidenceChain": {
//!       "nodes": [
//!         {
//!           "id": "node-uuid-1",
//!           "nodeType": "EVIDENCE",
//!           "title": "Nmap scan completed",
//!           "description": "Port scan discovered 5 open ports on target",
//!           "confidence": 0.95,
//!           "timestamp": "2026-04-14T10:30:00Z",
//!           "createdBy": "pick-agent-1",
//!           "engagementId": "eng-123",
//!           "target": "192.168.1.10",
//!           "metadata": {
//!             "tool": "nmap",
//!             "command": "nmap -sV 192.168.1.10",
//!             "output": "...",
//!             "openPorts": [22, 80, 443, 3306, 8080]
//!           }
//!         },
//!         {
//!           "id": "node-uuid-2",
//!           "nodeType": "HYPOTHESIS",
//!           "title": "SSH service may have weak credentials",
//!           "description": "Port 22 is open with OpenSSH 7.4, known to be vulnerable to brute force",
//!           "confidence": 0.75,
//!           "timestamp": "2026-04-14T10:31:00Z",
//!           "createdBy": "ai-planner",
//!           "engagementId": "eng-123",
//!           "target": "192.168.1.10",
//!           "metadata": {
//!             "reasoning": "OpenSSH 7.4 commonly has weak default configs",
//!             "cve": null,
//!             "mitreAttack": "T1110.001"
//!           }
//!         },
//!         {
//!           "id": "node-uuid-3",
//!           "nodeType": "EXPLOIT_ATTEMPT",
//!           "title": "Hydra SSH brute force",
//!           "description": "Attempted credential brute force against SSH service",
//!           "confidence": 0.80,
//!           "timestamp": "2026-04-14T10:35:00Z",
//!           "createdBy": "pick-agent-1",
//!           "engagementId": "eng-123",
//!           "target": "192.168.1.10",
//!           "metadata": {
//!             "tool": "hydra",
//!             "username": "admin",
//!             "attempts": 1000,
//!             "duration": 45,
//!             "success": true
//!           }
//!         },
//!         {
//!           "id": "node-uuid-4",
//!           "nodeType": "FINDING",
//!           "title": "Weak SSH credentials confirmed",
//!           "description": "Successfully authenticated with admin/password123",
//!           "confidence": 1.0,
//!           "timestamp": "2026-04-14T10:36:00Z",
//!           "createdBy": "pick-agent-1",
//!           "engagementId": "eng-123",
//!           "target": "192.168.1.10",
//!           "metadata": {
//!             "severity": "HIGH",
//!             "cvssScore": 8.8,
//!             "credentials": "admin/password123",
//!             "remediation": "Enforce strong password policy, disable password auth, use key-based auth"
//!           }
//!         }
//!       ],
//!       "edges": [
//!         {
//!           "id": "edge-uuid-1",
//!           "from": "node-uuid-1",
//!           "to": "node-uuid-2",
//!           "relationship": "SUPPORTS",
//!           "confidence": 0.85,
//!           "createdAt": "2026-04-14T10:31:00Z"
//!         },
//!         {
//!           "id": "edge-uuid-2",
//!           "from": "node-uuid-2",
//!           "to": "node-uuid-3",
//!           "relationship": "LEADS_TO",
//!           "confidence": 0.80,
//!           "createdAt": "2026-04-14T10:35:00Z"
//!         },
//!         {
//!           "id": "edge-uuid-3",
//!           "from": "node-uuid-3",
//!           "to": "node-uuid-4",
//!           "relationship": "CONFIRMS",
//!           "confidence": 1.0,
//!           "createdAt": "2026-04-14T10:36:00Z"
//!         }
//!       ]
//!     }
//!   }
//! }
//! ```
//!
//! ## Error Responses
//!
//! ```json
//! {
//!   "errors": [
//!     {
//!       "message": "Engagement not found",
//!       "extensions": {
//!         "code": "NOT_FOUND",
//!         "engagementId": "eng-invalid"
//!       }
//!     }
//!   ]
//! }
//! ```
//!
//! ### Error Codes
//!
//! - `NOT_FOUND`: Engagement ID does not exist
//! - `UNAUTHORIZED`: Invalid or expired authentication token
//! - `FORBIDDEN`: User does not have access to this engagement
//! - `VALIDATION_ERROR`: Invalid filter parameters
//! - `INTERNAL_ERROR`: Server-side error
//!
//! ## Node Types
//!
//! - `EVIDENCE`: Raw observations from tool executions
//! - `HYPOTHESIS`: AI-generated theories about potential vulnerabilities
//! - `EXPLOIT_ATTEMPT`: Actions taken to test hypotheses
//! - `FINDING`: Confirmed vulnerabilities with evidence
//!
//! ## Edge Relationships
//!
//! - `SUPPORTS`: Evidence supports a hypothesis (Evidence → Hypothesis)
//! - `LEADS_TO`: Hypothesis leads to an exploit attempt (Hypothesis → ExploitAttempt)
//! - `CONFIRMS`: Exploit attempt confirms a finding (ExploitAttempt → Finding)
//! - `REFERENCES`: Generic reference between nodes (any → any)
//!
//! ## Confidence Levels
//!
//! Confidence is a float between 0.0 (no confidence) and 1.0 (certain):
//!
//! - `0.0 - 0.3`: Low confidence (red)
//! - `0.3 - 0.7`: Medium confidence (yellow/orange)
//! - `0.7 - 0.9`: High confidence (light green)
//! - `0.9 - 1.0`: Very high confidence (dark green)

pub mod filters;
pub mod mock;
pub mod transformer;
pub mod types;

#[cfg(test)]
mod tests {
    //! Module-level tests for evidence chain functionality
}
