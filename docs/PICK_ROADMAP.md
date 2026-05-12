# Pick Platform Roadmap (Q2-Q3 2026)

## Executive Summary

Transform Pick from a capable penetration testing connector into a **production-ready, enterprise-grade security testing platform** with robust report generation, stable authentication, and comprehensive tool integration. By Q3 2026, Pick will deliver reliable autonomous pentesting workflows with transparent evidence chains and professional reporting.

### Current State (May 2026)

Pick v0.1.3+ is operational with:

- Cross-platform agent (desktop, mobile, headless modes)
- 80+ integrated security tools with gRPC orchestration
- Matrix connector for team collaboration
- Evidence collection and storage
- Basic report generation (MDX format)
- WiFi AutoPwn proof-of-concept
- REST API for scan management

**We have a strong foundation. Now we stabilize and scale.**

### What We're Building Next

Three strategic priorities will move Pick from alpha to production-ready:

**1. Report Quality & Reliability** - Professional-grade reports with provable evidence chains, stable finding IDs, and reproducible probe commands

**2. Authentication & Connection Stability** - Robust OAuth flows, graceful failure handling, and reliable reconnection logic

**3. Platform Expansion** - Official Kali support, WebKitGTK 6.0 upgrade, expanded tool library

---

## Issue Organization by Priority

### P0 - Critical (Now!)

| Issue | Area | Status | Target |
|-------|------|--------|--------|
| #52 | Report Quality | Backlog | Week 1-2 |
| #51 | Report Quality | Backlog | Week 1-2 |

**Focus:** These block enterprise adoption and StrikeKit integration quality.

### P1 - High Priority (Very Soon)

| Issue | Area | Status | Target |
|-------|------|--------|--------|
| #85 | Authentication | Backlog | Week 3-4 |
| #58 | Report Quality | Backlog | Week 3-4 |
| #54 | Report Quality | Backlog | Week 5-6 |
| #53 | Report Quality | Backlog | Week 5-6 |
| #44 | Tool Integration | Backlog | Week 7-10 |
| #42 | StrikeKit Integration | Backlog | Week 7-10 |

**Focus:** Report quality improvements and critical authentication fix.

### P2 - Medium Priority (Next Up)

| Issue | Area | Status | Target |
|-------|------|--------|--------|
| #84 | Authentication | Backlog | Week 11-12 |
| #87 | UI Bug | Backlog | Week 11-12 |
| #57 | Report Quality | Backlog | Week 13-14 |
| #56 | Report Quality | Backlog | Week 13-14 |
| #55 | Report Quality | Backlog | Week 13-14 |
| #43 | StrikeKit Integration | Backlog | Week 15-16 |
| #41 | WiFi AutoPwn | Backlog | Week 15-16 |
| #40 | Post-Exploitation | Backlog | Week 17-18 |
| #35 | File Management | Backlog | Week 17-18 |
| #12 | Security | Backlog | Week 17-18 |

### P3 - Lower Priority (Backlog)

| Issue | Area | Status | Target |
|-------|------|--------|--------|
| #82 | Authentication | Backlog | Week 19-20 |
| #81 | Authentication | Backlog | Week 19-20 |
| #75 | Platform Support | Backlog | Week 21-22 |
| #74 | Platform Support | Backlog | Week 23-24 |
| #64 | Tool Library Epic | Backlog | Ongoing |
| #30 | Platform Support | Backlog | Future |

---

## Strategic Roadmap (18 Weeks)

### Phase 1: Report Quality Foundation (Weeks 1-6)

**Goal:** Deliver professional-grade reports with reproducible evidence

#### Weeks 1-2: P0 Critical Fixes

**#52: Tool provenance & reproducible probe commands**
- Add tool version tracking to all executions
- Store full command line with environment context
- Include working directory, timestamp, operator
- Generate reproducible command blocks in reports
- **Success Criteria:** Any finding can be re-executed from report alone

**#51: Post-validation report re-synthesis**
- Implement report regeneration after severity changes
- Preserve user annotations during re-synthesis
- Add version history tracking
- **Success Criteria:** Severity changes trigger clean report updates without data loss

**Deliverables:**
- Provenance schema in evidence database
- Report versioning system
- Re-synthesis workflow engine
- Unit tests for versioning logic

#### Weeks 3-4: Severity & Audit Trail

