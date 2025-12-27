# System Architecture Documentation

This directory contains the architectural documentation for Project Galactus.

---

## Start Here

### 🎯 [**System Boundaries**](system-boundaries.md)
**The foundational document defining what Galactus is and is not.**

Read this first to understand:
- The four core system boundaries (Ingestion, Inference, Research, Outputs)
- What is explicitly excluded (Execution and Strategy layers)
- Why these boundaries exist
- How boundaries are enforced

---

## Core Architecture Documents

### [High-Level Design](high-level-design.md)
Detailed system architecture covering:
- System objectives and architectural principles
- The six major layers and their responsibilities
- Data flow and component interactions
- Technology choices and design characteristics

### [Component Boundaries](component-boundaries.md)
Component responsibility definitions:
- Detailed ownership for each component
- Allowed and forbidden behaviors
- Cross-boundary communication rules
- Boundary enforcement mechanisms

### [Event-Driven Architecture](event-driven-architecture.md)
Event processing model:
- Why Galactus is event-driven (not tick-driven)
- Event types and time semantics
- Ordering guarantees and replayability
- Windowing and failure handling

### [Rust vs Python Contract](rust-vs-python-contract.md)
Technology separation contract:
- Role of Python (research and discovery)
- Role of Rust (production enforcement)
- Promotion process from research to production
- Interface patterns and testing requirements

### [Rust Python Boundary Enforcement](rust-python-boundary-enforcement.md)
Concrete enforcement mechanisms:
- Physical directory separation
- Automated CI/CD checks
- Promotion validation workflow
- Violation detection and response
- Cultural norms and team practices

---

## Document Relationships

```
system-boundaries.md          ← START HERE (defines WHAT the boundaries are)
    │
    ├─▶ high-level-design.md         (details HOW layers work)
    │
    ├─▶ component-boundaries.md      (defines component responsibilities)
    │
    ├─▶ event-driven-architecture.md (explains data flow semantics)
    │
    └─▶ rust-vs-python-contract.md   (defines technology separation)
```

---

## Reading Order

1. **First time?** Start with [`system-boundaries.md`](system-boundaries.md)
2. **Need implementation details?** Read [`high-level-design.md`](high-level-design.md)
3. **Building a component?** Check [`component-boundaries.md`](component-boundaries.md)
4. **Working with data flow?** Review [`event-driven-architecture.md`](event-driven-architecture.md)
5. **Choosing tech stack?** See [`rust-vs-python-contract.md`](rust-vs-python-contract.md)

---

## Key Architectural Principles

1. **Event-driven, not tick-driven** — React to structural changes, not every price tick
2. **Deterministic core** — Same inputs always produce same outputs
3. **Separation of research and production** — Python discovers, Rust enforces
4. **Replayability first** — All inference can be replayed from events
5. **Explainability over complexity** — Every output must be explainable

---

## Boundaries Are Sacred

All documents in this directory support the core principle:

**Galactus has exactly four boundaries: Ingestion, Inference, Research, and Outputs.**

**Execution and Strategy are permanently outside these boundaries.**

Any architectural change must be justified against [`system-boundaries.md`](system-boundaries.md).

---

## Related Documentation

- **Vision and Non-Goals**: [`../00-vision-and-non-goals/`](../00-vision-and-non-goals/)
- **Decision Log**: [`../11-decision-log/`](../11-decision-log/)
- **Design Principles**: [`../00-vision-and-non-goals/design-principles.md`](../00-vision-and-non-goals/design-principles.md)
