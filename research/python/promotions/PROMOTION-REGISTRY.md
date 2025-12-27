# Promotion Registry

This document tracks all signal promotions from research (Python) to production (Rust core).

## Purpose

- Track promotion history
- Monitor active promotions
- Identify stalled promotions
- Maintain accountability

---

## Active Promotions

| Signal Name | Owner | Date Initiated | Status | Checklist File |
|-------------|-------|----------------|--------|----------------|
| OI Decay Pressure | Galactus Research Team | 2025-12-27 | Ready for Review | 2025-12-27-oi-decay-promotion.yml |
| Hedge Pressure | Galactus Research Team | 2025-12-27 | Ready for Review | 2025-12-27-hedge-pressure-promotion.yml |
| Basis Pressure | Galactus Research Team | 2025-12-27 | Ready for Review | 2025-12-27-basis-pressure-promotion.yml |

---

## Completed Promotions

| Signal Name | Owner | Date Promoted | Rust Version | Checklist File | Notes |
|-------------|-------|---------------|--------------|----------------|-------|
| *(No completed promotions yet)* | - | - | - | - | - |

---

## Rejected Promotions

| Signal Name | Owner | Date Rejected | Reason | Checklist File |
|-------------|-------|---------------|--------|----------------|
| *(No rejected promotions)* | - | - | - | - |

---

## Promotion Statistics

- **Total Initiated**: 3
- **Total Approved**: 0
- **Total Rejected**: 0
- **Currently Active**: 3
- **Average Time to Approval**: N/A

---

## Maintenance Instructions

### Adding a New Promotion

When a new promotion is initiated:

1. Add entry to **Active Promotions** table
2. Include signal name, owner, date, and checklist file path
3. Initial status should be `pending`
4. Commit this change with the checklist file

### Updating Promotion Status

As promotion progresses:

1. Update status in **Active Promotions** table
   - `pending` → Initial state
   - `in_review` → All checklist items complete, awaiting reviews
   - `approved` → All reviews approved, ready for implementation
   - `in_progress` → Rust implementation underway
   - `rejected` → Promotion rejected

2. Keep this registry in sync with checklist file status

### Completing a Promotion

When promotion is deployed:

1. Move entry from **Active Promotions** to **Completed Promotions**
2. Add deployment date and Rust version
3. Add any relevant notes
4. Update statistics

### Rejecting a Promotion

When promotion is rejected:

1. Move entry from **Active Promotions** to **Rejected Promotions**
2. Document rejection reason
3. Update statistics

---

## Example Entry

### Active Promotion Example

| Signal Name | Owner | Date Initiated | Status | Checklist File |
|-------------|-------|----------------|--------|----------------|
| Forced Expiry Pressure | research-team | 2024-03-15 | in_review | `research/python/promotions/2024-03-15-forced-expiry-pressure.yml` |

### Completed Promotion Example

| Signal Name | Owner | Date Promoted | Rust Version | Checklist File | Notes |
|-------------|-------|---------------|--------------|----------------|-------|
| Forced Expiry Pressure | research-team | 2024-04-10 | v0.2.0 | `research/python/promotions/2024-03-15-forced-expiry-pressure.yml` | Successfully validated across 3 expiry cycles |

---

## Related Documents

- [`promotion-checklist.md`](promotion-checklist.md) — Promotion requirements
- [`promotion-checklist-format.md`](promotion-checklist-format.md) — Machine-readable format
- [`signal-lifecycle.md`](../04-signal-and-metrics/signal-lifecycle.md) — Signal lifecycle stages
- [`decision-log.md`](../11-decision-log/decision-log.md) — Architectural decisions

---

## Notes

- This registry is manually maintained
- Always keep it synchronized with checklist files
- Review this registry during quarterly audits
- Use it to identify stuck promotions
