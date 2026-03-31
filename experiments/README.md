# Experiments

This directory contains experiment definitions and results. Each experiment is tracked in Git on a separate branch (`research/exp-{timestamp}`).

## Structure

```
experiments/
├── exp-20260331-120000/
│   ├── hypothesis.json      # The research hypothesis
│   ├── changes.diff         # Code changes made
│   ├── run.log             # Training output
│   └── metrics.json        # Final metrics
└── ...
```

## Experiment Lifecycle

1. **Generate**: Research engine creates hypothesis
2. **Branch**: Create `research/exp-{timestamp}` branch
3. **Apply**: Harness applies code changes
4. **Execute**: Run training for fixed time budget
5. **Evaluate**: Compare metrics against baseline
6. **Decide**: Commit (if improved) or revert (if not)
7. **Log**: Record result in `results.tsv`

## Metrics

All experiments record:
- **val_bpb**: Validation bits-per-byte (primary metric)
- **training_seconds**: Wall-clock training time
- **peak_vram_mb**: Maximum GPU memory usage
- **mfu_percent**: Model FLOPs utilization
- **total_tokens_m**: Throughput
- **simplicity_score**: Code complexity (LOC + cyclomatic)

## Viewing Results

```bash
# View all experiment results
cat results.tsv | column -t -s $'\t'

# View only kept experiments
grep "keep" results.tsv | column -t -s $'\t'

# View crashed experiments
grep "crash" results.tsv | column -t -s $'\t'

# Get best result so far
sort -t$'\t' -k2 -n results.tsv | head -2 | tail -1
```