**#58: Severity-change audit trail + Summary of Changes box**
- Track who changed severity, when, and why
- Add "Summary of Changes" section to reports
- Include change history in metadata
- **Success Criteria:** Auditable trail for all finding modifications

**Additional:**
- Report template improvements
- Evidence attachment validation
- Finding deduplication enhancements

#### Weeks 5-6: Evidence & Attack Chains

**#54: Per-finding probe method & live evidence in appendix**
- Add "How This Was Tested" section per finding
- Include screenshots, command output in appendix
- Link evidence to specific findings
- **Success Criteria:** Every finding has reproducible test procedure

**#53: Attack chain classification (3-state)**
- Classify findings as: Confirmed Exploit / Probable Vulnerability / Potential Issue
- Add confidence scoring (0-100)
- Visual indicators in reports
- **Success Criteria:** Clear risk communication to clients

**Deliverables:**
- Attack chain schema
- Confidence scoring engine
- Enhanced report templates with evidence appendices
- Integration tests for complete report flow

**Phase 1 Success Gate:**
- All P0 issues resolved
- Reports include tool provenance and reproducible commands
- Severity changes tracked with audit trail
- Attack chains classified with confidence scores

---

### Phase 2: Authentication & Stability (Weeks 7-12)

**Goal:** Robust authentication flows and graceful failure handling

#### Weeks 7-8: Chat/Device Info Regression

**#85: Desktop Chat/Device Info regression (Matrix session token)**
- Fix ChatPanel access to Matrix session when browser-auth disabled
- Implement secure token passing between components
- Add session state synchronization
- **Success Criteria:** Chat and device info work in all authentication modes

#### Weeks 9-10: Connection Resilience

**#84: Connector wedging on failed OTT redemption**
- Add retry logic with exponential backoff
- Surface authentication errors to user
- Implement automatic token refresh
- Add connection health monitoring
- **Success Criteria:** Failed auth attempts don't wedge connector, user sees clear errors

**#87: Parent directory navigation malformed URL**
- Fix trailing # in /reports parent navigation
- Add URL validation tests
- Audit all file browser navigation paths
- **Success Criteria:** Clean URLs in all file navigation scenarios

#### Weeks 11-12: Startup & OAuth Hardening

**#82: Startup stalls on stale credentials (lower priority)**
- Add credential validation before full startup
- Implement timeout for auth URL checks
- Show progress indicator during startup
- **Success Criteria:** Startup fails fast with clear error on stale credentials

**#81: OAuth callback port conflicts (lower priority)**
- Implement configurable OAuth callback ports
- Add CORS configuration validation
- Fallback port selection logic
- **Success Criteria:** OAuth works even when default ports busy

**Deliverables:**
- Refactored authentication module
- Retry and exponential backoff logic
- Connection health monitoring dashboard
- Integration tests for auth failure scenarios

**Phase 2 Success Gate:**
- Chat/Device Info working in all modes
- No connector wedging on auth failures
- Clean URL navigation throughout file browser
- Graceful handling of stale credentials

---

### Phase 3: StrikeKit Integration & Tool Expansion (Weeks 13-18)

**Goal:** Seamless StrikeKit integration and expanded tool ecosystem

#### Weeks 13-14: Report Quality Enhancements

**#57: Infrastructure profile synthesis section**
- Auto-generate infrastructure overview from recon data
- Network topology visualization
- Service distribution charts
- **Success Criteria:** Reports include comprehensive infrastructure summary

**#56: Controls In Place section**
- Detect defensive controls (WAF, IDS, EDR)
- Document observed security measures
- Add to report as dedicated section
- **Success Criteria:** Reports highlight client's existing security posture

**#55: Stable finding IDs across re-synthesis**
- Implement deterministic finding ID generation
- Preserve IDs across report regeneration
- Add ID migration logic for schema changes
- **Success Criteria:** Finding IDs stable across report updates

#### Weeks 15-16: StrikeKit Integration

**#43: Display Knowledge Graph from StrikeKit**
- Integrate with StrikeKit knowledge graph API
- Add graph visualization component
- Link evidence chains to graph nodes
- **Success Criteria:** Evidence chains visible in interactive graph

**#42: Integrate StrikeKit Evidence Chain APIs**
- Implement Evidence Chain schema in Pick
- Auto-link findings to evidence chains
- Bidirectional sync with StrikeKit
- **Success Criteria:** Evidence chains flow seamlessly between Pick and StrikeKit

#### Weeks 17-18: UI Polish & Security

