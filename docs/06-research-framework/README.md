# Research Framework Documentation

**Structured discipline for research and experimentation in Galactus**

This directory contains the complete framework for conducting research, designing experiments, and promoting findings to production.

---

## Purpose

The research framework exists to:
- Prevent p-hacking and overfitting
- Enforce structural discipline in hypothesis testing
- Maintain the boundary between discovery and production
- Ensure all promoted signals are sound, explainable, and stable

Research in Galactus is how we learn.  
This framework is how we avoid self-deception.

---

## Core Documents

### Research Philosophy and Methodology

**[`research-methodology.md`](research-methodology.md)**  
Defines the core research philosophy and methodology.

- Hypothesis-driven research principles
- Research stages and workflow
- Data discipline and reproducibility
- Avoiding look-ahead bias
- Negative results handling

**Start here** to understand how research operates in Galactus.

---

### Experiment Design Framework

**[`experiment-design.md`](experiment-design.md)** ⭐ **Primary Reference**  
Complete framework for experiment design, constraints, and discipline.

- Experiment definition and principles
- Detailed framing requirements
- Comprehensive constraint specifications
- Extensive failure expectations framework
- Complete documentation requirements

**Use this** for understanding experiment design principles.

**[`experiment-template.md`](experiment-template.md)**  
Blank template for documenting experiments.

- Structured sections for all required documentation
- Hypothesis statement format
- Methodology documentation
- Results and failure analysis templates
- Recommendation framework

**Use this** when starting a new experiment.

**[`experiment-checklist.md`](experiment-checklist.md)**  
Quick validation checklist for experiment quality.

- Pre-experiment validation checklist
- During-experiment monitoring checklist  
- Post-experiment evaluation checklist
- Pre-promotion checklist
- Common mistakes to avoid

**Use this** to validate your experiment at each stage.

**[`experiment-examples.md`](experiment-examples.md)**  
Practical examples of well-framed experiments.

- Complete example: FII derivatives roll-over pressure
- Complete example: Mutual fund deployment lag
- Failure mode documentation examples
- Constraint specification examples
- Anti-pattern warnings

**Use this** to see how proper experiments look in practice.

---

### Feature Discovery

**[`feature-discovery-rules.md`](feature-discovery-rules.md)**  
Rules for discovering and validating features.

- Feature definition and eligibility
- Discovery scope and boundaries
- Hypothesis requirements
- Parameter discipline
- Documentation standards

**Use this** when exploring new features.

---

### Validation and Promotion

**[`backtesting-guidelines.md`](backtesting-guidelines.md)**  
Standards for backtesting and validation.

- What backtesting may/may not evaluate
- Event-time alignment requirements
- Regime-aware testing
- Counterfactual testing
- Reproducibility standards

**Use this** when validating signal candidates.

**[`promotion-checklist.md`](promotion-checklist.md)**  
Mandatory checklist for promoting research to production.

- Structural validity requirements (non-negotiable)
- Determinism and reproducibility
- Data discipline
- Regime awareness
- Failure analysis (mandatory)
- Rust readiness

**Use this** before considering any promotion to production.

---

## Research Workflow

### 1. Start New Research

```
Read: research-methodology.md
Read: experiment-design.md
Use:  experiment-template.md (copy and fill in)
```

### 2. Design Experiment

```
Use:  experiment-checklist.md (pre-experiment section)
Ref:  experiment-examples.md (for guidance)
Check: All data sources in ../03-data-and-schemas/data-sources.md
```

### 3. Execute Experiment

```
Use:  experiment-checklist.md (during-experiment section)
Track: All deviations and observations
Document: Failures honestly as they occur
```

### 4. Analyze Results

```
Use:  experiment-checklist.md (post-experiment section)
Ref:  experiment-examples.md (failure documentation)
Complete: All sections of experiment-template.md
```

### 5. Consider Promotion

```
If: Recommendation is "PROMOTE"
Then:
  1. Complete: promotion-checklist.md (all items)
  2. Validate: backtesting-guidelines.md requirements
  3. Document: In decision log
  4. Prepare: Rust implementation spec
```

---

## Key Principles

### Research Philosophy

1. **Most hypotheses are wrong** — research exists to invalidate quickly
2. **Performance is not proof** — understanding matters more than optimization
3. **Failure is valuable** — document and learn from failures
4. **Determinism over cleverness** — reproducibility is mandatory
5. **Inference over prediction** — model capital behavior, not price forecasting

### Experiment Quality Standards

- ✅ Hypothesis maps to capital behavior
- ✅ Constraints are explicitly specified
- ✅ At least 3 failure modes documented
- ✅ Regime boundaries clearly defined
- ✅ Results reproducible by others
- ✅ Honest about limitations

### Promotion Philosophy

**Promotion is not a reward.**  
**It is a commitment to correctness under scrutiny.**

Galactus promotes slowly so it can remain trusted forever.

---

## Document Status

| Document | Status | Last Updated |
|----------|--------|--------------|
| research-methodology.md | Stable | v1.0 |
| experiment-design.md | Current | v1.1 |
| experiment-template.md | Current | v1.0 |
| experiment-checklist.md | Current | v1.0 |
| experiment-examples.md | Current | v1.0 |
| feature-discovery-rules.md | Stable | v1.0 |
| backtesting-guidelines.md | Stable | v1.0 |
| promotion-checklist.md | Stable | v1.0 |

---

## Related Documentation

### Market Theory and Architecture
- [`../01-market-theory/`](../01-market-theory/) — Capital behavior models
- [`../02-system-architecture/`](../02-system-architecture/) — System boundaries

### Data and Signals
- [`../03-data-and-schemas/`](../03-data-and-schemas/) — Data sources and schemas
- [`../04-signal-and-metrics/`](../04-signal-and-metrics/) — Signal philosophy

### Validation and Risk
- [`../07-backtesting-and-validation/`](../07-backtesting-and-validation/) — Validation frameworks
- [`../08-risk-and-failure-modes/`](../08-risk-and-failure-modes/) — Risk management

### Research Environment
- [`../../research/python/README.md`](../../research/python/README.md) — Python research layer

---

## Quick Reference Card

**Starting new experiment?**  
→ Use `experiment-template.md`

**Need examples?**  
→ Read `experiment-examples.md`

**Validating experiment?**  
→ Check `experiment-checklist.md`

**Ready to promote?**  
→ Complete `promotion-checklist.md`

**Understanding philosophy?**  
→ Read `research-methodology.md`

**Need complete framework?**  
→ Read `experiment-design.md`

---

## Enforcement

Violations of these frameworks are architectural issues:

1. **Experiments without documented failures** → Incomplete, not promotable
2. **Promotion without checklist completion** → Rejected
3. **Outcome-driven framing** → Research failure, restart required
4. **Look-ahead bias** → Results invalidated
5. **Bypassed promotion process** → Revert and require proper process

These rules exist to protect Galactus integrity.

---

## Contributing

When contributing to research framework documentation:

1. Ensure alignment with core vision ([`../00-vision-and-non-goals/`](../00-vision-and-non-goals/))
2. Maintain consistency with existing framework
3. Update this README if adding new documents
4. Document rationale in decision log
5. Get review before merging

---

## Final Statement

**Research in Galactus exists to reduce ignorance, not to create confidence.**

Truth survives scrutiny. Hypotheses do not.

When in doubt, document failures honestly and promote slowly.
