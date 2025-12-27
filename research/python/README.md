# Galactus Research — Python Experimentation Layer

**This directory contains the research and discovery engine of Galactus.**

---

## Purpose

The Python research layer is where **truth is discovered**.

It is responsible for:
- Feature discovery and hypothesis generation
- Exploratory data analysis
- Backtesting and walk-forward validation
- Stress testing and failure analysis
- Visualization and intuition-building
- Prototyping new signals and metrics

---

## What Belongs Here

### Allowed Content

- Exploratory analysis and prototyping
- Jupyter notebooks for investigation
- Experimental feature implementations
- Backtesting frameworks and scripts
- Visualization and plotting code
- Data quality checks and profiling
- Hypothesis testing and validation
- Research documentation and findings
- Ad-hoc analysis scripts
- Prototype signal implementations

### Explicit Criteria

Code belongs here if:
1. It is experimental or exploratory
2. It does not need to be deterministic (yet)
3. It can be rewritten or discarded
4. It serves to validate hypotheses
5. It supports decision-making for promotion

---

## What Does NOT Belong Here

### Forbidden Content

- ❌ Production inference logic
- ❌ Live decision-making systems
- ❌ Real-time execution paths
- ❌ Deterministic system state
- ❌ Critical business logic
- ❌ Production APIs

### Why These Are Forbidden

Production logic belongs in `core/rust/`. Mixing production with research:
- Violates the separation of discovery and enforcement
- Creates hidden dependencies
- Breaks determinism guarantees
- Obscures the promotion boundary
- Leads to production drift

---

## Code Organization

```
research/python/
├── notebooks/          # Jupyter notebooks for exploration
│   ├── exploratory/    # Ad-hoc investigations
│   ├── validation/     # Signal validation studies
│   └── backtesting/    # Backtest analysis
├── src/
│   ├── features/       # Experimental feature prototypes
│   ├── signals/        # Signal prototypes (pre-promotion)
│   ├── backtesting/    # Backtesting framework
│   ├── visualization/  # Plotting and visualization
│   ├── utils/          # Research utilities
│   └── validation/     # Validation tools
├── tests/              # Research code tests (sanity checks)
├── data/               # Research datasets (not production)
├── results/            # Analysis outputs and reports
├── pyproject.toml      # Python dependencies
└── README.md           # This file
```

---

## Development Philosophy

### Principle: Fast Iteration

Research code should be:
- **Iterative** — Rewrite frequently, don't over-engineer
- **Exploratory** — Try things, fail fast, learn
- **Flexible** — Adapt to new hypotheses quickly
- **Visual** — Show, don't just tell

### Principle: Research ≠ Production

Research code is allowed to be:
- Messy and exploratory
- Redundant across notebooks
- Not fully tested
- Quick and dirty
- Rewritten multiple times

**This is intentional and expected.**

---

## Research Workflow

### 1. Hypothesis Generation

Start with a question:
- What capital behavior are we trying to model?
- What constraint or pressure is at play?
- What would we expect to see if this is true?

Document the hypothesis clearly.

### 2. Exploratory Analysis

- Load data and visualize
- Look for structural patterns
- Check across regimes and time periods
- Identify failure modes early

Use notebooks liberally.

### 3. Prototype Implementation

- Implement a first version of the signal
- Don't worry about perfection
- Focus on understanding behavior

### 4. Backtesting and Validation

- Test across multiple regimes
- Look for brittleness and overfitting
- Stress test edge cases
- Document failure modes

### 5. Promotion Consideration

If a signal proves stable and explainable:
- Complete the [promotion checklist](../../docs/06-research-framework/promotion-checklist.md)
- Document the rationale
- Formalize the implementation plan
- Submit for promotion to Rust

---

## Testing in Research

Research code should have:

1. **Sanity checks** — Basic smoke tests
2. **Distribution checks** — Verify outputs are reasonable
3. **Regime consistency** — Check behavior across regimes

Research tests are **not replacements** for production tests.

They are lightweight checks to catch obvious bugs.

---

## Interaction with Rust

### Allowed Patterns

Python may:
- Call Rust binaries to validate outputs
- Load Rust-produced datasets for analysis
- Compare research prototypes against production
- Generate configuration candidates for Rust

Python must:
- Treat Rust outputs as authoritative
- Not "fix" or smooth Rust outputs
- Surface discrepancies explicitly

### Forbidden Patterns

- ❌ Re-implementing Rust production logic in Python
- ❌ Using Python for live inference
- ❌ Sharing mutable state with Rust
- ❌ Bypassing the promotion process

---

## Notebook Guidelines

### Good Notebook Practices

- Start with a clear question or hypothesis
- Document assumptions explicitly
- Show visualizations and distributions
- Highlight failure cases
- Summarize findings at the end

### Notebook Organization

```
notebooks/
├── exploratory/
│   └── YYYY-MM-DD-descriptive-name.ipynb
├── validation/
│   └── validate-signal-name-YYYY-MM-DD.ipynb
└── backtesting/
    └── backtest-signal-name-YYYY-MM-DD.ipynb
```

Use ISO date prefixes to maintain chronological order.

### Notebook Lifecycle

- **Exploratory notebooks** — Keep if useful, archive if not
- **Validation notebooks** — Keep if signal is promoted
- **One-off analysis** — Archive after use

Don't hoard notebooks indefinitely.

---

## Data Management

