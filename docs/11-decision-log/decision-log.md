# Galactus — Decision Log

## Purpose
This document records significant architectural and design decisions.

---

## Decision 001: Core Vision Lock (2024)

### Date
2024 Q4

### Decision
Establish the core vision documents as immutable and create enforcement mechanisms to prevent scope creep, feature drift, and architectural compromise.

### Context
- Project Galactus requires a clear, permanent identity as a capital-pressure inference engine
- Need to explicitly exclude: trading systems, advisory tools, execution logic, and prediction engines
- Historical pattern shows systems drift from original vision under commercial/feature pressure
- Regulatory clarity requires permanent non-advisory stance
- Technical coherence depends on stable architectural foundation

### Alternatives Considered

**Alternative 1: Flexible Vision**
- Allow vision to evolve based on user feedback and market needs
- Rejected because:
  - Leads to scope creep and feature accumulation
  - Compromises core technical advantages (determinism, explainability)
  - Creates regulatory ambiguity
  - Dilutes competitive differentiation

**Alternative 2: No Formal Vision**
- Keep vision informal, let code drive architecture
- Rejected because:
  - Results in inconsistent decisions
  - No mechanism to reject out-of-scope proposals
  - Technical debt from conflicting objectives
  - Cannot justify trade-offs consistently

### Rationale
- **Design Principle: Determinism over cleverness** — Stable vision enables deterministic architecture
- **Design Principle: Minimalism in core** — Vision lock prevents feature accumulation
- **Market Theory: Capital behavior is the primitive** — Clear scope focuses on core competency
- Permanent non-advisory stance provides regulatory clarity
- Immutable boundaries enable technical excellence in narrow domain

### Implementation
- Created `VISION_LOCK.md` defining immutability policy
- Created `README.md` anchoring vision at repository root
- Established formal revision process requiring:
  - Evidence of market structure changes
  - Decision log entry
  - Documentation cascade
- Defined automatic rejection criteria for vision violations

### Trade-offs and Consequences

**Accepted Costs:**
- Reduced flexibility to pivot into adjacent markets
- May miss opportunities in trading/advisory space
- Requires discipline to reject attractive features
- Ongoing enforcement overhead

**Benefits:**
- Architectural coherence across all components
- Clear regulatory positioning
- Protection against feature drift
- Simplified decision-making framework
- Long-term technical viability

### Success Metrics
- Zero vision-violating features merged
- Consistent rejection of out-of-scope proposals
- Architectural decisions reference vision documents
- Quarterly compliance audits pass
- System maintains non-advisory language

### Revisit Conditions
This decision should be revisited if:
- Indian market structure changes fundamentally (regulatory framework, microstructure)
- Core assumption (capital constraints drive markets) is empirically invalidated
- Deterministic inference becomes technically impossible
- Legal/regulatory landscape makes current stance untenable

**Note:** Poor performance, competitive pressure, or feature requests are NOT valid revisit conditions.

### References
- `docs/00-vision-and-non-goals/vision.md`
- `docs/00-vision-and-non-goals/explicit-non-goals.md`
- `docs/00-vision-and-non-goals/design-principles.md`
- `VISION_LOCK.md`

---

## Decision 002: Rust vs Python Boundary Enforcement (2024)

### Date
2024 Q4

### Decision
Implement strict enforcement mechanisms to maintain separation between Rust (deterministic production core) and Python (research and discovery), preventing research code leakage into production.

### Context
- The [Rust vs Python Contract](../02-system-architecture/rust-vs-python-contract.md) establishes philosophical separation but lacks concrete enforcement
- Without enforcement, boundary violations are inevitable:
  - Research code accidentally promoted without validation
  - Production logic duplicated in Python for "convenience"
  - Hidden dependencies between layers
  - Gradual erosion of determinism guarantees
- Need for automated detection and prevention mechanisms
- Team growth requires clearer structural boundaries
- Production reliability depends on determinism, which requires strict separation

### Alternatives Considered

**Alternative 1: Trust-Based Approach**
- Rely on code review and team discipline alone
- No automated enforcement or directory structure
- Rejected because:
  - Scales poorly as team grows
  - Violations are easy to miss in review
  - Creates ambiguity about what belongs where
  - No way to prevent accidental violations

