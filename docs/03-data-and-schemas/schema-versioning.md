
- **Major**: Breaking semantic change
- **Minor**: Backward-compatible extension
- **Patch**: Non-semantic clarification or metadata change

---

## Change Categories

### 1. Patch Changes (x.y.Z)

Examples:
- Documentation clarification
- Metadata addition that does not affect inference
- Typo fixes in field descriptions

Rules:
- No inference impact
- No migration required

---

### 2. Minor Changes (x.Y.z)

Examples:
- Adding optional fields
- Adding new enum values
- Adding new event subtypes

Rules:
- Must not alter meaning of existing fields
- Consumers may safely ignore new fields
- Documentation update required

---

### 3. Major Changes (X.y.z)

Examples:
- Renaming fields
- Changing field semantics
- Removing fields
- Changing numeric units or normalization

Rules:
- Require new schema version
- Require explicit migration strategy
- Parallel support recommended during transition

Major changes invalidate naive backtest comparisons.

---

## Migration Strategy

For major schema changes:

- Old and new schemas must coexist temporarily
- Migration logic must be explicit and documented
- Backfilled data must be version-tagged

No automatic or silent migrations are allowed.

---

## Backward Compatibility Expectations

- Consumers must explicitly declare supported schema versions
- Fallback behavior must be explicit
- Unsupported versions must fail fast

Graceful degradation is preferred to silent acceptance.

---

## Schema Ownership and Review

Schema changes require:
- Review against design principles
- Alignment with market theory
- Documentation updates in `/docs`

No individual owns a schema.  
Schemas are system contracts.

---

## Impact on Backtesting and Research

Schema changes must:
- Preserve historical comparability where possible
- Clearly document incompatibilities
- Annotate affected research results

Results derived under different schema versions must not be compared blindly.

---

## Enforcement Mechanisms

Schema governance should be enforced through:
- Version checks in code
- CI validation
- Documentation requirements

Violations are architectural bugs.

---

## Final Statement

**Schemas evolve, but inference integrity must not.**

Versioning is how Galactus remembers its past correctly.
