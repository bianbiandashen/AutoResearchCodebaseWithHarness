# Research Guide for AI Agents

This guide explains how to effectively use AutoCodeHarness for autonomous research.

## Core Loop

```
┌─────────────────────────────────────────┐
│ 1. Analyze Codebase                     │
│    - Read current structure             │
│    - Review recent experiments          │
│    - Calculate complexity metrics       │
└─────────────┬───────────────────────────┘
              │
┌─────────────▼───────────────────────────┐
│ 2. Generate Hypothesis                  │
│    - Based on patterns observed         │
│    - Consider failed experiments        │
│    - Estimate impact and risk           │
└─────────────┬───────────────────────────┘
              │
┌─────────────▼───────────────────────────┐
│ 3. Design Experiment                    │
│    - Create code changes                │
│    - Set resource limits                │
│    - Choose isolation level             │
└─────────────┬───────────────────────────┘
              │
┌─────────────▼───────────────────────────┐
│ 4. Execute in Harness                   │
│    - Apply changes                      │
│    - Run training                       │
│    - Monitor resources                  │
│    - Collect metrics                    │
└─────────────┬───────────────────────────┘
              │
┌─────────────▼───────────────────────────┐
│ 5. Evaluate Results                     │
│    - Compare metrics                    │
│    - Check simplicity                   │
│    - Decide keep/discard                │
└─────────────┬───────────────────────────┘
              │
       ┌──────┴──────┐
       │             │
┌──────▼─────┐ ┌────▼──────┐
│ Keep       │ │ Discard   │
│ - Commit   │ │ - Revert  │
│ - Log      │ │ - Log     │
└──────┬─────┘ └────┬──────┘
       │             │
       └──────┬──────┘
              │
        ┌─────▼─────┐
        │ Continue  │
        │ Loop      │
        └───────────┘
```

## Hypothesis Generation Strategies

### 1. Pattern-Based

Analyze existing code for opportunities:
- Repeated code → Can we abstract it?
- Complex functions → Can we simplify?
- Magic numbers → Should they be learned parameters?

### 2. Literature-Based

Reference known improvements:
- Flash Attention variants
- Rotary embeddings
- Layer normalization alternatives
- Activation function choices

### 3. Failure-Driven

Learn from crashes and regressions:
- OOM → Reduce model size or batch size
- Timeout → More efficient implementation
- Worse metrics → Revert the approach

### 4. Simplification-First

Try removing code:
- Can we delete this component without hurting performance?
- Is this abstraction actually needed?
- Can we use a simpler algorithm?

## Decision Making

### When to Keep

Keep an experiment if:
1. **Primary**: `val_bpb` improved by any amount
2. **Tiebreaker**: Equal `val_bpb` but lower `simplicity_score`

### When to Discard

Discard if:
1. `val_bpb` degraded
2. Crashed (unless trivial fix available)
3. Complexity increased significantly for minimal gain

### Example Decisions

```
Baseline:  val_bpb=1.000, simplicity=100
Experiment A: val_bpb=0.995, simplicity=150 → KEEP (better metric)
Experiment B: val_bpb=1.000, simplicity=80  → KEEP (simpler)
Experiment C: val_bpb=1.002, simplicity=90  → DISCARD (worse metric)
Experiment D: val_bpb=0.999, simplicity=200 → DISCARD? (marginal gain, much complexity)
```

## Resource Management

### Time Budgets

- Default: 300 seconds (5 minutes)
- Enforced by harness (process will be killed)
- Use for fair comparison across experiments

### Memory Limits

- Set `max_vram_gb` to prevent OOM
- Harness monitors usage every second
- Experiment killed if limit exceeded

### Isolation

Risk levels determine isolation:
- **Low**: Same branch, no isolation needed
- **Medium**: Separate branch, can merge back
- **High**: Separate branch + sandboxed environment

## Common Pitfalls

### 1. Premature Optimization

Don't micro-optimize before establishing baseline. Get something working first.

### 2. Ignoring Simplicity

A 0.001 improvement that adds 50 LOC is usually not worth it. Future you (or future agents) will thank you for keeping it simple.

### 3. Not Reading Experiments Log

Learn from history! Check `results.tsv` before generating similar hypotheses.

### 4. Forgetting to Revert

Always `git reset --hard` failed experiments. Leaving broken code will confuse future iterations.

### 5. Chasing Noise

Sub-0.001 differences might be measurement noise. Need statistical significance.

## Multi-Model Collaboration

### Using Codex for Review

After a successful experiment:

```bash
codex review experiments/exp-20260331-120000/
```

GPT-5.4 can provide:
- Code quality feedback
- Alternative approaches
- Related literature
- Risk assessment

### Division of Labor

- **Claude Code**: Fast execution, codebase navigation, tool use
- **GPT-5.4** (via Codex): Deep reasoning, paper references, architectural critique

## Debugging

### Experiment Crashed

1. Check `experiments/exp-*/run.log` for stack trace
2. Common causes:
   - OOM: Reduce model size or batch size
   - Syntax error: Fix typo and rerun
   - Missing import: Add it
3. If unfixable, log as "crash" and move on

### Metrics Not Improving

After 10+ experiments with no improvement:
- Try more radical changes
- Review literature for new ideas
- Simplify instead (remove code)
- Check if baseline was already optimal

### Stuck in Local Minimum

- Jump to different part of design space
- Try opposite of recent approach
- Combine multiple small improvements
- Reset to earlier checkpoint and branch

## Best Practices

1. **Document reasoning** in commit messages
2. **One change at a time** (easier to attribute impact)
3. **Test extremes** before fine-tuning middle ground
4. **Celebrate simplifications** as much as performance wins
5. **Learn from failures** by reading crash logs carefully

---

Remember: The goal is **iteration velocity**. Run many experiments quickly. Most will fail—that's okay. The winners compound over time.
