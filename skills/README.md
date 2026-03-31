# Skills

Reusable research workflow templates that can be invoked by agents.

## Structure

Each skill is a self-contained workflow that agents can execute:

```
skills/
├── architecture-search/
│   ├── skill.toml          # Skill metadata
│   └── template.rs         # Code template
├── hyperparameter-sweep/
│   ├── skill.toml
│   └── sweep.toml         # Parameter ranges
└── ...
```

## Creating a Skill

A skill consists of:

1. **Metadata** (`skill.toml`):
```toml
name = "architecture-search"
description = "Search for optimal model architecture"
risk_level = "medium"
expected_experiments = 20
```

2. **Implementation**: Code templates, parameter ranges, or execution scripts

## Built-in Skills

(To be implemented)

- `architecture-search`: Vary model depth, width, attention patterns
- `hyperparameter-sweep`: Grid search over learning rates, batch sizes
- `optimizer-ablation`: Test different optimizers (Adam, SGD, Muon, etc.)
- `simplification`: Remove components and test if performance drops
- `scaling-laws`: Measure compute-optimal model sizes

## Using Skills

From an agent:

```rust
use autocodeharness::skills::load_skill;

let skill = load_skill("architecture-search")?;
let experiments = skill.generate_experiments()?;

for exp in experiments {
    let result = harness.run(exp).await?;
    // ...
}
```

From Claude Code:

```markdown
Run the architecture-search skill with depth range 4-12
```