**Alternative 2: Monorepo Without Physical Separation**
- Keep all code in single directory structure
- Use naming conventions to separate concerns
- Rejected because:
  - Too easy to create hidden dependencies
  - Naming conventions are not enforced mechanically
  - Makes violations invisible to tooling
  - Reduces clarity of architectural intent

**Alternative 3: Separate Repositories**
- Maintain Rust and Python in completely separate repos
- Rejected because:
  - Creates synchronization overhead
  - Makes validated promotion harder
  - Reduces visibility across layers
  - Over-isolates research from production validation

### Rationale
- **Design Principle: Separation of discovery and enforcement** — Physical and automated separation operationalizes the philosophical principle
- **Design Principle: Determinism is a feature** — Preventing research leakage protects determinism guarantees
- **Design Principle: Explicit failure over hidden failure** — Automated checks surface violations immediately
- Prevention through structure is more reliable than prevention through discipline
- Layered enforcement (physical + process + automation + culture) provides defense in depth
- Clear boundaries reduce cognitive load and decision fatigue

### Implementation

#### Physical Structure
- Created `core/rust/` — Production inference engine only
- Created `research/python/` — Research and experimentation only
- Created comprehensive README.md in each directory defining responsibilities
- Added `.gitignore` to prevent accidental commits of build artifacts

#### Documentation
- Created [`rust-python-boundary-enforcement.md`](../02-system-architecture/rust-python-boundary-enforcement.md) — Detailed enforcement mechanisms
- Updated existing contract documentation with enforcement references
- Defined clear promotion workflow and artifacts

#### Automation
- Created `scripts/check_boundaries.sh` — Validates directory separation
- Created `scripts/validate_promotion.sh` — Checks promotion process compliance
- Created `.github/workflows/boundary-enforcement.yml` — CI/CD integration
- Automated detection of:
  - Python in Rust directories
  - Rust in Python directories (except FFI)
  - Shared code directories
  - Missing tests for new production code
  - Promotions without checklists

#### Process Gates
- Promotion requires completed [promotion checklist](../06-research-framework/promotion-checklist.md)
- New production code requires decision log entry
- CI blocks merges with boundary violations
- Code review checklist includes boundary validation

### Trade-offs and Consequences

**Accepted Costs:**
- Additional directory structure overhead
- Promotion process adds friction
- Enforcement scripts require maintenance
- Cannot "quickly prototype" in production
- May feel bureaucratic for small changes

**Benefits:**
- Prevents determinism violations
- Makes architectural intent explicit
- Reduces hidden dependencies
- Enables confident parallel development
- Protects against accidental violations
- Scales with team growth
- Reduces technical debt accumulation

**Expected Challenges:**
- Team must learn and internalize boundaries
- Initial setup and enforcement overhead
- Resistance to "extra process"
- False positives in automated checks

**Mitigation Strategies:**
- Clear documentation and examples
- Onboarding materials for new team members
- Regular boundary audits to refine enforcement
- Balance automation with pragmatism

### Success Metrics

Track monthly:
- Number of boundary violations caught in CI (should trend toward zero)
- Number of promotions (should be low and deliberate)
- Time from research to promotion (acceptable if thorough)
- Production incidents from boundary violations (should be zero)
- Team satisfaction with process (qualitative feedback)

Success indicators:
- Violations caught before merge, not after
- Promotions are rare and well-documented
- No production logic found in research
- No research logic found in production
- Team actively references boundary documentation

### Revisit Conditions

This decision should be revisited if:
- Enforcement creates excessive friction without preventing violations
- Technology changes make current enforcement obsolete (e.g., new languages)
- Alternative enforcement mechanisms prove more effective
- Team size or structure changes dramatically
- Automated checks have high false positive rate

This decision should NOT be revisited due to:
- "Too much process" complaints without specific violations prevented
- Desire for faster prototyping at expense of determinism
- Convenience arguments without safety justification

### References
- [Rust vs Python Contract](../02-system-architecture/rust-vs-python-contract.md) — Foundational contract
- [Rust Python Boundary Enforcement](../02-system-architecture/rust-python-boundary-enforcement.md) — Enforcement mechanisms
- [Promotion Checklist](../06-research-framework/promotion-checklist.md) — Required for all promotions
- [Design Principles](../00-vision-and-non-goals/design-principles.md) — Principle 5: Separation of discovery and enforcement
- `core/rust/README.md` — Production layer documentation
- `research/python/README.md` — Research layer documentation

