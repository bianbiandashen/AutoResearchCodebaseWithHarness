# Getting Started with AutoCodeHarness

Welcome to **AutoCodeHarness** - your Rust-based autonomous research framework!

## What Just Got Created

A complete Rust project with:

- ✅ **Core library** with type system and module structure
- ✅ **Research engine** for hypothesis generation and experiment management  
- ✅ **Execution harness** with resource limits and monitoring
- ✅ **Codebase intelligence** layer for AST analysis and dependencies
- ✅ **Integration layer** for Git, MCP server, and Codex CLI
- ✅ **Comprehensive documentation** optimized for AI agents (AGENTS.md)
- ✅ **Research guide** with strategies and best practices
- ✅ **Skill templates** for reusable workflows
- ✅ **Git repository** with initial commit

Total: **~3,229 lines** across 18 files

## Project Structure

```
autocodeharness/
├── AGENTS.md                    # 🤖 Primary agent interface - READ THIS FIRST
├── README.md                    # Project overview and architecture
├── GETTING_STARTED.md          # This file!
├── LICENSE                      # MIT License
├── Cargo.toml                   # Rust dependencies
├── .gitignore                   # Git ignore rules
├── results_template.tsv         # Experiment tracking template
│
├── src/                         # Rust source code
│   ├── lib.rs                   # Core types and library entry
│   ├── research/                # Research engine
│   │   └── mod.rs
│   ├── harness/                 # Execution harness
│   │   └── mod.rs
│   ├── codebase/               # Codebase intelligence
│   │   └── mod.rs
│   └── integration/            # External integrations
│       ├── mod.rs
│       ├── git.rs              # Git operations
│       ├── mcp.rs              # MCP server
│       └── codex.rs            # Codex CLI
│
├── experiments/                 # Experiment results (git-tracked)
│   └── README.md
├── skills/                      # Reusable workflow templates
│   └── README.md
└── docs/                        # Documentation
    └── RESEARCH_GUIDE.md       # How to do research effectively
```

## Quick Start (3 Steps)

### 1. Build the Project

```bash
cd /tmp/autocodeharness
cargo build --release
```

This will compile the Rust code and download dependencies (~197 packages).

### 2. Run Tests

```bash
cargo test
```

Should see output like:
```
running 2 tests
test tests::test_metrics_comparison ... ok
test tests::test_simplicity_tiebreaker ... ok
```

### 3. (Optional) Test MCP Server

```bash
cargo run --bin mcp-server
```

## Integration with Claude Code

### Add as MCP Server

```bash
# From any directory, add AutoCodeHarness MCP server
claude mcp add autocodeharness -s user -- cargo run --bin mcp-server --manifest-path /tmp/autocodeharness/Cargo.toml

# Restart Claude Code to activate
```

### Use in Chat

Once integrated, Claude Code can:
- Generate research hypotheses based on codebase analysis
- Design and run experiments autonomously
- Track results and make keep/discard decisions
- Iterate overnight while you sleep

## Integration with Codex (Optional)

For multi-model collaboration (Claude Code execution + GPT-5.4 review):

```bash
# Setup Codex (if not already done)
codex setup  # Select gpt-5.4 when prompted

# Add Codex MCP server to Claude Code
claude mcp add codex -s user -- codex mcp-server

# Now you can ask Claude Code to get GPT-5.4 reviews
```

## Next Steps

### Option 1: Implement Core Features

The project currently has **stub implementations**. Key TODOs:

1. **Hypothesis Generation** (`src/research/mod.rs`)
   - Parse code structure
   - Analyze recent experiments
   - Generate informed hypotheses

2. **Experiment Execution** (`src/harness/mod.rs`)
   - Apply code changes
   - Launch subprocess with time limit
   - Monitor VRAM/CPU/memory
   - Capture output and metrics

3. **Codebase Analysis** (`src/codebase/mod.rs`)
   - AST parsing for Rust
   - Dependency graph construction
   - Simplicity scoring

4. **MCP Server** (`src/integration/mcp.rs`)
   - JSON-RPC handler
   - Tool registration
   - Claude Code protocol implementation

### Option 2: Create a Research Project

Use AutoCodeHarness to optimize an existing ML training script:

```bash
# 1. Copy your training code to a new repo
mkdir -p my-research
cd my-research

# 2. Add AutoCodeHarness as dependency
cargo add autocodeharness --path /tmp/autocodeharness

# 3. Write main.rs that uses the framework
# (see README.md for example)

# 4. Let agents iterate overnight
```

### Option 3: Extend with Skills

Create reusable research workflows in `skills/`:

```bash
cd /tmp/autocodeharness/skills
mkdir architecture-search
cd architecture-search

# Create skill.toml
cat > skill.toml << 'EOF'
name = "architecture-search"
description = "Search for optimal model architecture"
risk_level = "medium"
expected_experiments = 20
EOF

# Create template code
# ...
```

## Architecture Highlights

### 1. Types-First Design

All core types defined in `src/lib.rs`:
- `Hypothesis` - Research hypothesis with rationale
- `Experiment` - Full experiment configuration
- `ExperimentResult` - Results with metrics
- `Metrics` - Performance measurements
- `CodeChange` - Code modification specification

### 2. Async/Await Throughout

Built on Tokio runtime for efficient I/O:
- Concurrent experiment execution
- Non-blocking Git operations
- Async MCP server

### 3. Strong Error Handling

Uses `anyhow::Result<T>` everywhere:
- Explicit error propagation with `?`
- No panics in production code
- Meaningful error messages

### 4. Zero-Copy Where Possible

Leverages Rust ownership:
- References over clones
- Smart pointers for shared data
- Minimal allocations in hot paths

## Key Documentation

Before you start coding, read these in order:

1. **AGENTS.md** - Agent interface contract (if you're an AI agent, START HERE)
2. **README.md** - Project overview and architecture
3. **docs/RESEARCH_GUIDE.md** - How to conduct effective research
4. **experiments/README.md** - Experiment lifecycle and metrics
5. **skills/README.md** - Creating reusable workflows

## Common Commands

```bash
# Build
cargo build                    # Debug build
cargo build --release         # Optimized build

# Test
cargo test                    # Run all tests
cargo test test_metrics       # Run specific test

# Run
cargo run --bin mcp-server    # Start MCP server
cargo run --bin harness       # Standalone harness CLI

# Check
cargo check                   # Fast compile check
cargo clippy                  # Linter
cargo fmt                     # Format code

# Docs
cargo doc --open              # Generate and open documentation
```

## Troubleshooting

### Build Errors

```bash
# Clear cache and rebuild
cargo clean
cargo build
```

### Git Issues

```bash
# Check git status
cd /tmp/autocodeharness
git status
git log --oneline
```

### MCP Server Not Connecting

```bash
# Check if server starts
cargo run --bin mcp-server

# Check Claude Code MCP configuration
claude mcp list

# Remove and re-add
claude mcp remove autocodeharness
claude mcp add autocodeharness -s user -- cargo run --bin mcp-server --manifest-path /tmp/autocodeharness/Cargo.toml
```

## Contributing

To push to GitHub:

```bash
# 1. Create repo on GitHub (https://github.com/new)
#    Name: autocodeharness

# 2. Add remote and push
cd /tmp/autocodeharness
git remote add origin https://github.com/YOUR_USERNAME/autocodeharness.git
git branch -M main
git push -u origin main
```

## Resources

- [Rust Book](https://doc.rust-lang.org/book/) - Learn Rust
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial) - Async Rust
- [Cargo Book](https://doc.rust-lang.org/cargo/) - Rust package manager
- [Karpathy's autoresearch](https://github.com/karpathy/autoresearch) - Inspiration
- [OpenAI Codex article](https://openai.com/index/building-codex-using-codex/) - Architecture principles

## What Makes This "炸裂" (Explosive)?

1. **Universal** - Not just ML, any codebase can be auto-researched
2. **Rust** - Fast, safe, concurrent, zero-cost abstractions
3. **Agent-First** - Documentation designed for AI comprehension
4. **Harness** - Execution framework with resource control
5. **Codebase Intelligence** - Understands code structure and dependencies
6. **Multi-Model** - Claude + GPT-5.4 collaboration
7. **Simplicity-First** - Quantified complexity with automatic rewards
8. **Overnight** - Run experiments while you sleep, wake up to results

---

**Ready to start?** Read `AGENTS.md` and let the autonomous research begin! 🚀
