# Galactus Core — Rust Production Engine

**This directory contains the deterministic production core of Galactus.**

---

## Purpose

The Rust core is the **system of record** for all production inference logic.

It is responsible for:
- Canonical data processing
- Deterministic feature computation
- Capital pressure inference
- Regime classification
- Confidence and stability evaluation
- Exposing stable inference APIs

---

## What Belongs Here

### Allowed Content

- Production inference logic
- Deterministic signal implementations
- Canonical data transformations
- Core data structures and schemas
- Production configuration schemas
- Integration APIs (gRPC, HTTP)
- Unit and integration tests
- Performance-critical code

### Explicit Criteria

Code may be placed here **only if**:
1. It has passed the full [promotion checklist](../../docs/06-research-framework/promotion-checklist.md)
2. It is deterministic (same inputs → same outputs)
3. It has no hidden state or randomness
4. It is fully documented
5. It has comprehensive test coverage
6. It can be explained in market-structure terms

---

## What Does NOT Belong Here

### Forbidden Content

- ❌ Exploratory analysis
- ❌ Experimental features
- ❌ Rapid prototyping code
- ❌ Ad-hoc analysis scripts
- ❌ Visualization logic
- ❌ Jupyter notebooks
- ❌ Research experiments
- ❌ Unproven hypotheses
- ❌ Python code (except for bindings/FFI)

### Why These Are Forbidden

Research belongs in `research/python/`. Mixing research with production:
- Breaks determinism guarantees
- Creates hidden dependencies
- Makes testing impossible
- Obscures the promotion boundary
- Violates the core architectural principle

---

## Code Organization

```
core/rust/
├── src/
│   ├── ingestion/      # Data ingestion and normalization
│   ├── features/       # Feature computation (promoted signals)
│   ├── intent/         # Intent engine core logic
│   ├── regime/         # Regime classification
│   ├── confidence/     # Confidence and stability metrics
│   ├── api/            # External APIs (gRPC, HTTP)
│   └── lib.rs          # Library root
├── tests/              # Integration tests
├── benches/            # Performance benchmarks
├── Cargo.toml          # Dependencies and build config
└── README.md           # This file
```

---

## Development Guidelines

### Principle: Boring Is Good

Rust code should be:
- **Explicit** — No clever tricks or hidden behavior
- **Minimal** — Do one thing extremely well
- **Testable** — Every function has unit tests
- **Documented** — Every public API is documented
- **Deterministic** — No randomness without explicit seeding

### Principle: Performance Is Not an Excuse

Performance requirements do **not** justify:
- Skipping documentation
- Reducing test coverage
- Breaking determinism
- Making code unexplainable

Fast and wrong is worse than slow and correct.

---

## Testing Requirements

All production code **must** have:

1. **Unit tests** — Every function tested in isolation
2. **Golden tests** — Known input/output pairs verified
3. **Edge case tests** — Boundary conditions enumerated
4. **Replay tests** — Historical events can be replayed exactly
5. **Failure tests** — Known failure modes are tested

Tests are not optional.

---

## Promotion Process

Logic may enter this directory **only** via the formal promotion process:

1. **Research phase** — Feature is developed and validated in `research/python/`
2. **Promotion proposal** — Complete the [promotion checklist](../../docs/06-research-framework/promotion-checklist.md)
3. **Documentation** — Update all relevant docs **before** implementation
4. **Implementation** — Implement in Rust with full test coverage
5. **Validation** — Verify output matches research prototype
6. **Decision log** — Document the promotion decision

If any step fails, promotion is blocked.

See: [Promotion Checklist](../../docs/06-research-framework/promotion-checklist.md)

---

## Interaction with Python

### Allowed Patterns

Python may:
- Call Rust binaries via CLI
- Invoke Rust services via HTTP/gRPC
- Load Rust-produced datasets
- Validate Rust outputs against research prototypes

Rust may:
- Accept explicit configuration files
- Emit structured outputs (JSON, Parquet, etc.)
- Remain completely agnostic of Python internals

