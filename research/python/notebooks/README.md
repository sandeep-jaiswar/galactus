# Example Research Notebook

This directory contains Jupyter notebooks for exploratory analysis.

## Organization

```
notebooks/
├── exploratory/        # Ad-hoc investigations
│   └── YYYY-MM-DD-descriptive-name.ipynb
├── validation/         # Signal validation studies  
│   └── validate-signal-name-YYYY-MM-DD.ipynb
└── backtesting/        # Backtest analysis
    └── backtest-signal-name-YYYY-MM-DD.ipynb
```

## Naming Convention

Use ISO date prefixes: `YYYY-MM-DD-description.ipynb`

Examples:
- `2024-12-27-explore-options-volume.ipynb`
- `2024-12-28-validate-capital-pressure-signal.ipynb`

## Best Practices

1. **Start with a clear question** — What are you investigating?
2. **Document assumptions** — What are you assuming about the data?
3. **Show failure cases** — Don't cherry-pick results
4. **Summarize findings** — Add a conclusion section
5. **Clean up before committing** — Clear large outputs if needed

## Archiving

- Keep useful notebooks
- Archive one-off analysis after use
- Preserve notebooks for promoted signals

## Remember

These notebooks are for **research and discovery**, not production inference.

For production implementation, follow the [promotion checklist](../../docs/06-research-framework/promotion-checklist.md).
