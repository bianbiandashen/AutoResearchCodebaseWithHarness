# AutoCodeHarness - Agent Development Guide

**AutoCodeHarness** is a universal Rust-based autonomous research framework that combines automated research workflows with codebase intelligence. This is the primary documentation for AI agents working on this codebase.

## Philosophy

The repository is the source of truth. Agents read, modify, and evolve this codebase following these principles:

1. **Code Repository as System of Record** - All research state, experiment history, and architectural decisions live in version control
2. **Agent-First Development** - Documentation and code are optimized for AI agent comprehension and autonomous modification
3. **Iterative Research Loop** - Continuous hypothesis generation → experiment → evaluation → integration cycle

## Architecture Overview

```
autocodeharness/
├── AGENTS.md           # You are here - agent interface contract
├── src/
│   ├── lib.rs          # Core library entry point
│   ├── research/       # Research workflow engine
│   ├── harness/        # Execution harness & experiment runner
│   ├── codebase/       # Codebase analysis & intelligence
│   └── integration/    # Integration with external tools (codex, mcp, etc)
├── experiments/        # Experiment definitions & results
├── skills/             # Reusable research skills
└── docs/               # Agent-readable documentation
```

## Core Components

### 1. Research Engine (`src/research/`)
- **Hypothesis Generation**: Autonomous generation of research hypotheses based on codebase analysis
- **Experiment Design**: Template-based experiment creation with parameter sweeps
- **Result Analysis**: Statistical analysis and visualization of experiment outcomes

### 2. Harness (`src/harness/`)
- **Execution Environment**: Sandboxed, reproducible execution for experiments
- **Resource Management**: GPU/CPU allocation, memory limits, time budgets
- **Monitoring**: Real-time metrics collection (VRAM, MFU, training loss, etc.)

### 3. Codebase Intelligence (`src/codebase/`)
- **AST Analysis**: Parse and understand code structure
- **Dependency Tracking**: Map relationships between modules, functions, types
- **Change Impact**: Predict blast radius of code modifications
- **Pattern Recognition**: Identify architectural patterns and anti-patterns

### 4. Integration Layer (`src/integration/`)
- **MCP Server**: Model Context Protocol for Claude Code integration
- **Codex CLI**: Integration with OpenAI Codex for multi-model collaboration
- **Git Operations**: Automated branching, committing, and experiment tracking

## Agent Workflow

### Research Loop

```rust
loop {
    // 1. Analyze current state
    let codebase_state = analyze_codebase()?;
    let metrics = load_recent_metrics()?;
    
    // 2. Generate hypothesis
    let hypothesis = generate_hypothesis(codebase_state, metrics)?;
    
    // 3. Design experiment
    let experiment = design_experiment(hypothesis)?;
    
    // 4. Execute in harness
    let result = harness.run(experiment).await?;
    
    // 5. Evaluate & decide
    if result.improved(metrics) {
        git_commit(&format!("feat: {}", hypothesis.description))?;
        advance_branch();
    } else {
        git_reset_hard("HEAD~1")?;
    }
    
    // 6. Log results
    log_to_tsv(result)?;
}
```

### Experiment Anatomy

Each experiment is:
1. **Isolated**: Runs in separate branch (`research/exp-{timestamp}`)
2. **Reproducible**: Fixed random seeds, deterministic builds
3. **Time-Boxed**: Maximum runtime budget enforced by harness
4. **Comparable**: Standard metrics (val_bpb, VRAM, MFU) for fair comparison

## Agent Capabilities

### What Agents CAN Do
- Modify any Rust source code in `src/`
- Create new experiments in `experiments/`
- Generate hypotheses based on codebase analysis
- Run experiments with resource limits
- Commit changes that improve metrics
- Revert changes that degrade performance
- Update this documentation when architecture evolves

### What Agents CANNOT Do
- Break reproducibility (no random seeds without explicit justification)
- Exceed resource budgets without approval
- Modify experiment results retroactively
- Skip evaluation phase
- Push to main branch without review (use PR flow)

## Metrics & Success Criteria

Primary metrics (lower is better):
- **val_bpb**: Validation bits-per-byte (vocab-independent)
- **training_time**: Wall-clock seconds (fixed budget: 300s)
- **peak_vram_mb**: Maximum GPU memory usage

Secondary metrics:
- **mfu_percent**: Model FLOPs Utilization
- **total_tokens_M**: Throughput in millions of tokens
- **simplicity_score**: Lines of code + cyclomatic complexity (lower is better)

## Simplicity Principle

**All else being equal, simpler is better.**

- A 0.001 val_bpb improvement requiring 50 LOC of complex code? **Discard.**
- A 0.001 val_bpb improvement by deleting 20 LOC? **Keep immediately.**
- Equal performance with 30% less code? **Major win.**

Complexity is technical debt. The harness should make this quantifiable.

## Integration Points

### Claude Code (via MCP)
```bash
# Add AutoCodeHarness MCP server
claude mcp add autocodeharness -s user -- cargo run --bin mcp-server
```

### Codex (for multi-model review)
```bash
# Review experiment with GPT-5.4
codex review experiments/exp-20260331-142857/
```

## Development Principles

1. **Types over tests**: Use Rust's type system to make invalid states unrepresentable
2. **Errors as values**: `Result<T, E>` everywhere, no panics in production code
3. **Zero-copy where possible**: Use references and smart pointers, avoid unnecessary clones
4. **Async for I/O**: Tokio runtime for all network/disk operations
5. **Docs in code**: Document intent, not mechanics. Let types speak for themselves.

## Next Steps for Agents

On first run, an agent should:

1. Read this file completely (you're doing it now!)
2. Explore `src/lib.rs` to understand module structure
3. Check `experiments/` for recent research history
4. Review `skills/` for reusable workflow templates
5. Generate first hypothesis based on current codebase state
6. Create and run a baseline experiment to establish metrics

---

**Remember**: This repository is alive. It evolves through agent iterations. When you discover better patterns, update this file. When you simplify the architecture, reflect that here. The code repository is the system of record.

Last updated: 2026-03-31 (Initial creation)
