# Project Galactus — GitHub Copilot Instructions

## 1. Project Context

**Galactus** is a capital-intent inference engine for the Indian stock market.

The system does **not** predict prices.  
It infers **forced and strategic capital behavior** using deterministic signals derived from public market data.

Galactus follows a Moneyball / BOB-style philosophy: redefine the unit of optimization rather than compete on traditional indicators.

---

## 2. Core Architectural Principles (Non-Negotiable)

1. **Determinism over cleverness**
   - Identical inputs must always produce identical outputs.
   - No hidden or mutable global state in production logic.

2. **Inference over prediction**
   - Outputs describe *pressure, probability, regime, or intent*.
   - Never frame logic as “bullish / bearish” unless explicitly requested.

3. **Capital behavior is the truth**
   - Optimize for **who must act**, not **what price did**.
   - Price is a secondary artifact.

4. **Strict separation of research and production**
   - Python discovers signals.
   - Rust enforces signals.

---

## 3. Language Responsibilities

### 3.1 Rust — Core Engine (Truth Layer)

Rust is used **only** for:

- Signal canonicalization
- Deterministic feature computation
- Capital pressure and constraint math
- Streaming consumers
- Stable APIs (gRPC / HTTP)
- Logic that directly affects live decisions

Rust code must be:

- Deterministic and reproducible
- Explicit about numeric types (`f64`, `Decimal`; no implicit casting)
- Free of randomness unless explicitly seeded and documented
- Allocation-aware and concurrency-safe
- Identical in behavior between backtest and live execution

Rust is the **system of record**.

---

### 3.2 Python — Research & Control Plane

Python is used **only** for:

- Feature discovery
- Hypothesis testing
- Statistical analysis
- Visualization
- Backtesting and evaluation
- Orchestration of Rust binaries or services

Python code may be:

- Exploratory and iterative
- Notebook-oriented
- Experimental

Python outputs are **never production truth** unless explicitly promoted.

---

## 4. Signal Design Guidelines

When generating or suggesting signals:

- Prefer **continuous measures** over binary signals
- Prefer **pressure, imbalance, constraint, or gradient-based metrics**
- Normalize by:
  - Free float
  - Average daily traded value
  - Time-to-expiry (for derivatives)
- Explicitly state:
  - Which capital is constrained
  - Why action becomes *forced*, not optional

Good framing:
> “Dealer gamma exposure creates forced hedging pressure within ±1.2% of spot.”

Bad framing:
> “The stock looks bullish.”

---

## 5. Naming Conventions

Use domain-correct, non-trading-slang terminology.

Preferred:
- `capital_pressure`
- `forced_flow_ratio`
- `intent_score`
- `regime_state`
- `liquidity_constraint`

Avoid:
- `buy_signal`
- `sell_signal`
- `alpha`
- `edge`
- `prediction`

---

## 6. Data Handling Rules

- Assume Indian market structure (NSE / BSE).
- Treat bhavcopy, options chain, and disclosures as **event-driven**, not continuous.
- Handle missing data explicitly.
- Never silently forward-fill.
- Document any interpolation or smoothing assumptions.

---

## 7. Explicit Anti-Patterns (Must Avoid)

Copilot must not suggest or generate:

### ❌ Price-only strategies
- RSI, MACD, moving averages, VWAP-only logic
- Candlestick pattern detection without capital context

### ❌ Python-only production engines
- Long-running Python services for live signals
- Python concurrency as a substitute for Rust

### ❌ Ad-hoc heuristics in Rust
- Magic thresholds without derivation
- Undocumented constants or shortcuts

### ❌ Black-box modeling by default
- Deep learning models without interpretability
- Models that cannot explain *why* capital must act

### ❌ Trading language leakage
- “Buy”, “Sell”, “Entry”, “Target” terminology
- Tip-based or discretionary phrasing

If uncertain, default to **research-grade Python**, not Rust.

---

## 8. Python → Rust Promotion Rules

Logic may be promoted from Python to Rust **only if all conditions are met**:

1. **Economic rationale is documented**
   - What capital is constrained?
   - Why does the constraint force behavior?