### Forbidden Patterns

- ❌ Shared mutable state
- ❌ Python embedding in production paths
- ❌ Rust calling Python for core logic
- ❌ Ad-hoc scripting in production
- ❌ Python re-implementing production logic

---

## Failure Handling

Rust must:
- Fail explicitly when assumptions break
- Surface errors clearly
- Avoid silent degradation
- Log all failures structurally

"Almost correct" outputs are unacceptable.

---

## Performance Expectations

Production code should:
- Process events in real-time (<100ms latency typical)
- Support replay at 1000x+ speed
- Scale to full market data without degradation

However, **correctness beats performance**. Optimizations that compromise determinism or explainability are rejected.

---

## Configuration Management

All configuration must be:
- Explicit (no hidden defaults)
- Versioned (tracked in git)
- Auditable (who changed what and why)
- Validated (schema-checked before use)

Python may generate configuration candidates, but only approved configs are consumed in production.

---

## Dependency Policy

### Allowed Dependencies

- Standard Rust libraries
- Well-maintained, deterministic crates
- Numerical and data processing libraries

### Forbidden Dependencies

- Machine learning frameworks (inference only, no training)
- Python interpreters (except for optional bindings)
- Non-deterministic libraries
- Unmaintained or experimental crates

Every dependency must be justified in `Cargo.toml` comments.

---

## Documentation Requirements

Every production module must have:

1. **Module-level docs** — What it does and why
2. **Function docs** — Inputs, outputs, edge cases
3. **Example usage** — How to use the API
4. **Failure modes** — Known ways it can fail
5. **References** — Links to theory docs

Undocumented code will not merge.

---

## Review Process

All changes require:

1. **Code review** — At least one reviewer approval
2. **Test coverage** — All new code is tested
3. **Documentation** — Docs are updated
4. **Vision compliance** — Aligns with core principles
5. **Promotion validation** — Checklist is satisfied (for new features)

Exceptions are not allowed.

---

## Anti-Patterns

### 🚫 Quick Fixes Without Tests

**Problem:** Adding logic without test coverage  
**Why forbidden:** Breaks determinism guarantees, creates tech debt  
**Solution:** Write tests first, then implement

### 🚫 Clever Code

**Problem:** Overly clever, hard-to-read implementations  
**Why forbidden:** Reduces explainability, increases maintenance burden  
**Solution:** Be explicit and boring

### 🚫 Hidden State

**Problem:** Global state, thread-local storage, caches  
**Why forbidden:** Breaks determinism, makes testing impossible  
**Solution:** Make all state explicit in function signatures

### 🚫 Copy-Paste from Research

**Problem:** Directly porting Python research code without formalization  
**Why forbidden:** Research code is exploratory, not production-ready  
**Solution:** Follow the promotion checklist

---

## Migration Path

If you have existing logic that needs to move here:

1. Validate it meets all promotion criteria
2. Complete full documentation
3. Implement comprehensive tests
4. Get formal approval
5. Update decision log
6. Archive the research prototype

Do not shortcut this process.

---

## Success Criteria

The Rust core is successful if:

- All outputs are deterministic and replayable
- All code is tested and documented
- No research logic leaks into production
- Performance is acceptable without sacrificing correctness
- The promotion boundary is respected

---

## Enforcement

Violations of these rules are considered **architectural bugs** and must be:

1. Caught in code review
2. Blocked from merging
3. Documented as anti-patterns
4. Used to improve enforcement

The boundary between research and production is **sacred**.

---

## References

- [Rust vs Python Contract](../../docs/02-system-architecture/rust-vs-python-contract.md)
- [Promotion Checklist](../../docs/06-research-framework/promotion-checklist.md)
- [Design Principles](../../docs/00-vision-and-non-goals/design-principles.md)
- [Component Boundaries](../../docs/02-system-architecture/component-boundaries.md)

---

## Final Statement

**Rust enforces truth.**

This directory contains only logic that has been proven worthy of production use.

Everything else stays in research.