**#41: Polish WiFi AutoPwn UI/UX**
- Improve workflow clarity
- Add progress indicators
- Better error messages
- **Success Criteria:** WiFi AutoPwn UX matches manual workflow quality

**#40: Post-Exploitation Tool UI Enhancements**
- Streamline post-exploit workflows
- Add credential management UI
- Improve pivot tracking
- **Success Criteria:** Post-exploit tools have consistent, polished UX

**#35: Report created but file not in Pick files**
- Debug file registration issue
- Add file integrity checks
- Improve error reporting
- **Success Criteria:** All generated reports visible in file browser

**#12: Add cargo-deny and resolve advisory ignores**
- Configure cargo-deny for security audits
- Resolve known dependency vulnerabilities
- Add to CI/CD pipeline
- **Success Criteria:** cargo-deny runs in CI with zero unresolved advisories

**Deliverables:**
- Enhanced report templates with infrastructure and controls sections
- StrikeKit Evidence Chain integration
- Knowledge graph visualization component
- Polished WiFi AutoPwn and post-exploit UIs
- Security audit tooling in CI/CD

**Phase 3 Success Gate:**
- Reports include infrastructure and controls sections
- Evidence chains integrated with StrikeKit
- WiFi AutoPwn and post-exploit UIs polished
- cargo-deny running clean in CI

---

### Phase 4: Platform Expansion (Weeks 19-24)

**Goal:** Expand platform support and tool ecosystem

#### Weeks 19-20: Authentication Edge Cases

**#82: Startup stalls on stale credentials**
- (Completed in Phase 2 if time, otherwise here)

**#81: OAuth callback port conflicts**
- (Completed in Phase 2 if time, otherwise here)

#### Weeks 21-22: Kali Linux Support

**#75: Add official Kali Linux support**
- Test Pick on Kali Linux
- Fix platform-specific issues
- Add Kali installation guide
- Update CI/CD for Kali testing
- **Success Criteria:** Pick installs and runs cleanly on Kali Linux

#### Weeks 23-24: Ubuntu 26.04 Support

**#74: Upgrade to WebKitGTK 6.0 for Ubuntu 26.04+ support**
- Migrate from WebKitGTK 4.x to 6.0
- Test on Ubuntu 26.04 beta
- Update dependencies and build scripts
- **Success Criteria:** Pick supports Ubuntu 26.04 LTS

#### Ongoing: Tool Library Expansion

**#64: Epic: Expand Tool Library with BlackArch Tools**
- Add 20+ tools from BlackArch repository
- Focus on quality over quantity (AI orchestration)
- Ensure proper evidence parsing for each tool
- **Success Criteria:** Tool count grows from 80 to 100+ with high-quality integrations

**Phase 4 Deliverables:**
- Official Kali Linux support with documentation
- WebKitGTK 6.0 migration complete
- BlackArch tool integration underway
- Comprehensive platform testing across Ubuntu, Kali, and future macOS

**Phase 4 Success Gate:**
- Pick runs on Kali Linux and Ubuntu 26.04
- Tool library expanded with 20+ quality integrations
- All authentication edge cases resolved

---

## Success Metrics (Q3 2026 Targets)

| Metric | Target | Why It Matters |
|--------|--------|----------------|
| Report Quality Score | 95%+ client satisfaction | Core value proposition for enterprise |
| Auth Failure Rate | <1% of connections | Reliability critical for production use |
| Platform Coverage | Ubuntu 24.04, 26.04, Kali | Broad adoption across pentest distros |
| Tool Integration Count | 100+ tools | Comprehensive toolkit for all scenarios |
| Evidence Chain Completeness | 100% findings linked | Full traceability for StrikeKit integration |
| Uptime (connector stability) | 99.5%+ | Production-grade reliability |

---

## Technical Architecture

### Evidence Chain Schema

```rust
pub struct EvidenceChain {
    pub id: Id,
    pub finding_id: Id,
    pub evidence_items: Vec<EvidenceItem>,
    pub confidence_score: f32,
    pub classification: AttackChainClassification,
}

pub enum AttackChainClassification {
    ConfirmedExploit,      // Exploited successfully
    ProbableVulnerability, // High confidence finding
    PotentialIssue,        // Needs validation
}

pub struct EvidenceItem {
    pub id: Id,
    pub item_type: EvidenceType,
    pub tool: String,
    pub tool_version: String,
    pub command: String,
    pub output: String,
    pub timestamp: DateTime<Utc>,
    pub operator: String,
    pub working_directory: PathBuf,
}
```