2. **Empirical stability**
   - Holds across multiple expiries
   - Works in high- and low-liquidity regimes
   - Survives at least one stress regime

3. **Deterministic formulation**
   - No randomness
   - No hidden state

4. **Clear mathematical definition**
   - Inputs, outputs, and normalization are explicit
   - No notebook-only shortcuts

5. **Backtest ↔ live parity**
   - Same code path for both
   - No research-only assumptions

If any condition fails, the logic remains in Python.

---

## 9. Testing Expectations

### Rust
- Deterministic unit tests
- Golden input / golden output tests
- Edge-case coverage (expiry, illiquidity, missing data)

### Python
- Sanity checks
- Distribution checks
- Regime consistency tests
- Failure analysis, not only success metrics

Avoid mock-heavy designs. Prefer real, shaped market data.

---

## 10. Copilot Self-Check (Mandatory)

Before suggesting code, implicitly verify:

1. Is this **research or production**?
2. Does it belong in **Python or Rust**?
3. Does it infer **capital behavior** or merely react to price?
4. Will it remain valid on:
   - Expiry day
   - Index rebalance weeks
   - Low-liquidity conditions?

If uncertain, default to **research-grade Python**.

---

## 11. Top Level Repository Structure

galactus/
├── README.md
├── CONTRIBUTING.md
├── CODE_OF_CONDUCT.md

├── docs/                          # 📜 Single source of truth
│   ├── 00-vision-and-non-goals/
│   ├── 01-market-theory/
│   ├── 02-system-architecture/
│   ├── 03-data-and-schemas/
│   ├── 04-signal-and-metrics/
│   ├── 05-intent-engine/
│   ├── 06-research-framework/
│   ├── 07-backtesting-and-validation/
│   ├── 08-risk-and-failure-modes/
│   ├── 09-compliance-and-language/
│   ├── 10-operational-playbooks/
│   ├── 11-decision-log/
│   └── 12-roadmap-and-deprecation/

├── registry/                      # 🧾 Canonical registries
│   ├── signals.yaml
│   ├── metrics.yaml
│   ├── regimes.yaml
│   └── schemas.yaml

├── core/                          # 🧠 Rust — production inference
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs
│   │   ├── engine/
│   │   │   ├── mod.rs
│   │   │   ├── intent_engine.rs
│   │   │   ├── capital_pressure.rs
│   │   │   ├── forced_flow.rs
│   │   │   ├── regime_classifier.rs
│   │   │   └── confidence.rs
│   │   ├── signals/
│   │   │   ├── mod.rs
│   │   │   ├── expiry_pressure.rs
│   │   │   └── gamma_imbalance.rs
│   │   ├── data/
│   │   │   ├── canonical_events.rs
│   │   │   ├── schemas.rs
│   │   │   └── validation.rs
│   │   ├── kill_switch.rs
│   │   └── errors.rs
│   └── tests/
│       ├── determinism_tests.rs
│       ├── confidence_degradation.rs
│       └── kill_switch.rs

├── research/                      # 🔬 Python — exploration only
│   ├── README.md
│   ├── notebooks/
│   ├── experiments/
│   │   ├── expiry_pressure/
│   │   │   ├── hypothesis.md
│   │   │   ├── experiment.py
│   │   │   ├── walk_forward.md
│   │   │   └── stress_results.md
│   ├── features/
│   ├── backtesting/
│   └── utils/

├── data/                          # 📦 Raw & processed datasets
│   ├── raw/
│   ├── canonical/
│   └── snapshots/

├── api/                           # 🔌 Non-advisory interfaces
│   ├── openapi.yaml
│   ├── server/
│   └── response_filters/

├── ci/                            # 🛡️ CI enforcement logic
│   ├── docs_checks.sh
│   ├── language_scan.sh
│   └── registry_sync.sh

├── .github/
│   ├── workflows/
│   │   ├── docs-enforcement.yml
│   │   ├── determinism-tests.yml
│   │   └── compliance-checks.yml
│   └── ISSUE_TEMPLATE/
│       ├── signal_proposal.md
│       └── research_experiment.md


---

## 12. Final Guiding Principle

**Python discovers truth.  
Rust enforces truth.  
Galactus never guesses.**
