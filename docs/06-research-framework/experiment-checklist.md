# Experiment Design Checklist

**Quick reference checklist for experiment validation in Galactus**

Use this checklist at each stage of your experiment lifecycle to ensure compliance with Galactus principles.

---

## Pre-Experiment Checklist

**Complete BEFORE starting any experiment**

### Hypothesis Validation
- [ ] Hypothesis clearly stated in one sentence
- [ ] Capital pool explicitly identified
- [ ] Binding constraint clearly defined
- [ ] Observable implications specified
- [ ] Falsification criteria documented
- [ ] Regime applicability defined

### Structural Grounding
- [ ] Hypothesis maps to documented market theory
- [ ] Mechanism explanation is structural, not narrative
- [ ] Alignment with Indian market structure validated
- [ ] Capital behavior is primary, not price behavior

### Data Compliance
- [ ] All data sources are approved and public
- [ ] Data sources are documented in `data-sources.md`
- [ ] No proprietary or privileged data used
- [ ] No personalized or user-level data included
- [ ] Schema versions identified

### Methodology Design
- [ ] Analysis approach is deterministic
- [ ] Parameters minimized (prefer 0-3)
- [ ] Each parameter has structural justification
- [ ] Time window selection justified structurally
- [ ] Control mechanisms defined
- [ ] Negative controls included

### Failure Planning
- [ ] At least 3 failure modes anticipated
- [ ] Failure detection mechanisms planned
- [ ] Stress scenarios identified
- [ ] Counterfactual tests designed

### Resource Planning
- [ ] Estimated computation time < 1 hour
- [ ] Code location determined
- [ ] Documentation template prepared
- [ ] Reproducibility requirements understood

**🛑 Do not proceed if any item is incomplete**

---

## During-Experiment Checklist

**Monitor these items WHILE conducting the experiment**

### Execution Discipline
- [ ] Following pre-defined methodology (no ad-hoc changes)
- [ ] No parameter tuning based on intermediate results
- [ ] Documenting unexpected observations
- [ ] Maintaining research log/notes
- [ ] Tracking deviations from plan

### Data Integrity
- [ ] Event time alignment maintained
- [ ] No look-ahead bias introduced
- [ ] Publication delays respected
- [ ] Missing data handled explicitly
- [ ] Data transformations documented

### Intermediate Validation
- [ ] Sanity checks performed on intermediate results
- [ ] Distributions examined (not just point estimates)
- [ ] Outliers investigated
- [ ] Edge cases tested
- [ ] Failure modes encountered and documented

### Code Quality
- [ ] Code is readable and commented
- [ ] Version control commits made regularly
- [ ] Reproducible from raw data
- [ ] Dependencies documented

### Honest Assessment
- [ ] Recording failures honestly
- [ ] Not cherry-picking favorable periods
- [ ] Noting conflicting evidence
- [ ] Acknowledging limitations as they emerge

---

## Post-Experiment Checklist

**Complete AFTER analysis, BEFORE drawing conclusions**

### Results Validation
- [ ] Results address original hypothesis directly
- [ ] Findings are reproducible
- [ ] Statistical summaries completed
- [ ] Regime-specific behavior analyzed
- [ ] Visualizations created with clear captions

### Failure Analysis
- [ ] At least 3 failure modes documented
- [ ] Each failure has structural explanation
- [ ] Historical failure examples identified
- [ ] Detection mechanisms defined
- [ ] Response strategies specified
- [ ] Stress test results analyzed

### Regime Analysis
- [ ] Behavior across multiple regimes examined
- [ ] Regime transitions analyzed
- [ ] Boundaries of applicability clear
- [ ] Confidence degradation patterns identified
- [ ] Invalid regimes explicitly listed

### Interpretation
- [ ] Structural explanation provided
- [ ] Alignment with market theory verified
- [ ] Alternative interpretations considered
- [ ] Limitations acknowledged
- [ ] Confidence level justified

### Documentation Complete
- [ ] All metadata filled in
- [ ] Hypothesis section complete
- [ ] Methodology section complete
- [ ] Results section complete
- [ ] Failure analysis section complete
- [ ] Regime analysis section complete
- [ ] Interpretation section complete
- [ ] Learning outcomes documented
- [ ] Recommendation made (Promote/Refine/Reject)
- [ ] Open questions listed

### Quality Standards
- [ ] Documentation is discoverable
- [ ] Work is reproducible by another researcher
- [ ] Failures reported honestly
- [ ] Context and related work linked
- [ ] Clear outcome and recommendation provided

### No Forbidden Patterns
- [ ] No outcome-led framing used
- [ ] No selective time period cherry-picking
- [ ] No post-hoc parameter tuning
- [ ] No silent data exclusions
- [ ] No narrative-only explanations
- [ ] No unexplained failures
- [ ] No attribution to "market irrationality"

---

## Pre-Promotion Checklist

**Complete BEFORE considering promotion to production**

### Structural Validity
- [ ] Capital behavior mapping is explicit
- [ ] Market structure alignment confirmed
- [ ] Mechanism is not contradictory to theory

### Determinism
- [ ] Mathematical/logical definition is explicit
- [ ] No randomness or implicit state
- [ ] Same inputs always produce same outputs
- [ ] Replayable from raw canonical events

### Data Discipline
- [ ] Only approved public data sources used
- [ ] Canonical schemas respected
- [ ] Schema versioning handled
- [ ] Missing data handled explicitly

### Normalization
- [ ] Capital size normalized
- [ ] Liquidity normalized
- [ ] Time urgency modeled explicitly
- [ ] Cross-instrument validity maintained