### Research Data

Research may use:
- Historical market data (for backtesting)
- Derived datasets (for validation)
- Synthetic data (for testing)

Research must **not** use:
- Live production data for experimentation
- Personalized or private data
- Unapproved proprietary data

### Data Versioning

- Tag datasets with versions or dates
- Document data sources and transformations
- Make analysis reproducible

---

## Backtesting Standards

### Backtesting Requirements

All signal candidates must be backtested with:

1. **Multiple regimes** — Bull, bear, sideways, volatile
2. **Walk-forward validation** — No peeking at future data
3. **Stress testing** — How does it fail?
4. **Regime transition analysis** — Behavior at boundaries
5. **Failure documentation** — Periods where it breaks

### Backtesting Anti-Patterns

🚫 **Overfitting to a single period**  
🚫 **Cherry-picking good results**  
🚫 **Ignoring failure cases**  
🚫 **Forward-looking bias**  
🚫 **PnL-only validation** (structure matters more)

---

## Promotion Process

When a signal is ready for production:

1. **Complete the checklist** — [Promotion Checklist](../../docs/06-research-framework/promotion-checklist.md)
2. **Document rationale** — Why this signal, why now?
3. **Define Rust interface** — How will it be implemented?
4. **Get approval** — Review with team
5. **Hand off to core** — Provide spec and test cases

After promotion:
- Keep the research prototype for reference
- Archive exploratory notebooks
- Document the decision in the decision log

---

## Visualization Guidelines

### Good Visualizations

- Show distributions, not just point estimates
- Highlight regime differences
- Make failure cases visible
- Use consistent scales and axes

### Visualization Tools

Recommended libraries:
- **matplotlib** / **seaborn** — Standard plotting
- **plotly** — Interactive visualizations
- **pandas.plot** — Quick and dirty plots

Keep visualizations in notebooks, not in production code.

---

## Dependency Management

### Allowed Dependencies

Research may use:
- Data science libraries (pandas, numpy, scipy)
- Visualization libraries (matplotlib, plotly)
- Statistical libraries (statsmodels, scikit-learn)
- Jupyter and notebook tools

### Discouraged Dependencies

Avoid in research:
- Heavy ML frameworks (unless necessary)
- Undocumented or experimental libraries
- Libraries with licensing issues

Keep `pyproject.toml` clean and documented.

---

## Code Quality in Research

### Minimum Standards

Even research code should:
- Have docstrings for key functions
- Use meaningful variable names
- Avoid deeply nested logic
- Be readable by others

### Pragmatic Approach

Research code does **not** need:
- 100% test coverage
- Perfect abstractions
- Production-grade error handling
- Optimization for performance

Focus on clarity and speed of iteration.

---

## Documentation in Research

### What to Document

- Hypotheses and assumptions
- Data sources and versions
- Key findings and insights
- Failure modes and edge cases
- Recommendations for promotion

### What Not to Document

- Every exploratory step
- Dead ends (unless instructive)
- Obvious implementation details

Document insights, not every keystroke.

---

## Failure Analysis

Research must actively seek failure:

- Test signals across all regimes
- Find edge cases where logic breaks
- Document structural reasons for failure
- Understand when silence is better than a signal

**A signal that "never fails" is suspicious.**

---

## Anti-Patterns

### 🚫 Re-implementing Production in Python

**Problem:** Copying Rust logic into Python "for convenience"  
**Why forbidden:** Creates divergence, defeats the purpose of separation  
**Solution:** Call Rust binaries or services

### 🚫 Hidden Production Use

**Problem:** Research code sneaking into live systems  
**Why forbidden:** Violates determinism and promotion process  
**Solution:** Keep research isolated, enforce promotion boundaries

### 🚫 PnL-Only Validation

**Problem:** Judging signals purely on backtest returns  
**Why forbidden:** Ignores structural validity and regime stability  
**Solution:** Validate structure first, performance second

### 🚫 Cherry-Picking Results

**Problem:** Showing only good periods, hiding failures  
**Why forbidden:** Creates false confidence, leads to bad promotions  
**Solution:** Always show failure cases explicitly

---

## Collaboration and Sharing

### Notebook Sharing

- Commit notebooks with outputs for review
- Clear outputs before committing if they are too large
- Add markdown cells explaining context

### Code Reviews

- Research code reviews focus on:
  - Clarity of hypothesis
  - Soundness of methodology
  - Honest treatment of failures
- Not focused on code perfection

---

## Success Criteria

The Python research layer is successful if:

- It generates valid hypotheses efficiently
- Signals are validated before promotion
- Failure modes are discovered early
- The promotion process is respected
- No research logic leaks into production

---

## Enforcement

Violations of these rules are architectural issues:

1. Research in production → Block and revert
2. Production re-implemented in Python → Block and document
3. Bypassed promotion process → Reject and require checklist

The boundary between research and production is **sacred**.

---

## References

- [Rust vs Python Contract](../../docs/02-system-architecture/rust-vs-python-contract.md)
- [Promotion Checklist](../../docs/06-research-framework/promotion-checklist.md)
- [Design Principles](../../docs/00-vision-and-non-goals/design-principles.md)
- [Research Methodology](../../docs/06-research-framework/research-methodology.md)

---

## Final Statement

**Python discovers truth.**

This directory is where we explore, fail, learn, and iterate.

Only proven discoveries graduate to production.
