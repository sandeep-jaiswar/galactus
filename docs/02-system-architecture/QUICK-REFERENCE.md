# Rust vs Python Separation — Quick Reference

**This is a quick reference for developers. For complete details, see the full documentation.**

---

## The Golden Rule

**Python discovers truth. Rust enforces truth.**

The boundary between them is **sacred** and **non-negotiable**.

---

## Directory Structure

```
galactus/
├── core/rust/              ← Production ONLY (deterministic, tested, documented)
│   ├── src/                   
│   └── tests/              
│
└── research/python/        ← Research ONLY (exploratory, iterative, experimental)
    ├── notebooks/             
    ├── src/                   
    └── tests/              
```

---

## What Goes Where?

### ✅ Core/Rust (Production)

Put code here if it is:
- Production inference logic
- Deterministic (same inputs → same outputs)
- Fully tested (>90% coverage)
- Documented completely
- Passed the [promotion checklist](../06-research-framework/promotion-checklist.md)

### ✅ Research/Python (Research)

Put code here if it is:
- Exploratory analysis
- Experimental features
- Hypothesis testing
- Visualization
- Backtesting
- Not yet ready for production

---

## ❌ Never Do This

### Forbidden: Python in Production Core
```bash
# ❌ WRONG
core/rust/src/analysis.py
```

### Forbidden: Rust in Research Layer
```bash
# ❌ WRONG
research/python/src/production_inference.rs
```

### Forbidden: Shared Code Directories
```bash
# ❌ WRONG
shared/utils.py
common/helpers.rs
lib/mixed_code.py
```

### Forbidden: Production Logic Duplication
```python
# ❌ WRONG - In research/python/
def calculate_capital_pressure():
    # Copying production algorithm from Rust
    ...
```

Instead:
```python
# ✅ CORRECT - In research/python/
import subprocess
result = subprocess.run(['./core/rust/target/release/galactus'], ...)
```

---

## Promotion Workflow

```
Research → Validate → Checklist → Document → Implement → Test → Production
```

### Steps:

1. **Develop in Python** (`research/python/`)
   - Prototype and iterate
   - Explore and validate

2. **Complete Checklist** ([promotion-checklist.md](../06-research-framework/promotion-checklist.md))
   - All items must be satisfied
   - No shortcuts allowed

3. **Document Decision** (decision log entry)
   - Why promote?
   - What evidence supports it?

4. **Implement in Rust** (`core/rust/`)
   - Full test coverage
   - Comprehensive documentation
   - Match research prototype exactly

5. **Validate Output**
   - Rust matches Python prototype
   - No drift or divergence

---

## Automated Checks

### Before committing:
```bash
./scripts/check_boundaries.sh
```

### In CI/CD:
- Boundary integrity validated automatically
- Promotions require checklist reference
- Tests must exist for new production code

---

## Code Review Checklist

### For Rust (Production):
- [ ] Logic promoted via formal process
- [ ] Deterministic (no hidden state)
- [ ] Full test coverage
- [ ] Documentation complete
- [ ] Aligns with design principles

### For Python (Research):
- [ ] Hypothesis clearly stated
- [ ] Methodology sound
- [ ] Failure modes explored
- [ ] Not production inference logic

---

## Common Questions

### Q: Can I quickly prototype something in Rust?
**A:** No. Prototype in Python first. Promote to Rust only after validation.

### Q: Can I copy Python research code directly to Rust?
**A:** No. Follow the promotion checklist. Formalize and test properly.

### Q: Can I call Rust from Python?
**A:** Yes! Call compiled binaries or services. That's the correct pattern.

### Q: Can I call Python from Rust production code?
**A:** No. Rust should not depend on Python at runtime.

### Q: This process seems slow. Can I skip steps?
**A:** No. The process protects determinism and quality. Speed comes with practice.

### Q: What if I'm just fixing a bug?
**A:** Bug fixes in production still need tests. Small changes, full rigor.

---

## Red Flags

🚨 Stop immediately if you hear:
- "We'll formalize it later"
- "Just copy this from research"
- "Too complex to test"
- "Works but I don't know why"
- "Let's skip the checklist this time"

These are **anti-patterns** that violate the boundary.

---

## Getting Help

### Documentation:
- [Rust vs Python Contract](rust-vs-python-contract.md) — The foundational contract
- [Boundary Enforcement](rust-python-boundary-enforcement.md) — Detailed enforcement
- [Promotion Checklist](../06-research-framework/promotion-checklist.md) — Required for promotion

### Scripts:
- `./scripts/check_boundaries.sh` — Validate separation
- `./scripts/validate_promotion.sh` — Check promotion compliance

### README files:
- `core/rust/README.md` — What belongs in production
- `research/python/README.md` — What belongs in research

---

## Success Mantras

✅ **"Python discovers, Rust enforces"**  
✅ **"Same inputs, same outputs"** (determinism)  
✅ **"Promote slowly, trust forever"**  
✅ **"Boring production, exciting research"**  
✅ **"The boundary is sacred"**

---

## Final Statement

**The separation between research and production is not bureaucracy.**

**It is the foundation of Galactus's reliability.**

Respect the boundary. Follow the process. Build trust through rigor.
