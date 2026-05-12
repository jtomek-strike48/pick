# P0 Issues Action Plan

**Priority:** Critical (Now!)  
**Target Timeline:** Weeks 1-2  
**Issues:** #52, #51

---

## Issue #52: Tool Provenance & Reproducible Probe Commands

### Problem Statement

Reports show Pick agent wrapper names (`autopwn_webapp`, `default_creds_test`) instead of actual tool names (`nuclei`, `nikto`, `httpx`). Senior pentesters cannot reproduce findings from reports.

### Technical Changes Required

#### 1. Schema Changes

**File:** `crates/core/src/agent_output.rs` (or equivalent schema definition)

Add new `Provenance` struct:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provenance {
    /// Actual tool executed (e.g., "nuclei", "nikto", "httpx", "custom-s48-ssl")
    pub underlying_tool: String,
    
    /// Runtime-detected tool version
    pub tool_version: String,
    
    /// Exact commands executed
    pub probe_commands: Vec<ProbeCommand>,
    
    /// First N bytes of raw target response
    pub raw_response_excerpt: String,
    
    /// When this was executed
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProbeCommand {
    /// Exact command as executed (may contain sensitive data)
    pub command: String,
    
    /// Sanitized version safe for reports (tokens redacted)
    pub effective_command: String,
    
    /// Optional one-line description
    pub description: Option<String>,
}
```

Update `Finding` struct:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub id: String,
    pub title: String,
    pub severity: Severity,
    
    // NEW: Required provenance field
    pub provenance: Provenance,
    
    // ... existing fields
}
```

#### 2. Agent Modifications

**Files to update:**
- All agent implementations in `crates/agents/src/`
- Especially:
  - `autopwn_webapp.rs`
  - `default_creds_test.rs`
  - `web_vuln_scan.rs`
  - `port_scan.rs`

**Example change for nuclei wrapper:**

```rust
// Before
pub async fn run_nuclei_scan(&self, target: &str) -> Result<Vec<Finding>> {
    let output = Command::new("nuclei")
        .args(["-u", target, "-json"])
        .output()?;
    
    // Parse and return findings
}

// After
pub async fn run_nuclei_scan(&self, target: &str) -> Result<Vec<Finding>> {
    let command = format!("nuclei -u {} -json", target);
    let effective_command = format!("nuclei -u <TARGET> -json");
    
    let output = Command::new("nuclei")
        .args(["-u", target, "-json"])
        .output()?;
    
    // Detect nuclei version
    let version_output = Command::new("nuclei").arg("-version").output()?;
    let tool_version = parse_nuclei_version(&version_output.stdout)?;
    
    // Parse findings and add provenance
    let mut findings = parse_nuclei_output(&output.stdout)?;
    for finding in &mut findings {
        finding.provenance = Provenance {
            underlying_tool: "nuclei".to_string(),
            tool_version: tool_version.clone(),
            probe_commands: vec![ProbeCommand {
                command: command.clone(),
                effective_command: effective_command.clone(),
                description: Some("Web vulnerability scan with nuclei".to_string()),
            }],
            raw_response_excerpt: truncate_response(&output.stdout, 2048),
            timestamp: Utc::now(),
        };
    }
    
    Ok(findings)
}
```

#### 3. Report Generation Changes

**File:** `crates/report-agent/src/generator.rs`

Update report template to include:

1. **Per-finding provenance in detail blocks:**

```markdown
## Finding: SQL Injection in Login Form

**Severity:** CRITICAL

**Tool Used:** nuclei v3.2.4

**Probe Command:**
```bash
nuclei -u https://target.com/login -json -t sql-injection.yaml
```

**Evidence Excerpt:**
```
HTTP/1.1 200 OK
Server: nginx/1.18.0
...
{"error": "You have an error in your SQL syntax..."}
```

**Timestamp:** 2026-05-12T14:32:18Z
```

2. **Appendix table:**

```markdown
## Appendix: Finding Provenance

| Finding ID | Tool | Version | Probe Command | Verdict |
|------------|------|---------|---------------|---------|
| FIND-001 | nuclei | 3.2.4 | `nuclei -u <TARGET> -t sql-injection.yaml` | Confirmed |
| FIND-002 | nikto | 2.5.0 | `nikto -h <TARGET> -ssl` | Confirmed |
| FIND-003 | httpx | 1.3.7 | `httpx -u <TARGET> -status-code -title` | Info Only |
```

#### 4. Schema Validation

**File:** `crates/core/src/validation.rs`

Add validation that rejects findings without provenance:

```rust
pub fn validate_finding(finding: &Finding) -> Result<()> {
    if finding.provenance.underlying_tool.is_empty() {
        return Err(Error::InvalidFinding(
            "Finding missing required provenance.underlying_tool".to_string()
        ));
    }
    
    if finding.provenance.tool_version.is_empty() {
        return Err(Error::InvalidFinding(
            "Finding missing required provenance.tool_version".to_string()
        ));
    }
    
    if finding.provenance.probe_commands.is_empty() {
        return Err(Error::InvalidFinding(
            "Finding missing required provenance.probe_commands".to_string()
        ));
    }
    
    Ok(())
}
```

### Testing Strategy

#### Unit Tests

**File:** `crates/core/tests/provenance_tests.rs`

```rust
#[test]
fn test_provenance_serialization() {
    let provenance = Provenance {
        underlying_tool: "nuclei".to_string(),
        tool_version: "3.2.4".to_string(),
        probe_commands: vec![ProbeCommand {
            command: "nuclei -u https://target.com -json".to_string(),
            effective_command: "nuclei -u <TARGET> -json".to_string(),
            description: Some("Web scan".to_string()),
        }],
        raw_response_excerpt: "HTTP/1.1 200 OK".to_string(),
        timestamp: Utc::now(),
    };
    
    let json = serde_json::to_string(&provenance).unwrap();
    let deserialized: Provenance = serde_json::from_str(&json).unwrap();
    
    assert_eq!(provenance.underlying_tool, deserialized.underlying_tool);
}

#[test]
fn test_finding_validation_rejects_missing_provenance() {
    let finding = Finding {
        id: "FIND-001".to_string(),
        title: "Test Finding".to_string(),
        severity: Severity::High,
        provenance: Provenance {
            underlying_tool: "".to_string(), // Empty - should fail
            tool_version: "1.0.0".to_string(),
            probe_commands: vec![],
            raw_response_excerpt: "".to_string(),
            timestamp: Utc::now(),
        },
    };
    
    assert!(validate_finding(&finding).is_err());
}
```

#### Integration Tests

**File:** `tests/integration/report_provenance_test.rs`

```rust
#[tokio::test]
async fn test_full_scan_produces_reproducible_commands() {
    // Run a full scan
    let scan_result = run_full_scan("https://target.com").await.unwrap();
    
    // Generate report
    let report = generate_report(&scan_result).await.unwrap();
    
    // Verify every finding has provenance
    for finding in &scan_result.findings {
        assert!(!finding.provenance.underlying_tool.is_empty());
        assert!(!finding.provenance.tool_version.is_empty());
        assert!(!finding.provenance.probe_commands.is_empty());
        
        // Verify effective_command is in report
        assert!(report.contains(&finding.provenance.probe_commands[0].effective_command));
    }
    
    // Verify no agent wrapper names in report
    assert!(!report.contains("autopwn_webapp"));
    assert!(!report.contains("default_creds_test"));
    assert!(!report.contains("web_vuln_scan"));
}

#[tokio::test]
async fn test_sensitive_tokens_redacted_in_effective_command() {
    let finding = Finding {
        provenance: Provenance {
            probe_commands: vec![ProbeCommand {
                command: "curl -H 'Authorization: Bearer sk-abc123...' https://api.target.com".to_string(),
                effective_command: "curl -H 'Authorization: Bearer <REDACTED>' https://api.target.com".to_string(),
                description: None,
            }],
            // ... other fields
        },
        // ... other fields
    };
    
    let report = generate_report_with_findings(vec![finding]).await.unwrap();
    
    // Verify actual token not in report
    assert!(!report.contains("sk-abc123"));
    
    // Verify redacted version is in report
    assert!(report.contains("<REDACTED>"));
}
```

### Acceptance Criteria Checklist

- [ ] `Provenance` struct added to schema with all required fields
- [ ] `Finding` struct updated to require `provenance` field
- [ ] All agent implementations updated to populate provenance
- [ ] Tool version detection implemented for all wrapped tools
- [ ] `effective_command` sanitization logic implemented
- [ ] Report generator renders provenance in finding details (monospace code blocks)
- [ ] Report generator creates consolidated appendix table
- [ ] Schema validation rejects findings without provenance
- [ ] Unit tests for provenance serialization pass
- [ ] Unit tests for validation pass
- [ ] Integration test: full scan produces reproducible commands
- [ ] Integration test: sensitive tokens redacted in reports
- [ ] No agent wrapper names appear in generated reports
- [ ] Documentation updated with provenance schema examples

---

## Issue #51: Post-Validation Report Re-Synthesis

### Problem Statement

Reports generate executive summary, risk rating, and attack chains from pre-validation findings. When severity changes during validation, only the findings table updates - exec summary contradicts itself.

### Technical Changes Required

#### 1. Orchestrator Pipeline Changes

**File:** `crates/orchestrator/src/pipeline.rs`

Current (broken) flow:
```rust
pub async fn run_pentest_pipeline(&self, target: &str) -> Result<Report> {
    let recon_data = self.run_recon(target).await?;
    let scan_results = self.run_scan(&recon_data).await?;
    let exploit_results = self.run_exploit(&scan_results).await?;
    
    // BUG: Synthesize runs before validation!
    let report = self.synthesize_report(&exploit_results).await?;
    
    // Validation only patches findings table
    let validated_findings = self.validate_findings(&exploit_results.findings).await?;
    
    Ok(report)
}
```

Correct flow:
```rust
pub async fn run_pentest_pipeline(&self, target: &str) -> Result<Report> {
    let recon_data = self.run_recon(target).await?;
    let scan_results = self.run_scan(&recon_data).await?;
    let exploit_results = self.run_exploit(&scan_results).await?;
    
    // FIXED: Validate before synthesizing
    let validated_findings = self.validate_findings(&exploit_results.findings).await?;
    
    // Check for pending validations
    if has_pending_validations(&validated_findings) {
        return Err(Error::PendingValidations(
            "Cannot generate report with pending validations".to_string()
        ));
    }
    
    // Synthesize from validated findings only
    let report = self.synthesize_report(&validated_findings).await?;
    
    Ok(report)
}

fn has_pending_validations(findings: &[Finding]) -> bool {
    findings.iter().any(|f| {
        matches!(f.validation_status, ValidationStatus::Pending)
    })
}
```

#### 2. Report Agent Input Contract

**File:** `crates/report-agent/src/lib.rs`

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportInput {
    /// Validated findings manifest - MUST NOT contain pending validations
    pub validated_findings_manifest: ValidatedFindingsManifest,
    
    // ... other fields
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatedFindingsManifest {
    pub findings: Vec<Finding>,
    pub validation_complete_at: DateTime<Utc>,
    pub validator_signature: String,
}

impl ReportAgent {
    pub async fn generate_report(&self, input: ReportInput) -> Result<Report> {
        // Validate input contract
        self.validate_input(&input)?;
        
        // Generate all sections from validated findings
        let exec_summary = self.generate_exec_summary(&input.validated_findings_manifest)?;
        let risk_rating = self.compute_risk_rating(&input.validated_findings_manifest)?;
        let attack_chains = self.generate_attack_chains(&input.validated_findings_manifest)?;
        let remediation = self.generate_remediation_roadmap(&input.validated_findings_manifest)?;
        
        Ok(Report {
            exec_summary,
            risk_rating,
            attack_chains,
            remediation,
            findings: input.validated_findings_manifest.findings,
        })
    }
    
    fn validate_input(&self, input: &ReportInput) -> Result<()> {
        for finding in &input.validated_findings_manifest.findings {
            if matches!(finding.validation_status, ValidationStatus::Pending) {
                return Err(Error::InvalidInput(
                    format!("Finding {} has pending validation status", finding.id)
                ));
            }
        }
        Ok(())
    }
}
```

#### 3. Re-Synthesis on Validation Changes

**File:** `crates/orchestrator/src/report_manager.rs`

```rust
pub struct ReportManager {
    orchestrator: Arc<Orchestrator>,
}

impl ReportManager {
    /// Trigger full re-synthesis when any finding validation changes
    pub async fn handle_validation_change(&self, finding_id: &str) -> Result<()> {
        info!("Validation change detected for finding {}, triggering re-synthesis", finding_id);
        
        // Fetch all validated findings
        let all_findings = self.orchestrator.get_all_findings().await?;
        
        // Check if all validations complete
        if has_pending_validations(&all_findings) {
            warn!("Cannot re-synthesize: some findings still have pending validations");
            return Err(Error::PendingValidations(
                "Cannot generate report with pending validations".to_string()
            ));
        }
        
        // Trigger FULL re-synthesis
        let manifest = ValidatedFindingsManifest {
            findings: all_findings,
            validation_complete_at: Utc::now(),
            validator_signature: self.compute_signature()?,
        };
        
        let report_input = ReportInput {
            validated_findings_manifest: manifest,
        };
        
        let new_report = self.orchestrator.report_agent.generate_report(report_input).await?;
        
        // Save new report version
        self.save_report_version(&new_report).await?;
        
        Ok(())
    }
}
```

#### 4. Draft Mode

**File:** `crates/report-agent/src/draft.rs`

```rust
pub struct DraftReport {
    pub report: Report,
    pub watermark: String,
    pub unvalidated_findings: Vec<String>,
}

impl ReportAgent {
    pub async fn generate_draft_report(&self, unvalidated_findings: &[Finding]) -> Result<DraftReport> {
        // Allow draft generation from unvalidated findings
        let report = self.synthesize_report_internal(unvalidated_findings)?;
        
        Ok(DraftReport {
            report,
            watermark: "DRAFT - UNVALIDATED FINDINGS".to_string(),
            unvalidated_findings: unvalidated_findings.iter()
                .map(|f| f.id.clone())
                .collect(),
        })
    }
}
```

### Testing Strategy

#### Integration Tests

**File:** `tests/integration/report_synthesis_test.rs`

```rust
#[tokio::test]
async fn test_severity_change_triggers_re_synthesis() {
    // Run initial pentest
    let mut findings = run_pentest("https://target.com").await.unwrap();
    
    // Initial finding is CRITICAL
    assert_eq!(findings[0].severity, Severity::Critical);
    
    // Generate initial report
    let report_v1 = generate_report(&findings).await.unwrap();
    
    // Verify exec summary mentions CRITICAL
    assert!(report_v1.exec_summary.contains("CRITICAL"));
    assert!(report_v1.exec_summary.contains("IMMEDIATE ACTION REQUIRED"));
    
    // Change severity to FALSE_POSITIVE
    findings[0].severity = Severity::Info;
    findings[0].validation_status = ValidationStatus::FalsePositive;
    findings[0].validation_note = Some("Manual validation confirmed app correctly rejects credentials".to_string());
    
    // Trigger re-synthesis
    let report_v2 = regenerate_report(&findings).await.unwrap();
    
    // Verify exec summary NO LONGER mentions CRITICAL
    assert!(!report_v2.exec_summary.contains("CRITICAL"));
    assert!(!report_v2.exec_summary.contains("IMMEDIATE ACTION REQUIRED"));
    
    // Verify overall risk rating changed
    assert_ne!(report_v1.risk_rating, report_v2.risk_rating);
    
    // Verify attack chain updated
    assert_ne!(report_v1.attack_chains, report_v2.attack_chains);
    
    // Verify remediation roadmap changed
    assert_ne!(report_v1.remediation_roadmap, report_v2.remediation_roadmap);
}

#[tokio::test]
async fn test_orchestrator_blocks_report_with_pending_validations() {
    let findings = vec![
        Finding {
            id: "FIND-001".to_string(),
            validation_status: ValidationStatus::Pending, // Still pending
            severity: Severity::Critical,
            // ... other fields
        }
    ];
    
    let result = generate_report(&findings).await;
    
    // Should error
    assert!(result.is_err());
    
    // Error message should be clear
    let err = result.unwrap_err();
    assert!(err.to_string().contains("pending"));
    assert!(err.to_string().contains("FIND-001"));
}

#[tokio::test]
async fn test_draft_mode_produces_watermarked_output() {
    let findings = vec![
        Finding {
            validation_status: ValidationStatus::Pending,
            severity: Severity::Critical,
            // ... other fields
        }
    ];
    
    let draft_report = generate_draft_report(&findings).await.unwrap();
    
    // Verify watermark present
    assert!(draft_report.watermark == "DRAFT - UNVALIDATED FINDINGS");
    
    // Verify watermark appears in rendered output
    let rendered = render_report(&draft_report.report).await.unwrap();
    assert!(rendered.contains("DRAFT - UNVALIDATED FINDINGS"));
}
```

### Acceptance Criteria Checklist

- [ ] Pipeline ordering fixed: validation runs before synthesis
- [ ] Orchestrator validation check rejects pending findings with clear error
- [ ] Report agent input contract requires `ValidatedFindingsManifest`
- [ ] Report agent rejects input with pending validations
- [ ] Severity change triggers full re-synthesis of all sections
- [ ] Executive summary regenerates from validated findings
- [ ] Risk rating recomputes from validated severities
- [ ] Attack chains regenerate from validated findings
- [ ] Remediation roadmap regenerates from validated findings
- [ ] ISO control mapping regenerates
- [ ] Draft mode generates watermarked reports
- [ ] Draft reports blocked from final PDF export
- [ ] Integration test: CRITICAL → false_positive reflected across all sections
- [ ] Integration test: orchestrator blocks report with pending validations
- [ ] Integration test: partial validation update triggers full re-synthesis
- [ ] Integration test: draft mode watermark present
- [ ] Regression test added for original failure scenario
- [ ] Documentation updated with pipeline flow diagrams

---

## Implementation Order

### Week 1

**Days 1-2: Schema Changes (#52)**
1. Add `Provenance` and `ProbeCommand` structs
2. Update `Finding` struct to require provenance
3. Add schema validation logic
4. Write unit tests

**Days 3-4: Agent Updates (#52)**
1. Update 3-5 high-priority agents (nuclei, nikto, httpx)
2. Implement tool version detection
3. Implement command sanitization for `effective_command`
4. Write agent-specific tests

**Day 5: Pipeline Refactor (#51)**
1. Refactor orchestrator pipeline ordering
2. Add validation checks before report synthesis
3. Update report agent input contract
4. Write orchestrator tests

### Week 2

**Days 1-2: Report Generation (#52)**
1. Update report template with provenance sections
2. Add appendix table generation
3. Remove agent wrapper names from templates
4. Write report rendering tests

**Days 3-4: Re-Synthesis Logic (#51)**
1. Implement validation change handler
2. Add full re-synthesis trigger
3. Implement draft mode
4. Write re-synthesis tests

**Day 5: Integration Testing**
1. Run full end-to-end tests
2. Fix any discovered bugs
3. Update documentation
4. Code review and merge

---

## Success Criteria

### Definition of Done (Per Issue)

- [ ] All code changes implemented and reviewed
- [ ] Unit tests written and passing (80%+ coverage)
- [ ] Integration tests written and passing
- [ ] Documentation updated
- [ ] No critical bugs remaining
- [ ] CI/CD pipeline passes
- [ ] Manual testing completed

### Overall P0 Completion

- [ ] Issue #52 closed and verified
- [ ] Issue #51 closed and verified
- [ ] Reports include tool provenance and reproducible commands
- [ ] Reports regenerate correctly after validation changes
- [ ] No agent wrapper names in user-facing output
- [ ] Exec summary/risk rating/attack chains consistent with validated findings
- [ ] All acceptance criteria met for both issues
- [ ] Roadmap updated with completion status

---

**Ready to begin implementation once branch strategy is finalized.**