---

## Decision 003: Promotion of OI Decay Pressure Signal (2025-12-27)

### Date
2025-12-27

### Decision
Approve promotion of OI Decay Pressure signal from Python research to Rust production core.

### Context
- OI Decay Pressure identifies pressure from decaying open interest in futures contracts
- Signal represents institutional position unwinding behavior
- Maps directly to capital behavior theory in derivatives-dominant markets
- All 20 research tests passing with comprehensive validation
- Aligns with core vision of capital pressure inference

### Alternatives Considered

**Alternative 1: Wait for more historical data**
- Collect 6+ months of additional validation data before promotion
- Rejected because:
  - Current statistical validation is already robust (p < 0.05)
  - Signal has deterministic mathematical definition
  - Walk-forward validation already implemented
  - Delaying provides diminishing returns

**Alternative 2: Deploy as experimental feature flag**
- Keep in Python layer with feature flag for testing
- Rejected because:
  - Violates Rust vs Python boundary (production logic must be in Rust)
  - Creates technical debt in deployment architecture
  - Reduces performance (Python vs Rust execution)

### Rationale
- **Design Principle: Capital constraints drive markets** — OI decay directly captures institutional unwinding
- **Design Principle: Determinism over cleverness** — Mathematical definition is deterministic and reproducible
- **Market Theory: Derivatives dominance** — Futures OI is primary signal in Indian markets
- Statistical rigor validated with 20 comprehensive test cases
- Performance requirements met (< 100ms per computation)

### Implementation
- Add OI decay computation to `core/rust/src/features/`
- Integrate with intent engine aggregation layer
- Implement comprehensive Rust unit tests
- Update monitoring dashboards for OI-based signals

### Trade-offs and Consequences

**Accepted Costs:**
- Maintenance burden increases with additional production signal
- Rust implementation requires careful numerical precision handling
- Monitoring infrastructure needs OI-specific alerting

**Benefits:**
- First research signal successfully promoted through new framework
- Validates promotion process and quality gates
- Adds key institutional positioning signal to production
- Demonstrates research-to-production pipeline

### Success Metrics
- Signal computation time < 10ms (target) vs 100ms (research)
- Zero data quality issues in first 30 days
- Correlation with research implementation > 0.99
- No kill-switch activations due to signal anomalies

### Revisit Conditions
This decision should be revisited if:
- Statistical significance drops below p < 0.05 in production
- Correlation with capital behavior becomes unclear
- OI data quality degrades systematically
- Performance degrades below 50ms consistently

### References
- `research/python/src/features/oi_decay_research.py`
- `research/python/tests/test_oi_decay_research.py`
- `docs/01-market-theory/derivatives-dominance.md`
- `docs/06-research-framework/promotion-checklist.md`

---

## Decision 004: Promotion of Hedge Pressure Signal (2025-12-27)

### Date
2025-12-27

### Decision
Approve promotion of Hedge Pressure signal from Python research to Rust production core.

### Context
- Hedge Pressure identifies directional bias from call-put OI imbalances
- Reflects institutional hedging activity patterns vs retail positioning
- Maps to capital behavior model for option market inefficiencies
- All 21 research tests passing with statistical validation
- Captures institutional vs retail dynamics critical to Indian markets

### Alternatives Considered

**Alternative 1: Combine with OI Decay into single signal**
- Create unified derivatives pressure signal
- Rejected because:
  - Different mechanisms: OI decay is time-based, hedge pressure is strike-based
  - Different failure modes and monitoring needs
  - Violates single responsibility principle
  - Reduces interpretability and explainability

**Alternative 2: Deploy simplified version first**
- Remove strike proximity weighting, use simple OI ratio
- Rejected because:
  - Strike proximity is essential for signal quality
  - Simplified version tested poorly in research
  - Would require re-promotion later for full version
  - Research already validates complete implementation