### Regime Awareness
- [ ] Applicable regimes documented
- [ ] Invalid regimes documented
- [ ] Regime transition behavior understood
- [ ] Confidence degrades under ambiguity

### Failure Analysis (Mandatory)
- [ ] At least 3 realistic failure scenarios documented
- [ ] Structural reasons for failures explained
- [ ] Historical failure evidence provided
- [ ] Failures not attributed to "unexpected price action"

### Backtesting Integrity
- [ ] Framed around structure, not profit
- [ ] Event-time aligned
- [ ] No forward-looking bias
- [ ] Stable across multiple regimes
- [ ] No fragile parameter dependence

### Interpretability
- [ ] Explainable without charts
- [ ] All sub-metrics visible
- [ ] No opaque aggregation
- [ ] Contributions inspectable

### Confidence Integration
- [ ] Confidence calculation explicit
- [ ] Confidence degrades under uncertainty
- [ ] Hard stop conditions defined

### Rust Readiness
- [ ] Implementable in pure Rust
- [ ] No Python-only dependencies
- [ ] Performance characteristics predictable
- [ ] Unit tests defined
- [ ] Golden input/output cases defined
- [ ] Edge cases enumerated

### Documentation
- [ ] Complete promotion checklist documentation
- [ ] Decision log entry prepared
- [ ] Scope explicitly bounded

**See [`promotion-checklist.md`](promotion-checklist.md) for complete promotion requirements**

---

## Review Checklist

**For peer reviewers evaluating experiments**

### Hypothesis Quality
- [ ] Hypothesis is testable and falsifiable
- [ ] Capital behavior is explicitly identified
- [ ] Structural mechanism is clear
- [ ] Failure conditions are specified

### Methodology Rigor
- [ ] Approach is deterministic and reproducible
- [ ] No obvious biases (look-ahead, selection, etc.)
- [ ] Controls are appropriate
- [ ] Parameters are justified structurally

### Data Integrity
- [ ] Data sources are approved
- [ ] Event time alignment verified
- [ ] Assumptions are reasonable and documented

### Results Honesty
- [ ] Failures reported transparently
- [ ] No obvious cherry-picking
- [ ] Limitations acknowledged
- [ ] Alternative interpretations considered

### Documentation Quality
- [ ] All required sections complete
- [ ] Sufficient detail for reproduction
- [ ] Clear recommendation with rationale
- [ ] Code and data are accessible

### Alignment with Galactus Principles
- [ ] Inference over prediction
- [ ] Determinism over cleverness
- [ ] Capital behavior as first-class truth
- [ ] Explainability is mandatory
- [ ] Silence preferred to false confidence

---

## Common Mistakes to Avoid

**🚫 Outcome-Led Research**
- Starting with performance results and working backward
- Framing hypothesis after seeing data
- "This works because returns are good"

**🚫 Look-Ahead Bias**
- Using future information implicitly
- Ignoring publication delays
- Processing time vs event time confusion

**🚫 Parameter Tuning**
- Adjusting parameters after seeing results
- Optimizing for backtest performance
- Lacking structural justification for values

**🚫 Cherry-Picking**
- Selecting favorable time periods
- Omitting failure cases
- Showing only best-performing regimes

**🚫 Narrative Fallacies**
- Explaining with stories instead of structure
- Post-hoc rationalization
- Ignoring alternative explanations

**🚫 Insufficient Failure Analysis**
- "Never fails" claims
- Unexplained failures
- Attributing failures to "irrationality"

**🚫 Opacity**
- Black-box metrics
- Undocumented assumptions
- Unclear methodology
- Missing reproducibility information

---

## Quick Self-Assessment

**Can you answer YES to all of these?**

1. Can someone else reproduce your experiment from your documentation?
2. Can you explain your results without showing charts?
3. Did you identify at least 3 ways your hypothesis could fail?
4. Did you find actual historical examples where it did fail?
5. Is your hypothesis grounded in capital behavior, not price patterns?
6. Would your experiment pass review if results were disappointing?
7. Are you confident there's no look-ahead bias?
8. Can you justify every parameter structurally?
9. Did you test across multiple regimes?
10. Is your recommendation (Promote/Refine/Reject) justified by structure, not performance?

**If NO to any question: Your experiment needs more work**

---

## When to Seek Help

Seek guidance if:
- Hypothesis doesn't map clearly to capital behavior
- Unsure if data source is approved
- Results contradict documented theory
- Failure modes are unclear
- Uncertain about regime classification
- Struggling with structural explanation
- Documentation requirements seem unclear

**Better to ask early than to redo the entire experiment**

---

## Final Reminders

✅ **Failure is valuable** — Document it honestly  
✅ **Structure over performance** — Understand before optimizing  
✅ **Silence is valid** — "No inference" is acceptable  
✅ **Determinism matters** — Must be reproducible  
✅ **Capital first** — Price is secondary  

❌ **Never promote without complete documentation**  
❌ **Never hide failures**  
❌ **Never tune for performance**  
❌ **Never bypass the checklist**

---

**Remember**: The checklist exists to maintain Galactus integrity, not to slow you down. A well-designed experiment satisfies these requirements naturally.

For complete details, see:
- [`experiment-design.md`](experiment-design.md) — Full framework
- [`experiment-template.md`](experiment-template.md) — Documentation template
- [`promotion-checklist.md`](promotion-checklist.md) — Promotion requirements
- [`research-methodology.md`](research-methodology.md) — Research principles