### Report Versioning System

```rust
pub struct ReportVersion {
    pub id: Id,
    pub report_id: Id,
    pub version: u32,
    pub generated_at: DateTime<Utc>,
    pub changes: Vec<ReportChange>,
    pub file_path: PathBuf,
}

pub struct ReportChange {
    pub change_type: ChangeType,
    pub user: String,
    pub timestamp: DateTime<Utc>,
    pub reason: String,
    pub affected_finding_ids: Vec<Id>,
}

pub enum ChangeType {
    SeverityChange { old: Severity, new: Severity },
    FindingAdded,
    FindingRemoved,
    EvidenceUpdated,
}
```

### Authentication State Machine

```rust
pub enum AuthState {
    Unauthenticated,
    Authenticating { attempt: u32 },
    Authenticated { token: Token, expires_at: DateTime<Utc> },
    Reconnecting { backoff: Duration },
    Failed { reason: AuthError, retryable: bool },
}

pub struct AuthManager {
    state: Arc<RwLock<AuthState>>,
    retry_policy: ExponentialBackoff,
    health_monitor: ConnectionHealthMonitor,
}
```

---

## Risks & Mitigation

| Risk | Impact | Probability | Mitigation |
|------|--------|-------------|------------|
| **WebKitGTK 6.0 migration breaks UI** | High (blocks Ubuntu 26.04) | Medium | Early testing on 26.04 beta, fallback to 4.x temporarily |
| **Report re-synthesis data loss** | High (enterprise blocker) | Low | Extensive versioning tests, backup before re-synthesis |
| **StrikeKit API changes** | Medium (integration delay) | Medium | Version pinning, abstraction layer, regular sync |
| **Authentication refactor breaks existing flows** | High (production outage) | Low | Feature flags, gradual rollout, extensive testing |
| **Tool integration quality varies** | Medium (user trust) | High | AI-assisted validation, quality gates per tool |

---

## Resource Requirements

**Development Team:**
- 1-2 Full-time engineers
- Part-time support for BlackArch tool integrations
- QA for platform testing (Kali, Ubuntu 26.04)

**Infrastructure:**
- Test environments: Ubuntu 24.04, 26.04 beta, Kali Linux
- StrikeKit staging environment for integration testing
- CI/CD pipeline for multi-platform builds

**Budget:**
- Test infrastructure: $0 (use existing Strike48 resources)
- CI/CD enhancements: $200/month (additional runners)
- Tool licenses for testing: $500 one-time
- **Total:** $2,900 for 6-month roadmap

---

## Timeline Summary

| Phase | Duration | Focus | Key Deliverables |
|-------|----------|-------|------------------|
| **Phase 1: Report Quality** | Weeks 1-6 | P0 fixes + evidence chains | Tool provenance, re-synthesis, attack chain classification |
| **Phase 2: Auth & Stability** | Weeks 7-12 | P1 fixes + resilience | ChatPanel fix, retry logic, connection health |
| **Phase 3: Integration & Polish** | Weeks 13-18 | StrikeKit + UI | Evidence Chain APIs, knowledge graph, polished UX |
| **Phase 4: Platform Expansion** | Weeks 19-24 | Platform support + tools | Kali support, WebKitGTK 6.0, BlackArch tools |

**Total Duration:** 24 weeks (6 months)
**Target Completion:** Mid-October 2026

---

## Open Questions

1. **StrikeKit Integration Priority:** Should Evidence Chain integration (Phase 3) be moved earlier?
2. **macOS Support Timing:** Issue #30 is P3 - should this be prioritized based on user demand?
3. **BlackArch Tool Selection:** Which 20+ tools from BlackArch have highest user value?
4. **Report Format:** Should we support PDF export in addition to MDX?
5. **Testing Resources:** Do we have Kali/Ubuntu 26.04 test environments available now?

---

## Next Steps

1. **Stakeholder Review:** Review and approve roadmap with Strike48 leadership
2. **Branch Strategy:** Create `feature/report-quality-p0` branch for #51 and #52
3. **Issue Assignment:** Assign P0 issues to developers
4. **Sprint Planning:** Break down Phase 1 work into 2-week sprints
5. **CI/CD Updates:** Add cargo-deny to pipeline in preparation for #12

---

**This roadmap positions Pick as a production-ready, enterprise-grade penetration testing platform by Q4 2026.**
