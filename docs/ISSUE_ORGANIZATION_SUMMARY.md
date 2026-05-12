# Pick Issue Organization Summary

**Date:** 2026-05-12  
**Project:** Pick Platform  
**GitHub Project:** [Red Team/Backlog (#21)](https://github.com/orgs/Strike48/projects/21)

---

## Executive Summary

Pick has **24 open issues** tracked in GitHub Project 21 (Red Team/Backlog), plus 1 additional issue (#87) that was just added to the project. These issues span:

- **Report Quality** (8 issues) - Most critical, P0-P1 priority
- **Authentication/Connection** (4 issues) - P1-P3, affecting reliability
- **Platform Support** (3 issues) - P3, expanding OS compatibility
- **Tool Expansion** (2 issues) - P1 and P3, growing tool library
- **UI/Polish** (3 issues) - P2, improving user experience
- **StrikeKit Integration** (2 issues) - P1-P2, evidence chains and knowledge graph
- **Security/Infrastructure** (1 issue) - P2, cargo-deny setup

---

## Immediate Action Plan

### Step 1: Create Branch for P0 Issues ✅

**Branch created:** `feature/report-quality-p0`

### Step 2: P0 Issues (Now! - Weeks 1-2)

These are **critical blockers** for enterprise adoption and StrikeKit integration quality:

#### Issue #52: Tool provenance & reproducible probe commands
**Priority:** P0 (Critical)  
**Area:** Report Quality, Agent Schema  
**Problem:** Reports show Pick agent wrapper names instead of actual tool names and commands  
**Impact:** Senior red teamers cannot reproduce findings from reports - immediate loss of trust

**Required Changes:**
```yaml
finding:
  provenance:
    underlying_tool: string        # e.g., "nuclei", "nikto", "httpx"
    tool_version: string           # runtime-detected version
    probe_commands:
      - command: string            # exact command executed
        effective_command: string  # sanitized version for reports
        description: string
    raw_response_excerpt: string   # first N bytes of target response
    timestamp: ISO8601
```

**Acceptance Criteria:**
- Every agent output schema includes required `provenance` object
- Reports render `effective_command` in monospace code blocks
- Consolidated appendix table: Finding ID | Tool | Probe Command | Evidence | Verdict
- No agent wrapper names in user-facing reports
- Integration test: full scan produces reproducible commands for every finding

---

#### Issue #51: Post-validation report re-synthesis
**Priority:** P0 (Critical)  
**Area:** Report Quality, Orchestrator  
**Problem:** Reports generate exec summary, risk rating, attack chains from pre-validation findings. When severity changes during validation, only findings table updates - exec summary contradicts itself.

**Example Failure:**
- Exec summary: "CRITICAL - IMMEDIATE ACTION REQUIRED - Supply-chain compromise"
- Later in report: Finding marked "FALSE POSITIVE - Manual validation confirmed app correctly rejects credentials"
- Result: Report internally contradicts itself, devastating to credibility

**Root Cause:** Pipeline ordering - Report agent runs before validation completes

**Current (Broken):**
```
Recon → Scan → Exploit → Synthesize → Validate (patches findings table only)
```

**Correct:**
```
Recon → Scan → Exploit → Validate → Synthesize (from validated findings only)
```

**Acceptance Criteria:**
- Orchestrator refuses to invoke Report agent if any finding has `validation_status: pending`
- Report agent input schema requires `validated_findings_manifest`
- Changing any finding severity triggers **full re-synthesis** of all derivative sections
- Integration test: CRITICAL → false_positive change reflected in exec summary, risk tiles, attack chain, remediation roadmap
- Draft mode (`--draft`) produces watermarked output, blocked from final PDF export

---

### Step 3: Branch Strategy Decision

**Question for User:** Should we:

**Option A: Create new branch from main**
- Branch: `feature/report-quality-p0`
- Base: `origin/main` (clean slate)
- Pros: Clean start, no merge conflicts
- Cons: Loses work from `feature/rest-api-scan-management` branch

**Option B: Continue on current branch**
- Branch: `feature/rest-api-scan-management`
- Base: Current state with REST API work
- Pros: Keeps existing work
- Cons: May have merge conflicts with main

**Option C: Merge current branch first, then create P0 branch**
- Steps:
  1. Merge/PR `feature/rest-api-scan-management` to main
  2. Create fresh `feature/report-quality-p0` from updated main
- Pros: Clean separation of concerns
- Cons: Requires PR review cycle

**Recommendation:** **Option C** - Clean up REST API work first, then tackle P0 issues on fresh branch

---

## Complete Issue Breakdown

### P0 - Critical (Now!)
| # | Title | Area | Weeks |
|---|-------|------|-------|
| 52 | Tool provenance & reproducible probe commands | Report Quality, Schema | 1-2 |
| 51 | Post-validation report re-synthesis | Report Quality, Orchestrator | 1-2 |

### P1 - High Priority (Very Soon)
| # | Title | Area | Weeks |
|---|-------|------|-------|
| 85 | ChatPanel has no way to get Matrix session token | Authentication | 3-4 |
| 58 | Severity-change audit trail + Summary of Changes box | Report Quality | 3-4 |
| 54 | Per-finding probe method & live evidence in appendix | Report Quality | 5-6 |
| 53 | Attack chain classification (3-state) | Report Quality | 5-6 |
| 44 | Expand integrated tool count from 80 to 100+ | Tool Integration | 7-10 |
| 42 | Integrate StrikeKit Evidence Chain APIs | StrikeKit Integration | 7-10 |

### P2 - Medium Priority (Next Up)
| # | Title | Area | Weeks |
|---|-------|------|-------|
| 84 | Connector silently wedges on failed OTT redemption | Authentication | 11-12 |
| 87 | Parent Directory navigation malformed URL | UI Bug | 11-12 |
| 57 | Infrastructure profile synthesis section | Report Quality | 13-14 |
| 56 | Controls In Place section | Report Quality | 13-14 |
| 55 | Stable finding IDs across re-synthesis | Report Quality | 13-14 |
| 43 | Display Knowledge Graph from StrikeKit | StrikeKit Integration | 15-16 |
| 41 | Polish WiFi AutoPwn UI/UX | WiFi AutoPwn | 15-16 |
| 40 | Post-Exploitation Tool UI Enhancements | Post-Exploitation | 17-18 |
| 35 | Report created, but report file not in pick files | File Management | 17-18 |
| 12 | Add cargo-deny and resolve advisory ignores | Security | 17-18 |

### P3 - Lower Priority (Backlog)
| # | Title | Area | Weeks |
|---|-------|------|-------|
| 82 | Startup stalls ~90s on stale credentials | Authentication | 19-20 |
| 81 | Browser OAuth callback port conflicts | Authentication | 19-20 |
| 75 | Add official Kali Linux support | Platform Support | 21-22 |
| 74 | Upgrade to WebKitGTK 6.0 for Ubuntu 26.04+ | Platform Support | 23-24 |
| 64 | Epic: Expand Tool Library with BlackArch Tools | Tool Library | Ongoing |
| 30 | Mac OS support for virtualization framework | Platform Support | Future |

---

## Roadmap Document

Created: `docs/PICK_ROADMAP.md`

**Structure:**
- Executive Summary with current state and strategic priorities
- Issue organization by priority (P0, P1, P2, P3)
- 4 strategic phases over 24 weeks (6 months):
  - Phase 1: Report Quality Foundation (Weeks 1-6)
  - Phase 2: Authentication & Stability (Weeks 7-12)
  - Phase 3: StrikeKit Integration & Tool Expansion (Weeks 13-18)
  - Phase 4: Platform Expansion (Weeks 19-24)
- Success metrics and technical architecture
- Risk mitigation strategies
- Resource requirements

**Target Completion:** Mid-October 2026

---

## Next Steps

1. **Decision Required:** Choose branch strategy (Option A, B, or C above)

2. **If Option C (Recommended):**
   - Create PR for `feature/rest-api-scan-management`
   - Review and merge REST API work
   - Create fresh `feature/report-quality-p0` branch from updated main
   - Start work on issues #51 and #52

3. **Start P0 Work:**
   - Issue #52: Add provenance schema to agent contracts
   - Issue #51: Refactor orchestrator pipeline ordering

4. **Sprint Planning:**
   - Break down P0 issues into 2-week sprint
   - Assign developers
   - Set up daily standups

5. **Documentation:**
   - Review roadmap with stakeholders
   - Get approval for 6-month timeline
   - Align with StrikeKit roadmap dependencies

---

## Files Created

1. `docs/PICK_ROADMAP.md` - Comprehensive 6-month roadmap
2. `docs/ISSUE_ORGANIZATION_SUMMARY.md` - This file (issue breakdown and action plan)

---

**Ready to proceed with P0 fixes once branch strategy is decided.**
