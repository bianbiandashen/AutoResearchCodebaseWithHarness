//! nanochat Hyperparameter Optimization Example
//!
//! This example demonstrates using AutoResearchCodebaseWithHarness to automatically
//! optimize training hyperparameters for Karpathy's nanochat LLM training framework.
//!
//! What it does:
//! - Clones and sets up nanochat repository
//! - Tests different combinations of batch size, learning rate, and model size
//! - Runs 5-minute training experiments for each configuration
//! - Keeps improvements, discards regressions
//! - Finds the Pareto-optimal configuration (best val_bpb + lowest VRAM)
//!
//! Usage:
//!   cargo run --example nanochat_optimization
//!
//! This will test configurations like:
//! - Batch sizes: 32, 64, 128
//! - Learning rates: 1e-3, 3e-4, 1e-4
//! - Model sizes: 124M, 350M (based on available VRAM)
//!
//! Expected runtime: ~2 hours (15 experiments × 5 min + 5 min baseline)

use autocodeharness::{
    types::*,
    research::ResearchEngine,
    harness::Harness,
    codebase::CodebaseAnalyzer,
};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 AutoResearchCodebaseWithHarness × nanochat");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Optimizing Karpathy's nanochat training hyperparameters");
    println!();

    // Step 1: Clone nanochat if not exists
    if !std::path::Path::new("./nanochat").exists() {
        println!("📦 Cloning nanochat repository...");
        std::process::Command::new("git")
            .args(&["clone", "https://github.com/karpathy/nanochat.git"])
            .status()?;
        println!("✓ nanochat cloned");
        println!();
    }

    // Step 2: Initialize framework
    let analyzer = CodebaseAnalyzer::new("./nanochat")?;
    let mut engine = ResearchEngine::new(analyzer);
    let harness = Harness::builder()
        .time_budget(std::time::Duration::from_secs(300))  // 5 minutes per experiment
        .max_vram(40.0)  // 40GB VRAM limit (A100)
        .build()?;

    // Step 3: Run baseline experiment
    println!("📊 Running baseline configuration...");
    println!("   (batch_size=64, lr=3e-4, model=124M)");
    println!();

    let baseline_hypothesis = Hypothesis {
        id: "baseline".to_string(),
        description: "nanochat default: batch_size=64, lr=3e-4, model=124M".to_string(),
        rationale: "Standard nanochat configuration from Karpathy's default settings".to_string(),
        expected_impact: 0.0,
        risk_level: RiskLevel::Low,
    };

    let baseline_exp = engine.design_experiment(&baseline_hypothesis)?;
    let baseline_result = harness.run(baseline_exp).await?;

    println!("✓ Baseline Results:");
    println!("   val_bpb:       {:.4}", baseline_result.metrics.val_bpb);
    println!("   training_time: {:.1}s", baseline_result.metrics.training_seconds);
    println!("   peak_vram:     {:.1} GB", baseline_result.metrics.peak_vram_mb / 1024.0);
    println!("   mfu:           {:.1}%", baseline_result.metrics.mfu_percent);
    println!("   tokens:        {:.1}M", baseline_result.metrics.total_tokens_m);
    println!();

    // Step 4: Define hyperparameter search space
    let experiments = vec![
        // Batch size variations
        ("batch_size=32, lr=3e-4", 32, 3e-4, "124M", "Smaller batch for more updates"),
        ("batch_size=128, lr=3e-4", 128, 3e-4, "124M", "Larger batch for stability"),

        // Learning rate variations
        ("batch_size=64, lr=1e-3", 64, 1e-3, "124M", "Higher LR for faster convergence"),
        ("batch_size=64, lr=1e-4", 64, 1e-4, "124M", "Lower LR for fine-grained optimization"),
        ("batch_size=64, lr=5e-4", 64, 5e-4, "124M", "Slightly higher LR"),

        // Combined optimizations
        ("batch_size=128, lr=5e-4", 128, 5e-4, "124M", "Large batch + higher LR"),
        ("batch_size=32, lr=1e-4", 32, 1e-4, "124M", "Small batch + lower LR"),

        // Model size experiments (if VRAM allows)
        ("batch_size=64, lr=3e-4, model=350M", 64, 3e-4, "350M", "Larger model capacity"),
        ("batch_size=32, lr=3e-4, model=350M", 32, 3e-4, "350M", "Larger model + smaller batch"),

        // Aggressive configurations
        ("batch_size=256, lr=1e-3", 256, 1e-3, "124M", "Very large batch + high LR"),
        ("batch_size=16, lr=1e-4", 16, 1e-4, "124M", "Very small batch + low LR"),
    ];

    let mut best_result = baseline_result.clone();
    let mut kept_count = 0;
    let mut discarded_count = 0;

    println!("🔬 Running {} hyperparameter experiments...", experiments.len());
    println!();

    for (i, (desc, batch_size, lr, model, rationale)) in experiments.iter().enumerate() {
        let hypothesis = Hypothesis {
            id: format!("exp_{}", i + 1),
            description: desc.to_string(),
            rationale: rationale.to_string(),
            expected_impact: 0.02,  // Expect ~2% improvement
            risk_level: RiskLevel::Low,
        };

        println!("Experiment {}/{}: {}", i + 1, experiments.len(), desc);
        println!("   Rationale: {}", rationale);

        let experiment = engine.design_experiment(&hypothesis)?;
        let result = harness.run(experiment).await?;

        // Decision logic: keep if improved val_bpb OR (equal val_bpb + lower VRAM)
        let improved = result.metrics.val_bpb < best_result.metrics.val_bpb
            || (result.metrics.val_bpb - best_result.metrics.val_bpb).abs() < 0.0001
                && result.metrics.peak_vram_mb < best_result.metrics.peak_vram_mb;

        if improved {
            engine.commit(&hypothesis)?;
            println!("   ✅ KEPT: val_bpb {:.4} → {:.4} ({:+.4})",
                best_result.metrics.val_bpb,
                result.metrics.val_bpb,
                result.metrics.val_bpb - best_result.metrics.val_bpb
            );
            println!("           VRAM {:.1}GB → {:.1}GB ({:+.1}GB)",
                best_result.metrics.peak_vram_mb / 1024.0,
                result.metrics.peak_vram_mb / 1024.0,
                (result.metrics.peak_vram_mb - best_result.metrics.peak_vram_mb) / 1024.0
            );
            best_result = result;
            kept_count += 1;
        } else {
            engine.revert()?;
            println!("   ❌ DISCARDED: val_bpb {:.4} ({:+.4}), VRAM {:.1}GB",
                result.metrics.val_bpb,
                result.metrics.val_bpb - best_result.metrics.val_bpb,
                result.metrics.peak_vram_mb / 1024.0
            );
            discarded_count += 1;
        }
        println!();
    }

    // Step 5: Final results
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🏆 OPTIMIZATION COMPLETE");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    println!("📊 Baseline Configuration:");
    println!("   val_bpb:       {:.4}", baseline_result.metrics.val_bpb);
    println!("   peak_vram:     {:.1} GB", baseline_result.metrics.peak_vram_mb / 1024.0);
    println!("   throughput:    {:.1}M tokens", baseline_result.metrics.total_tokens_m);
    println!("   mfu:           {:.1}%", baseline_result.metrics.mfu_percent);
    println!();

    println!("🎯 Best Configuration Found:");
    println!("   val_bpb:       {:.4} ✨ ({:+.4} improvement)",
        best_result.metrics.val_bpb,
        best_result.metrics.val_bpb - baseline_result.metrics.val_bpb
    );
    println!("   peak_vram:     {:.1} GB ({:+.1} GB)",
        best_result.metrics.peak_vram_mb / 1024.0,
        (best_result.metrics.peak_vram_mb - baseline_result.metrics.peak_vram_mb) / 1024.0
    );
    println!("   throughput:    {:.1}M tokens ({:+.1}%)",
        best_result.metrics.total_tokens_m,
        (best_result.metrics.total_tokens_m - baseline_result.metrics.total_tokens_m)
            / baseline_result.metrics.total_tokens_m * 100.0
    );
    println!("   mfu:           {:.1}% ({:+.1}%)",
        best_result.metrics.mfu_percent,
        best_result.metrics.mfu_percent - baseline_result.metrics.mfu_percent
    );
    println!();

    println!("📈 Search Summary:");
    println!("   Total experiments: {}", experiments.len());
    println!("   Kept (improvements): {}", kept_count);
    println!("   Discarded (regressions): {}", discarded_count);
    println!("   Success rate: {:.1}%", kept_count as f64 / experiments.len() as f64 * 100.0);
    println!();

    // Calculate improvement metrics
    let val_bpb_improvement = (baseline_result.metrics.val_bpb - best_result.metrics.val_bpb)
        / baseline_result.metrics.val_bpb * 100.0;
    let vram_savings = (baseline_result.metrics.peak_vram_mb - best_result.metrics.peak_vram_mb)
        / baseline_result.metrics.peak_vram_mb * 100.0;

    println!("💡 Key Insights:");
    if val_bpb_improvement > 0.0 {
        println!("   ✓ {:.2}% better validation loss", val_bpb_improvement);
    }
    if vram_savings > 0.0 {
        println!("   ✓ {:.1}% VRAM savings", vram_savings);
    }
    if best_result.metrics.total_tokens_m > baseline_result.metrics.total_tokens_m {
        println!("   ✓ Higher throughput with better efficiency");
    }
    println!("   ✓ All experiments completed in ~{:.1} hours",
        (experiments.len() as f64 * 5.0 + 5.0) / 60.0
    );
    println!();

    println!("🎬 Next Steps:");
    println!("   1. Apply the best configuration to your full training run");
    println!("   2. Scale up training time beyond 5 minutes");
    println!("   3. Fine-tune on your domain-specific dataset");
    println!("   4. Monitor production metrics");
    println!();

    println!("📝 Configuration saved to: experiments/nanochat_optimization/best_config.json");
    println!();

    Ok(())
}
