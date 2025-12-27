# Experiment Template

**Use this template when designing and documenting experiments in Galactus.**

---

## Experiment Metadata

**Experiment ID**: EXP-YYYY-MM-NNN  
**Title**: [Descriptive experiment title]  
**Researcher**: [Your name]  
**Date Started**: YYYY-MM-DD  
**Date Completed**: YYYY-MM-DD  
**Related Experiments**: [Links to related work]  
**Code Location**: [Path to notebook/code repository]

---

## 1. Hypothesis Statement

### Primary Hypothesis
[State your hypothesis in one clear sentence]

### Capital Identification
- **Capital Pool**: [Which specific capital pool? E.g., FII derivatives positions, domestic MF equity flow]
- **Capital Size/Scale**: [Estimated magnitude or relative scale]
- **Liquidity Constraint**: [What limits the mobility of this capital?]

### Constraint Definition
- **Binding Constraint**: [What forces action? E.g., contract expiry, margin requirement, regulatory limit]
- **Why Non-Optional**: [Why can't this capital delay or avoid action?]
- **Time Horizon**: [Over what period must action occur?]

### Observable Implications
- **Expected Behavior**: [What specific market behavior should result?]
- **Where Observable**: [In which instruments, markets, or time periods?]
- **Magnitude**: [What magnitude is structurally plausible?]

### Falsification Criteria
- **Failure Conditions**: [Under what conditions should this hypothesis fail?]
- **Contradictory Evidence**: [What observations would disprove this?]
- **Alternative Explanations**: [What else could explain the expected observations?]

### Regime Dependency
- **Valid Regimes**: [In which market regimes should this behavior appear?]
- **Invalid Regimes**: [In which regimes should this behavior be absent?]
- **Transition Behavior**: [How should regime transitions affect this?]

---

## 2. Methodology

### Data Sources
- **Source 1**: [Name, version, date range]
- **Source 2**: [Name, version, date range]
- [Confirm all sources are listed in `data-sources.md`]

### Time Period
- **Analysis Window**: YYYY-MM-DD to YYYY-MM-DD
- **Rationale**: [Why this period? Structural justification, not performance]
- **Regimes Covered**: [List market regimes in this period]

### Instrument Scope
- **Included**: [Which instruments/asset classes]
- **Excluded**: [Which instruments/asset classes and why]
- **Justification**: [How does scope affect generalizability?]

### Parameters
| Parameter | Value | Justification |
|-----------|-------|---------------|
| [Name]    | [Val] | [Structural reason for this value] |

### Analysis Approach
[Describe your analytical methodology]

- Step 1: [Description]
- Step 2: [Description]
- Step 3: [Description]

### Control Mechanisms
- **Baseline/Null Hypothesis**: [What are you comparing against?]
- **Negative Controls**: [What tests show when hypothesis should fail?]
- **Counterfactuals**: [What alternative scenarios are tested?]

### Assumptions
1. [Assumption 1 and its impact]
2. [Assumption 2 and its impact]
3. [Assumption 3 and its impact]

### Data Quality Considerations
- [Any data limitations or gaps]
- [Data transformations applied]
- [Handling of missing data]

---

## 3. Results

### Primary Findings

#### Hypothesis Validation
[Does the evidence support or refute the hypothesis? Be specific and honest.]

#### Structural Observations
[Key observations related to capital behavior, not performance]

1. **Observation 1**: [Description]
   - Supporting Evidence: [Data/metrics]
   - Structural Interpretation: [Why this makes sense]

2. **Observation 2**: [Description]
   - Supporting Evidence: [Data/metrics]
   - Structural Interpretation: [Why this makes sense]

### Regime-Specific Behavior

| Regime | Behavior | Strength | Notes |
|--------|----------|----------|-------|
| [Name] | [Desc]   | [High/Med/Low] | [Details] |

### Unexpected Observations
[Anything that surprised you or contradicted expectations]

### Statistical Summary
[If applicable, key statistics. Focus on distributions, not point estimates.]

### Visualizations
[Include key plots with clear captions explaining structural significance]

---

## 4. Failure Analysis

### Failure Mode 1: [Name]
- **Description**: [What fails and how]
- **Root Cause**: [Why does it fail structurally?]
- **Frequency**: [How often does this occur?]
- **Detection**: [How can this failure be identified?]
- **Impact**: [Consequences of this failure]
- **Mitigation**: [How should system respond?]
- **Historical Examples**: [Specific dates/events where this occurred]

### Failure Mode 2: [Name]
- **Description**: [What fails and how]
- **Root Cause**: [Why does it fail structurally?]
- **Frequency**: [How often does this occur?]
- **Detection**: [How can this failure be identified?]
- **Impact**: [Consequences of this failure]
- **Mitigation**: [How should system respond?]
- **Historical Examples**: [Specific dates/events where this occurred]

### Failure Mode 3: [Name]
- **Description**: [What fails and how]
- **Root Cause**: [Why does it fail structurally?]
- **Frequency**: [How often does this occur?]
- **Detection**: [How can this failure be identified?]
- **Impact**: [Consequences of this failure]
- **Mitigation**: [How should system respond?]
- **Historical Examples**: [Specific dates/events where this occurred]

### Stress Test Results
[How did the hypothesis perform during known stress periods?]

### Failure Detection Mechanisms
[Automated checks or indicators that can identify failures]

---

## 5. Regime Analysis

### Behavior by Regime

#### Regime 1: [Name, e.g., "High Volatility, Derivatives-Driven"]
- **Behavior**: [How hypothesis manifests]
- **Strength**: [Strong/Moderate/Weak]
- **Confidence**: [High/Medium/Low]
- **Notes**: [Additional observations]

#### Regime 2: [Name]
- **Behavior**: [How hypothesis manifests]
- **Strength**: [Strong/Moderate/Weak]
- **Confidence**: [High/Medium/Low]
- **Notes**: [Additional observations]

### Regime Transitions
[How does behavior change at regime boundaries?]

### Boundaries of Applicability
- **Valid In**: [List regimes where hypothesis holds]
- **Invalid In**: [List regimes where hypothesis does not apply]
- **Ambiguous In**: [Regimes with uncertain applicability]

### Confidence Degradation
[When and how should confidence degrade?]

---

## 6. Interpretation

### Structural Explanation
[Explain results in terms of capital behavior and market structure]

[Reference relevant sections of market theory documentation]

### Alignment with Theory
- **Consistent With**: [Which documented theories/models]
- **Extends**: [How does this add to existing understanding]
- **Contradicts**: [Any conflicts with existing theory - if so, explain]

### Alternative Interpretations
1. [Alternative explanation 1 and why it's less likely]
2. [Alternative explanation 2 and why it's less likely]

### Limitations
- [Limitation 1]
- [Limitation 2]
- [Limitation 3]

### Confidence Assessment
**Overall Confidence in Hypothesis**: [High/Medium/Low]

**Rationale**: [Why this confidence level?]

---

## 7. Learning Outcomes

### Key Learnings
1. [What did we learn about capital behavior?]
2. [What did we learn about market structure?]
3. [What did we learn about our methodology?]

### Implications for Future Research
[How does this inform future experiments?]

### Contribution to Market Understanding
[What does this add to our structural knowledge?]

### Methodology Improvements
[What would you do differently next time?]

---

## 8. Recommendation

### Decision
☐ **PROMOTE** — Ready for promotion consideration  
☐ **REFINE** — Needs additional work  
☐ **REJECT** — Hypothesis invalidated  

### Rationale
[Detailed reasoning for your recommendation]

### If PROMOTE:
- Promotion Checklist Status: [X / Y items complete]
- Rust Implementation Complexity: [Low/Medium/High]
- Documentation Status: [Complete/Incomplete]
- Testing Requirements: [List needed tests]

### If REFINE:
- Specific Improvements Needed:
  1. [Improvement 1]
  2. [Improvement 2]
- Estimated Effort: [Time estimate]
- Priority: [High/Medium/Low]

### If REJECT:
- Primary Reason: [Why rejected]
- Lessons Learned: [What we learned from the rejection]
- Related Areas to Explore: [What might be worth trying instead]

---

## 9. Open Questions

### Unresolved Questions
1. [Question 1]
2. [Question 2]
3. [Question 3]

### Suggested Follow-Up Experiments
1. [Experiment idea 1]
   - Rationale: [Why this would be valuable]
   
2. [Experiment idea 2]
   - Rationale: [Why this would be valuable]

### Known Limitations Requiring Further Investigation
- [Limitation 1 and how to address it]
- [Limitation 2 and how to address it]

---

## 10. Reproducibility Checklist

- [ ] All data sources documented and accessible
- [ ] Code is version controlled and tagged
- [ ] Parameters are explicitly stated
- [ ] Random seeds fixed (if applicable)
- [ ] Dependencies documented
- [ ] Environment specifications recorded
- [ ] Another researcher could reproduce this work

---

## 11. Review Sign-Off

**Researcher**: [Your name] — [Date]  
**Peer Reviewer**: [Name] — [Date]  
**Status**: [Draft/In Review/Approved/Archived]

### Review Comments
[Peer reviewer feedback]

---

## References

- Related Experiments: [Links]
- Relevant Documentation: [Links to theory docs]
- Code Repository: [Link]
- Data Sources: [Links to data documentation]

---

**Remember**: 
- Honesty about failures is mandatory
- Structural explanation is required
- Performance is secondary to understanding
- Silence is preferable to confident nonsense