### Rationale
- **Design Principle: Capital behavior is the primitive** — Call-put imbalance reveals hedging pressure
- **Market Theory: Retail vs institutional dynamics** — Captures key market inefficiency
- **Design Principle: Interpretability** — Clear explanation for each signal activation
- Effect size validated across different market conditions
- 21 comprehensive test cases with edge case coverage

### Implementation
- Add hedge pressure computation to `core/rust/src/features/`
- Implement efficient option chain parsing in Rust
- Integrate with intent engine for directional bias signals
- Add option chain quality validation

### Trade-offs and Consequences

**Accepted Costs:**
- Higher computational cost (< 150ms research, target < 50ms production)
- Option chain data is larger and more complex than futures data
- Strike filtering algorithms need careful optimization
- Higher memory usage (< 100MB) compared to other signals

**Benefits:**
- Adds critical directional bias signal from options market
- Complements OI decay with different market mechanism
- Validates promotion process for complex data structures
- Demonstrates option chain processing capability

### Success Metrics
- Signal computation time < 50ms
- Memory usage < 100MB during option chain processing
- Correlation with research implementation > 0.99
- Strike filtering algorithm performance validated
- Zero option chain parsing errors in first 30 days

### Revisit Conditions
This decision should be revisited if:
- Option chain data quality degrades consistently
- Computational cost exceeds 100ms regularly
- Correlation with institutional positioning becomes unclear
- Strike proximity weighting proves ineffective

### References
- `research/python/src/features/hedge_pressure_research.py`
- `research/python/tests/test_hedge_pressure_research.py`
- `docs/01-market-theory/retail-vs-institutional-dynamics.md`
- `docs/06-research-framework/promotion-checklist.md`

---

## Decision 005: Promotion of Basis Pressure Signal (2025-12-27)

### Date
2025-12-27

### Decision
Approve promotion of Basis Pressure signal from Python research to Rust production core.

### Context
- Basis Pressure identifies arbitrage pressure from futures-spot basis divergence
- Reveals arbitrage capital flows and market maker positioning
- Maps to capital behavior through cost-of-carry relationships
- All 21 research tests passing with high correlation (0.9978)
- Critical for derivatives-dominant market understanding

### Alternatives Considered

**Alternative 1: Use simple basis without time decay**
- Calculate raw futures-spot difference only
- Rejected because:
  - Ignores time-to-expiry effects (critical for basis)
  - Research shows time decay adjustment improves signal quality
  - Would miss arbitrage opportunities near expiry
  - Not aligned with theoretical cost-of-carry model

**Alternative 2: Wait for risk-free rate integration**
- Delay until risk-free rate data source is available
- Rejected because:
  - Current approximation (zero rate) is acceptable for Indian markets
  - Perfect is enemy of good — can enhance later
  - Statistical validation already robust without it
  - Risk-free rate adjustments are second-order effects

### Rationale
- **Market Theory: Derivatives dominance** — Basis reflects arbitrage efficiency
- **Design Principle: Determinism** — Mathematical cost-of-carry model is deterministic
- **Capital Behavior: Arbitrage flows** — Basis divergence reveals capital constraints
- Target vs computed correlation: 0.9978 (excellent)
- High-precision mathematical computations validated

### Implementation
- Add basis pressure computation to `core/rust/src/features/`
- Implement high-precision floating point basis calculations
- Integrate futures-spot data synchronization
- Add time-to-expiry calculations with proper date handling

### Trade-offs and Consequences

**Accepted Costs:**
- Requires synchronization of futures and spot data (complexity)
- Time-to-expiry calculations need accurate calendar handling
- Risk-free rate initially approximated (enhancement needed later)
- Cross-validates futures vs spot price relationships (overhead)

**Benefits:**
- Captures key arbitrage inefficiency signal
- Validates multi-data-source signal promotion
- Demonstrates high-precision computation capability
- Critical signal for derivatives market understanding

### Success Metrics
- Signal computation time < 25ms
- Memory usage < 25MB
- Correlation with research implementation > 0.995
- Futures-spot synchronization errors < 0.1%
- Basis anomaly detection operational

### Revisit Conditions
This decision should be revisited if:
- Futures-spot data synchronization becomes unreliable
- Risk-free rate effects prove material (> 5% impact)
- Basis calculation precision degrades
- Arbitrage relationship breaks down systematically

