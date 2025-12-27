# Galactus — Rust vs Python Contract

## Purpose of This Document

This document defines the **explicit contract** between Rust (production enforcement) and Python (research and discovery) in Project Galactus.

It exists to:
- Prevent accidental logic duplication
- Avoid production drift
- Preserve determinism and research velocity simultaneously

This contract is non-negotiable.

---

## Foundational Separation

Galactus enforces a strict separation:

- **Python discovers truth**
- **Rust enforces truth**

This separation is philosophical, architectural, and technical.

---

## Role of Python

### Python Is Responsible For

- Feature discovery
- Hypothesis generation
- Exploratory data analysis
- Visualization and intuition-building
- Backtesting and walk-forward evaluation
- Failure and stress analysis

Python code is allowed to be:
- Iterative
- Experimental
- Redundant
- Rewritten frequently

---

### Python Is NOT Responsible For

- Live inference
- Deterministic signal computation
- Production decision logic
- Maintaining system state
- Performance-critical paths

Python outputs are **advisory to humans**, not authoritative to the system.

---

## Role of Rust

### Rust Is Responsible For

- Canonical data processing
- Deterministic feature computation
- Capital pressure inference
- Regime classification
- Confidence and stability evaluation
- Exposing stable inference APIs

Rust code must be:
- Deterministic
- Minimal
- Explicit
- Fully testable

Rust is the **system of record**.

---

### Rust Is NOT Responsible For

- Feature exploration
- Model experimentation
- Ad-hoc analysis
- Visualization
- Rapid iteration on unproven ideas

---

## The Promotion Boundary

Logic may cross from Python to Rust **only** via a formal promotion process.

### Promotion Requirements

All of the following must be satisfied:

1. **Economic rationale**
   - The capital behavior being modeled is explicitly documented
2. **Empirical stability**
   - Signal holds across regimes and stress conditions
3. **Deterministic formulation**
   - No randomness or hidden state
4. **Clear mathematical definition**
   - Inputs, outputs, and normalization are explicit
5. **Documentation update**
   - Relevant docs are updated prior to promotion

If any requirement fails, promotion is blocked.

---

## Interface Between Python and Rust

### Allowed Interaction Patterns

Python may:
- Call Rust binaries
- Invoke Rust services (HTTP / gRPC)
- Load Rust-produced datasets
- Validate Rust outputs

Rust may:
- Accept explicit configuration
- Emit structured outputs
- Remain agnostic of Python internals

---

### Forbidden Interaction Patterns

- Shared mutable state
- Python re-implementing Rust production logic
- Rust embedding Python interpreters for core logic
- Ad-hoc scripting inside production paths

---

## Configuration Handling

- All configuration consumed by Rust must be:
  - Explicit
  - Versioned
  - Auditable

Python may generate configuration candidates, but Rust consumes only approved versions.

---

## Testing Contract

### Rust Testing

- Deterministic unit tests
- Golden input/output tests
- Replay-based validation

### Python Testing

- Sanity checks
- Distribution checks
- Regime consistency validation

Python tests **do not replace** Rust tests.

---

## Failure Handling Across the Boundary

- Rust failures must be explicit and surfaced
- Python must treat Rust outputs as authoritative
- Python must not “fix” or smooth Rust outputs

---

## Evolution of the Contract

Changes to this contract require:
- Documentation update
- Review against vision and non-goals
- Explicit rationale

Silent evolution is forbidden.

---

## Final Statement

**Python explores possibility.  
Rust enforces reality.  
The boundary between them is sacred.**
