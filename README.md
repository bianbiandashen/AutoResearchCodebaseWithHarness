# AutoCodeHarness

**Universal Rust-based autonomous research framework combining automated research workflows with codebase intelligence.**

AutoCodeHarness enables AI agents to autonomously conduct research by modifying code, running experiments, evaluating results, and iterating—all while you sleep. Inspired by [Karpathy's autoresearch](https://github.com/karpathy/autoresearch) and [OpenAI's Codex harness architecture](https://openai.com/index/building-codex-using-codex/).

## Key Features

- **Autonomous Research Loop**: Generate hypotheses → design experiments → execute → evaluate → iterate
- **Codebase Intelligence**: AST parsing, dependency tracking, change impact analysis
- **Resource Management**: Time budgets, VRAM limits, automatic OOM detection
- **Multi-Model Collaboration**: Claude Code for execution, GPT-5.4 for review via Codex
- **Simplicity-First**: Quantifiable complexity metrics with automatic simplification rewards
- **Agent-Optimized**: Repository structure designed for AI agent comprehension

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      Research Engine                         │
│  • Hypothesis Generation    • Experiment Design              │
│  • Result Analysis          • Decision Making                │
└──────────────────┬──────────────────────────────────────────┘
                   │
        ┌──────────┴──────────┐
        │                     │
┌───────▼────────┐   ┌────────▼─────────┐
│   Harness      │   │  Codebase        │
│  • Execution   │   │  Intelligence    │
│  • Monitoring  │   │  • AST Parsing   │
│  • Isolation   │   │  • Dependencies  │
└───────┬────────┘   └────────┬─────────┘
        │                     │
        └──────────┬──────────┘
                   │
        ┌──────────▼──────────┐
        │   Integration       │
        │  • MCP Server       │
        │  • Codex CLI        │
        │  • Git Operations   │
        └─────────────────────┘
```

## Quick Start

### Prerequisites

- Rust 1.75+ ([install](https://rustup.rs/))
- Git
- Claude Code CLI ([install](https://claude.ai/code))
- (Optional) Codex CLI for multi-model review

### Installation

```bash
# Clone the repository
git clone https://github.com/your-org/autocodeharness
cd autocodeharness

# Build
cargo build --release

# Run tests
cargo test

# Start MCP server for Claude Code integration
cargo run --bin mcp-server
```

### Configuration

Add AutoCodeHarness as an MCP server in Claude Code:

```bash
claude mcp add autocodeharness -s user -- cargo run --bin mcp-server
```

(Optional) Set up Codex for multi-model review:

```bash
codex setup  # Select gpt-5.4 when prompted
claude mcp add codex -s user -- codex mcp-server
```

### Running Research

```rust
use autocodeharness::{
    research::ResearchEngine,
    harness::Harness,
    codebase::CodebaseAnalyzer,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize
    let analyzer = CodebaseAnalyzer::new(".")?;
    let engine = ResearchEngine::new(analyzer);
    let harness = Harness::builder()
        .time_budget(std::time::Duration::from_secs(300))
        .max_vram(80.0)  // GB
        .build()?;

    // Autonomous research loop
    loop {
        let hypothesis = engine.generate_hypothesis().await?;
        let experiment = engine.design_experiment(&hypothesis)?;
        let result = harness.run(experiment).await?;

        if result.improved() {
            engine.commit(&hypothesis)?;
            println!("✓ Kept: {}", hypothesis.description);
        } else {
            engine.revert()?;
            println!("✗ Discarded: {}", hypothesis.description);
        }
    }
}
```

## Design Principles

### 1. Repository as Source of Truth

All research state lives in version control. No external databases, no hidden state. The Git history *is* the research log.

### 2. Agent-First Development

Code and documentation are optimized for AI agent comprehension. See [AGENTS.md](./AGENTS.md) for the agent interface contract.

### 3. Simplicity as a Feature

Complexity is technical debt. The framework quantifies it:

```rust
simplicity_score = lines_of_code + cyclomatic_complexity
```

When two experiments achieve equal performance, the simpler one wins automatically.

### 4. Fail Fast, Learn Faster

Experiments run in isolated branches with resource limits. OOM? Timeout? Crash? Log it and move on. The goal is iteration velocity.

### 5. Fixed Time Budget

All experiments run for exactly 5 minutes (configurable). This makes results directly comparable regardless of architectural changes (model size, batch size, etc.).

## Project Structure

```
autocodeharness/
├── AGENTS.md              # Agent interface documentation
├── README.md              # You are here
├── Cargo.toml             # Dependencies and build config
├── src/
│   ├── lib.rs             # Core library and types
│   ├── research/          # Research engine
│   │   └── mod.rs
│   ├── harness/           # Execution harness
│   │   └── mod.rs
│   ├── codebase/          # Codebase intelligence
│   │   └── mod.rs
│   ├── integration/       # External tool integration
│   │   ├── mod.rs
│   │   ├── git.rs
│   │   ├── mcp.rs
│   │   └── codex.rs
│   └── bin/
│       ├── mcp_server.rs  # MCP server binary
│       └── harness.rs     # Standalone harness CLI
├── experiments/           # Experiment results (git-tracked)
├── skills/                # Reusable research workflows
└── docs/                  # Agent-readable documentation
```

## Metrics

Primary metrics (lower is better):

- **val_bpb**: Validation bits-per-byte (vocabulary-independent)
- **training_time**: Wall-clock seconds (fixed at 300s)
- **peak_vram_mb**: Maximum GPU memory usage

Secondary metrics:

- **mfu_percent**: Model FLOPs Utilization
- **total_tokens_m**: Throughput in millions of tokens
- **simplicity_score**: LOC + cyclomatic complexity

## Comparison with Other Systems

| Feature | AutoCodeHarness | ARIS | Karpathy's autoresearch |
|---------|-----------------|------|-------------------------|
| Language | Rust | Markdown + Python | Python |
| Scope | Universal codebase | ML research | Single-file training |
| Architecture | Multi-module | Skills-based | Monolithic |
| Codebase Analysis | AST + dependencies | None | None |
| Multi-model | Yes (Codex) | Yes (GPT-5.4) | No |
| Time Budget | Configurable | Per-skill | Fixed 5min |
| Simplicity Tracking | Quantified | No | Implicit |

## Roadmap

- [ ] Implement hypothesis generation using code pattern analysis
- [ ] Add AST parsing for Rust codebases
- [ ] Create MCP server with Claude Code integration
- [ ] Implement resource monitoring (VRAM, CPU, memory)
- [ ] Add Codex CLI integration for multi-model review
- [ ] Build experiment result database (TSV format)
- [ ] Create reusable skill templates
- [ ] Add visualization dashboard for research progress
- [ ] Support distributed experiments across multiple GPUs
- [ ] Implement automated paper generation from experiment logs

## Contributing

This project is in early development. Contributions welcome!

Areas particularly needing help:
- AST parsing for non-Rust languages (Python, C++, etc.)
- Hypothesis generation algorithms
- Resource monitoring for different platforms (MPS, AMD, etc.)
- Integration with other AI frameworks (LangChain, etc.)

## License

MIT License - see [LICENSE](./LICENSE) for details.

## Inspiration & References

- [Karpathy's autoresearch](https://github.com/karpathy/autoresearch) - Single-GPU autonomous research
- [OpenAI's Codex harness](https://openai.com/index/building-codex-using-codex/) - Agent-first development
- [ARIS](https://github.com/wanshuiyin/Auto-claude-code-research-in-sleep) - Markdown-based research workflows
- [nanochat](https://github.com/karpathy/nanochat) - Simplified LLM training

## Citation

If you use AutoCodeHarness in your research, please cite:

```bibtex
@software{autocodeharness2026,
  title = {AutoCodeHarness: Universal Rust-based Autonomous Research Framework},
  author = {AutoCodeHarness Team},
  year = {2026},
  url = {https://github.com/your-org/autocodeharness}
}
```

---

**"Research is now entirely the domain of autonomous swarms of AI agents running across compute cluster megastructures in the skies."** - @karpathy, March 2026