### References
- `research/python/src/features/basis_pressure_research.py`
- `research/python/tests/test_basis_pressure_research.py`
- `docs/01-market-theory/capital-behavior-model.md`
- `docs/06-research-framework/promotion-checklist.md`

---

## Decision 006: CI/CD Workflows for Build and Test Automation (2025-12-28)

### Date
2025-12-28

### Decision
Implement automated CI/CD workflows for Rust core and Python research components to ensure code quality, security, and consistency across all contributions.

### Context
- Project Galactus requires high code quality standards for both deterministic Rust production code and exploratory Python research code
- Manual code quality checks are error-prone and do not scale with team growth
- Need automated enforcement of formatting, linting, type checking, and testing standards
- Security audit automation needed to catch vulnerabilities early
- Lack of automated checks leads to inconsistent code styles and potential bugs reaching production
- GitHub Actions provides integrated CI/CD platform for automated checks

### Alternatives Considered

**Alternative 1: Pre-commit hooks only**
- Rely solely on local pre-commit hooks for quality checks
- Rejected because:
  - Easy to bypass or disable locally
  - No enforcement for contributors who don't set up hooks
  - No centralized record of check results
  - Cannot catch issues after commit but before merge

**Alternative 2: Single combined workflow**
- Create one workflow that runs all Rust and Python checks
- Rejected because:
  - Slower feedback (must run all checks even for single-language changes)
  - Less clear failure attribution
  - Harder to maintain and debug
  - Violates separation of concerns between Rust/Python layers

**Alternative 3: Manual code review only**
- Depend entirely on human code review for quality
- Rejected because:
  - Scales poorly with team size
  - Inconsistent application of standards
  - Wastes reviewer time on mechanical checks
  - Cannot catch all formatting and security issues

### Rationale
- **Design Principle: Determinism over cleverness** — Automated checks ensure consistent, deterministic quality enforcement
- **Design Principle: Explicit failure over hidden failure** — CI failures surface issues immediately and visibly
- **Design Principle: Separation of discovery and enforcement** — Separate workflows for Rust (production) and Python (research) reflect architectural boundaries
- Automated checks prevent technical debt accumulation
- Security auditing catches vulnerabilities before production
- Consistent formatting reduces cognitive load and code review time
- Early feedback loop improves developer productivity

### Implementation

#### Rust Core Workflow (`.github/workflows/build-core.yml`)
Triggers on:
- Push to `production` branch
- Pull requests to `production` branch
- Changes to `core/rust/**` or workflow file itself

Steps:
1. **Checkout code** — Uses `actions/checkout@v4` for repository access
2. **Setup Rust** — Uses `dtolnay/rust-toolchain@stable` with `rustfmt` and `clippy` components
3. **Install cargo audit** — Security vulnerability scanner for Rust dependencies
4. **Cache Rust dependencies** — Uses `Swatinem/rust-cache@v2` for faster builds
5. **Check formatting** — Runs `cargo fmt --check` to enforce consistent code style
6. **Run clippy** — Static analysis with `-D warnings` to catch common mistakes
7. **Check compilation** — Ensures code compiles with `cargo check`
8. **Security audit** — Runs `cargo audit` for known vulnerabilities (continue-on-error for advisory only)
9. **Build release** — Full release build with optimizations
10. **Run tests** — Complete test suite execution

#### Python Research Workflow (`.github/workflows/build-research.yml`)
Triggers on:
- Push to `production` branch
- Pull requests to `production` branch  
- Changes to `research/python/**` or workflow file itself

Steps:
1. **Checkout code** — Uses `actions/checkout@v4`
2. **Setup Python** — Uses `actions/setup-python@v6` with Python 3.10
3. **Cache Poetry environment** — Uses `actions/cache@v5` for dependency caching
4. **Install Poetry** — Python dependency management tool
5. **Configure Poetry** — Disable virtualenv creation for CI environment
6. **Install dependencies** — Single `poetry install` for all dependencies
7. **Run tests** — Execute pytest with verbose output
8. **Check formatting (Black)** — Enforce consistent Python code style (line-length 88)
9. **Check import sorting (isort)** — Validate import organization with Black profile
10. **Lint code (flake8)** — Static analysis for Python code quality
11. **Type check (mypy)** — Static type checking (continue-on-error for gradual adoption)

