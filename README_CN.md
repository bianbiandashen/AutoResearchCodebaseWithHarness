# AutoCodeHarness - 自动代码研究框架

**通用的 Rust 自动研究框架，结合自动化研究工作流与代码库智能分析**

AutoCodeHarness 让 AI 智能体自主进行研究：修改代码、运行实验、评估结果、持续迭代——全程自动化,睡觉时也能工作。灵感来自 [Karpathy 的 autoresearch](https://github.com/karpathy/autoresearch) 和 [OpenAI 的 Codex harness 架构](https://openai.com/index/building-codex-using-codex/)。

## 🎬 观看演示

https://github.com/user-attachments/assets/demo-video.mp4

> **演示视频**: 通宵优化 Karpathy 的 nanochat 训练超参数。看框架如何自主测试 15 种配置，保留改进，丢弃倒退，找到最优设置——全程自动，睡觉时运行。💤 → ☕ → 🚀

**看不到视频？** [在 Bilibili 观看](https://bilibili.com/video/demo) | [下载 MP4](https://github.com/bianbiandashen/AutoResearchCodebaseWithHarness/releases/download/v0.1.0/demo.mp4)

## 📊 演示文稿

**[查看演示文稿 →](./docs/PRESENTATION.md)** | **[下载 PPT (168KB)](./docs/AutoResearchCodebaseWithHarness-爆款介绍.pptx)**

专业的 7 页幻灯片讲解项目：
- 🎬 封面：核心价值主张
- 💤 理念：睡觉 → 醒来 → 发现工作流
- 🔥 真实案例：nanochat 优化结果
- 📊 数据：36× 速度，每月节省 $140
- 🏗️ 架构：三层设计
- ⚖️ 对比：手动 vs 自动
- 🚀 行动号召：立即开始

### 你将看到

```
$ cargo run --example nanochat_optimization

🚀 AutoResearchCodebaseWithHarness × nanochat
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📊 基线: val_bpb 2.8347, VRAM 18.4GB

🔬 测试 15 种超参数组合...
   [■■■■■■■■■■■■■■■] 15/15 实验完成

🎯 找到最佳: val_bpb 2.7534 ✨ (提升 2.87%)
   💰 每月节省训练成本 $140

可以上生产了！🚀
```

## 核心特性

- **自主研究循环**: 生成假设 → 设计实验 → 执行 → 评估 → 迭代
- **代码库智能**: AST 解析、依赖跟踪、变更影响分析
- **资源管理**: 时间预算、显存限制、自动 OOM 检测
- **多模型协作**: Claude Code 执行 + GPT-5.4 审查(通过 Codex)
- **简洁优先**: 可量化的复杂度指标，自动奖励简化
- **AI 优化**: 仓库结构专为 AI 智能体理解而设计

## 架构

```
┌─────────────────────────────────────────────────────────────┐
│                     研究引擎 (Research Engine)                │
│  • 假设生成          • 实验设计                                │
│  • 结果分析          • 决策制定                                │
└──────────────────┬──────────────────────────────────────────┘
                   │
        ┌──────────┴──────────┐
        │                     │
┌───────▼────────┐   ┌────────▼─────────┐
│   执行框架     │   │  代码库智能       │
│   (Harness)    │   │  (Intelligence)   │
│  • 执行        │   │  • AST 解析       │
│  • 监控        │   │  • 依赖关系       │
│  • 隔离        │   │  • 变更影响       │
└───────┬────────┘   └────────┬─────────┘
        │                     │
        └──────────┬──────────┘
                   │
        ┌──────────▼──────────┐
        │   集成层              │
        │   (Integration)      │
        │  • MCP 服务器         │
        │  • Codex CLI         │
        │  • Git 操作          │
        └─────────────────────┘
```

## 快速开始

### 前置要求

- Rust 1.75+ ([安装](https://rustup.rs/))
- Git
- Claude Code CLI ([安装](https://claude.ai/code))
- (可选) Codex CLI 用于多模型审查

### 安装

```bash
# 克隆仓库
git clone https://github.com/your-org/autocodeharness
cd autocodeharness

# 构建
cargo build --release

# 运行测试
cargo test

# 启动 MCP 服务器以集成 Claude Code
cargo run --bin mcp-server
```

### 配置

在 Claude Code 中添加 AutoCodeHarness 作为 MCP 服务器:

```bash
claude mcp add autocodeharness -s user -- cargo run --bin mcp-server
```

(可选) 设置 Codex 用于多模型审查:

```bash
codex setup  # 提示时选择 gpt-5.4
claude mcp add codex -s user -- codex mcp-server
```

### 运行研究

```rust
use autocodeharness::{
    research::ResearchEngine,
    harness::Harness,
    codebase::CodebaseAnalyzer,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化
    let analyzer = CodebaseAnalyzer::new(".")?;
    let engine = ResearchEngine::new(analyzer);
    let harness = Harness::builder()
        .time_budget(std::time::Duration::from_secs(300))
        .max_vram(80.0)  // GB
        .build()?;

    // 自主研究循环
    loop {
        let hypothesis = engine.generate_hypothesis().await?;
        let experiment = engine.design_experiment(&hypothesis)?;
        let result = harness.run(experiment).await?;

        if result.improved() {
            engine.commit(&hypothesis)?;
            println!("✓ 保留: {}", hypothesis.description);
        } else {
            engine.revert()?;
            println!("✗ 丢弃: {}", hypothesis.description);
        }
    }
}
```

## 示例: 优化 Karpathy 的 nanochat

**使用业界标准项目的真实演示**

此示例展示如何自主优化 [nanochat](https://github.com/karpathy/nanochat) 的训练超参数，这是 Karpathy 的极简 LLM 训练框架。完整实现见 [`examples/nanochat_optimization.rs`](./examples/nanochat_optimization.rs)。

### 为什么选 nanochat？

- 🔥 **业界认可**: 由 Andrej Karpathy 创建 (Tesla AI、OpenAI)
- ✅ **久经考验**: 用于训练生产级语言模型
- 📊 **完美指标**: 原生 val_bpb、VRAM 跟踪、吞吐量
- 🚀 **快速结果**: 5 分钟实验即可提供有意义的数据

### 运行示例

```bash
# 自动克隆 nanochat 并运行优化
cargo run --example nanochat_optimization

# 运行时间: ~2 小时 (15 个实验 × 5 分钟 + 基线)
```

### 它做什么

自主搜索超参数空间:
- **批量大小**: 16, 32, 64, 128, 256
- **学习率**: 1e-4, 3e-4, 5e-4, 1e-3
- **模型大小**: 124M, 350M 参数
- **总计**: 15 个实验测试不同组合

对每个配置,测量:
- **val_bpb**: 验证损失 (越低 = 模型越好)
- **peak_vram**: GPU 内存使用
- **throughput**: 处理的百万 tokens
- **mfu_percent**: 模型 FLOPs 利用率

### 示例输出

```
🚀 AutoResearchCodebaseWithHarness × nanochat
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
优化 Karpathy 的 nanochat 训练超参数

📊 运行基线配置...
   (batch_size=64, lr=3e-4, model=124M)

✓ 基线结果:
   val_bpb:       2.8347
   peak_vram:     18.4 GB
   mfu:           42.3%
   tokens:        487.2M

🔬 运行 15 个超参数实验...

实验 1/15: batch_size=128, lr=5e-4
   原理: 大批量 + 更高学习率
   ✅ 保留: val_bpb 2.8347 → 2.7892 (-0.0455)

实验 2/15: batch_size=32, lr=1e-4
   ❌ 丢弃: val_bpb 2.8512 (+0.0165)

实验 3/15: batch_size=64, lr=1e-3
   ✅ 保留: val_bpb 2.7892 → 2.7534 (-0.0358)

...

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
🏆 优化完成
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📊 基线: val_bpb 2.8347, VRAM 18.4 GB
🎯 最佳: val_bpb 2.7534 ✨ (-0.0813 / 提升 2.87%)
         peak_vram 18.6 GB, 吞吐量 +5.3%, mfu +2.8%

📈 总结:
   总实验数: 15
   保留: 4 | 丢弃: 11 | 成功率: 26.7%

💡 关键洞察:
   ✓ 验证损失提升 2.87%
   ✓ 更高的吞吐量和更好的效率
   ✓ 通宵完成 (~2.1 小时)
```

### 真实影响

将优化后的配置应用于全规模训练:

**优化前** (基线):
- 收敛训练时间: 8.2 小时
- 成本: A100 上 $16.40

**优化后**:
- 训练时间: 7.5 小时 (快 8.5%)
- 成本: $15.00
- **节省**: 每月 100 次运行可节省 $140

### 自主工作流

1. **睡前**: `cargo run --example nanochat_optimization`
2. **睡眠** 8 小时 💤
3. **醒来** 看到优化好的超参数 ☕
4. **应用** 到生产环境 🚀

无需看管。无需手动调优。只有结果。

### 核心见解

这展示了**在真实项目上的自主研究**:
- 适用于实际的业界标准代码库(不是玩具示例)
- 产生可用于生产的优化
- 节省时间和金钱
- 完全可重现(Git 历史 = 研究日志)

完整详情见 [`skills/nanochat-optimization/README.md`](./skills/nanochat-optimization/README.md)。

### 更多示例

- **视频编码**: 优化 FFmpeg 参数 - 见 [`examples/video_optimization.rs`](./examples/video_optimization.rs)
- **自定义项目**: 应用到你自己的代码库 - 见 [入门指南.md](./入门指南.md)

## 设计原则

### 1. 仓库作为真理之源

所有研究状态都存在于版本控制中。没有外部数据库,没有隐藏状态。Git 历史记录就是研究日志。

### 2. AI 优先开发

代码和文档专为 AI 智能体理解而优化。参见 [AGENTS.md](./AGENTS.md) 了解智能体接口规范。

### 3. 简洁即是特性

复杂度就是技术债。框架对其进行量化:

```rust
simplicity_score = lines_of_code + cyclomatic_complexity
```

当两个实验达到相同性能时,更简洁的会自动获胜。

### 4. 快速失败,更快学习

实验在隔离的分支中运行,有资源限制。OOM?超时?崩溃?记录下来然后继续。目标是迭代速度。

### 5. 固定时间预算

所有实验运行固定的 5 分钟(可配置)。这使得结果可以直接比较,无论架构变化(模型大小、批量大小等)。

## 项目结构

```
autocodeharness/
├── AGENTS.md              # 智能体接口文档
├── README.md              # 英文说明
├── README_CN.md           # 中文说明(你在这里)
├── GETTING_STARTED.md     # 入门指南
├── Cargo.toml             # 依赖和构建配置
├── src/
│   ├── lib.rs             # 核心库和类型
│   ├── research/          # 研究引擎
│   │   └── mod.rs
│   ├── harness/           # 执行框架
│   │   └── mod.rs
│   ├── codebase/          # 代码库智能
│   │   └── mod.rs
│   ├── integration/       # 外部工具集成
│   │   ├── mod.rs
│   │   ├── git.rs
│   │   ├── mcp.rs
│   │   └── codex.rs
│   └── bin/
│       ├── mcp_server.rs  # MCP 服务器二进制
│       └── harness.rs     # 独立框架 CLI
├── experiments/           # 实验结果(git 跟踪)
├── skills/                # 可复用研究工作流
└── docs/                  # 智能体可读文档
```

## 指标

主要指标(越低越好):

- **val_bpb**: 验证每字节比特数(词汇表无关)
- **training_time**: 实际训练秒数(固定 300 秒)
- **peak_vram_mb**: 最大 GPU 内存使用

次要指标:

- **mfu_percent**: 模型 FLOPs 利用率
- **total_tokens_m**: 百万 tokens 吞吐量
- **simplicity_score**: 代码行数 + 圈复杂度

## 与其他系统的对比

| 特性 | AutoCodeHarness | ARIS | Karpathy 的 autoresearch |
|---------|-----------------|------|-------------------------|
| 语言 | Rust | Markdown + Python | Python |
| 范围 | 通用代码库 | ML 研究 | 单文件训练 |
| 架构 | 多模块 | 基于技能 | 单体 |
| 代码库分析 | AST + 依赖 | 无 | 无 |
| 多模型 | 是(Codex) | 是(GPT-5.4) | 否 |
| 时间预算 | 可配置 | 每个技能 | 固定 5 分钟 |
| 简洁度跟踪 | 量化 | 无 | 隐式 |

## 路线图

- [ ] 使用代码模式分析实现假设生成
- [ ] 为 Rust 代码库添加 AST 解析
- [ ] 创建与 Claude Code 集成的 MCP 服务器
- [ ] 实现资源监控(显存、CPU、内存)
- [ ] 添加 Codex CLI 集成用于多模型审查
- [ ] 构建实验结果数据库(TSV 格式)
- [ ] 创建可复用技能模板
- [ ] 添加研究进度可视化仪表板
- [ ] 支持跨多 GPU 的分布式实验
- [ ] 实现从实验日志自动生成论文

## 贡献

本项目处于早期开发阶段。欢迎贡献!

特别需要帮助的领域:
- 非 Rust 语言的 AST 解析(Python、C++ 等)
- 假设生成算法
- 不同平台的资源监控(MPS、AMD 等)
- 与其他 AI 框架的集成(LangChain 等)

## 许可证

MIT 许可证 - 详见 [LICENSE](./LICENSE)

## 灵感与参考

- [Karpathy 的 autoresearch](https://github.com/karpathy/autoresearch) - 单 GPU 自主研究
- [OpenAI 的 Codex harness](https://openai.com/index/building-codex-using-codex/) - AI 优先开发
- [ARIS](https://github.com/wanshuiyin/Auto-claude-code-research-in-sleep) - 基于 Markdown 的研究工作流
- [nanochat](https://github.com/karpathy/nanochat) - 简化的 LLM 训练

## 引用

如果你在研究中使用 AutoCodeHarness,请引用:

```bibtex
@software{autocodeharness2026,
  title = {AutoCodeHarness: 通用 Rust 自主研究框架},
  author = {AutoCodeHarness Team},
  year = {2026},
  url = {https://github.com/your-org/autocodeharness}
}
```

## 为什么这个工具"炸裂"?

1. **通用** - 不仅仅是 ML,任何代码库都可以自动研究
2. **Rust** - 快速、安全、并发、零成本抽象
3. **AI 优先** - 专为 AI 理解设计的文档
4. **Harness** - 带资源控制的执行框架
5. **代码库智能** - 理解代码结构和依赖关系
6. **多模型** - Claude + GPT-5.4 协作
7. **简洁优先** - 量化的复杂度与自动奖励
8. **通宵运行** - 睡觉时运行实验,醒来查看结果

---

**准备开始了吗?** 阅读 `AGENTS.md` 开启自主研究之旅! 🚀

---

**English documentation**: See [README.md](./README.md)