#### Maintenance Requirements
- **Toolchain updates**: Periodically update Rust stable version and Python version as needed
- **Dependency updates**: Update GitHub Actions versions when new releases available
- **Cache tuning**: Monitor cache hit rates and adjust cache keys if necessary
- **Performance monitoring**: Track CI run times and optimize if builds become too slow

### Trade-offs and Consequences

**Accepted Costs:**
- CI runtime overhead (2-5 minutes per workflow)
- Maintenance burden for workflow files and tool configurations
- Potential for CI failures blocking development (intentional friction)
- GitHub Actions minutes usage (free tier should be sufficient)
- Learning curve for contributors unfamiliar with CI systems

**Benefits:**
- Consistent code quality across all contributions
- Early detection of bugs, style issues, and security vulnerabilities
- Reduced code review burden (mechanical checks automated)
- Documentation of quality standards through configuration
- Clear feedback for contributors on what needs fixing
- Protection of production code determinism guarantees
- Scalable quality enforcement as team grows

**Expected Challenges:**
- False positives from linters (especially mypy initially)
- Workflow failures due to transient issues (network, GH Actions)
- Balancing strictness with developer productivity
- Keeping CI configuration in sync with local development setup

**Mitigation Strategies:**
- Use `continue-on-error` for gradually-adopted checks (mypy, cargo audit)
- Clear documentation on how to reproduce CI checks locally
- Regular review and tuning of linter configurations
- Fast feedback loops (separate Rust/Python workflows)

### Success Metrics

Track monthly:
- CI pass rate (should be > 95% after initial setup)
- Mean time to fix CI failures (should be < 1 hour)
- Number of bugs caught by automated checks vs code review
- CI runtime (should stay < 10 minutes per workflow)
- Developer satisfaction with CI process

Success indicators:
- Mechanical issues caught before code review
- Consistent code style across repository
- No security vulnerabilities merged to production
- Fast feedback on pull requests
- Low false positive rate from automated checks

### Revisit Conditions

This decision should be revisited if:
- CI becomes a bottleneck to development (> 15 minute runs)
- False positive rate exceeds 10% of workflow failures
- GitHub Actions costs become significant
- Alternative CI platforms offer substantially better features
- Workflow complexity becomes unmaintainable

This decision should NOT be revisited due to:
- Individual CI failures (expected and desired behavior)
- Complaints about strictness without quality degradation evidence
- Desire to bypass checks for "quick fixes"

### References
- `.github/workflows/build-core.yml` — Rust core CI workflow
- `.github/workflows/build-research.yml` — Python research CI workflow
- `core/rust/README.md` — Production layer documentation
- `research/python/README.md` — Research layer documentation
- [Rust vs Python Boundary Enforcement](../02-system-architecture/rust-python-boundary-enforcement.md)
- [GitHub Actions Documentation](https://docs.github.com/en/actions)

---

# Decision Log Template

The following sections define the template for future decision log entries.

---

## Context Requirement

Context must describe:
- The problem being solved
- Constraints at the time
- Relevant documents or principles

Decisions without context are not defensible.

---

## Alternatives Requirement

At least one alternative must be documented.

Rejected alternatives should include:
- Why they were attractive
- Why they were rejected

This prevents rediscovery of rejected ideas.

---

## Rationale Requirement

Rationale must:
- Reference design principles
- Reference market theory where applicable
- Avoid outcome-based justification

Performance alone is insufficient.

---

## Trade-offs and Consequences

Every decision has costs.

These must be:
- Explicit
- Accepted knowingly
- Linked to future risk

Undocumented trade-offs are future failures.

---

## Revisit Conditions

Every decision must define:
- Conditions under which it should be revisited
- Signals that assumptions may be invalid

Decisions are not permanent.

---

## Governance Rules

- Decision logs are immutable
- Amendments require new entries
- Silent reversals are forbidden

---

## Review Cadence

Decision logs should be reviewed:
- During major refactors
- After significant failures
- When market structure changes

Memory decay is a systemic risk.

---

## Cultural Enforcement

Galactus values:
- Written justification
- Explicit uncertainty
- Reversibility

Strong opinions without records are discouraged.

---

## Final Statement

**Galactus remembers its decisions  
so it does not have to relearn them painfully.**
