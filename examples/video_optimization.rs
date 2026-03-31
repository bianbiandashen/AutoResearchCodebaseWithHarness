//! Video optimization example
//!
//! This example shows how to use AutoResearchCodebaseWithHarness to automatically
//! find the best video encoding parameters.
//!
//! Usage:
//!   cargo run --example video_optimization -- input.mp4
//!
//! This will run 15 experiments testing different combinations of:
//! - Codecs (H.264, H.265, AV1)
//! - CRF values (18-28)
//! - Presets (slow, medium, fast)
//!
//! The framework will automatically keep the best results and discard worse ones.

use autocodeharness::{
    types::*,
    research::ResearchEngine,
    harness::Harness,
    codebase::CodebaseAnalyzer,
};
use anyhow::Result;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // Get input video from command line
    let args: Vec<String> = env::args().collect();
    let input_video = args.get(1).cloned().unwrap_or_else(|| "input.mp4".to_string());

    println!("🎬 AutoResearchCodebaseWithHarness - Video Optimization");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Input: {}", input_video);
    println!();

    // Initialize components
    let analyzer = CodebaseAnalyzer::new(".")?;
    let mut engine = ResearchEngine::new(analyzer);
    let harness = Harness::builder()
        .time_budget(std::time::Duration::from_secs(300))
        .max_memory(8.0)  // 8GB RAM limit
        .build()?;

    // Define baseline configuration
    let baseline_hypothesis = Hypothesis {
        id: "baseline".to_string(),
        description: "H.264 CRF 23 medium preset (YouTube default)".to_string(),
        rationale: "Industry standard baseline for comparison".to_string(),
        expected_impact: 0.0,
        risk_level: RiskLevel::Low,
    };

    println!("📊 Running baseline...");
    let baseline_exp = engine.design_experiment(&baseline_hypothesis)?;
    let baseline_result = harness.run(baseline_exp).await?;

    println!("✓ Baseline VMAF: {:.2}, Size: {:.1}MB, Time: {:.1}s",
        baseline_result.metrics.val_bpb,
        baseline_result.metrics.peak_vram_mb / 1000.0,
        baseline_result.metrics.training_seconds
    );
    println!();

    // Configuration matrix to test
    let configs = vec![
        // H.264 variants
        ("H.264", 20, "slow"),
        ("H.264", 22, "medium"),
        ("H.264", 24, "fast"),
        ("H.264", 26, "medium"),

        // H.265 variants (better compression)
        ("H.265", 20, "slow"),
        ("H.265", 22, "medium"),
        ("H.265", 24, "fast"),
        ("H.265", 26, "medium"),

        // AV1 variants (best compression, slower)
        ("AV1", 24, "slow"),
        ("AV1", 26, "medium"),
        ("AV1", 28, "fast"),
    ];

    let mut best_result = baseline_result.clone();
    let mut kept_count = 0;
    let mut discarded_count = 0;

    println!("🚀 Running {} experiments...", configs.len());
    println!();

    for (i, (codec, crf, preset)) in configs.iter().enumerate() {
        let hypothesis = Hypothesis {
            id: format!("exp_{}", i + 1),
            description: format!("{} CRF {} {} preset", codec, crf, preset),
            rationale: format!("Testing {} codec for quality/size optimization", codec),
            expected_impact: 0.05,
            risk_level: RiskLevel::Low,
        };

        println!("Experiment {}/{}: {}", i + 1, configs.len(), hypothesis.description);

        let experiment = engine.design_experiment(&hypothesis)?;
        let result = harness.run(experiment).await?;

        // Check if this is better than current best
        if result.metrics.improved_over(&best_result.metrics) {
            engine.commit(&hypothesis)?;
            println!("  ✅ KEPT: VMAF {:.2} (+{:.2}), Size {:.1}MB ({:.1}% smaller)",
                result.metrics.val_bpb,
                result.metrics.val_bpb - best_result.metrics.val_bpb,
                result.metrics.peak_vram_mb / 1000.0,
                (best_result.metrics.peak_vram_mb - result.metrics.peak_vram_mb) / best_result.metrics.peak_vram_mb * 100.0
            );
            best_result = result;
            kept_count += 1;
        } else {
            engine.revert()?;
            println!("  ❌ DISCARDED: VMAF {:.2} ({:.2}), Size {:.1}MB",
                result.metrics.val_bpb,
                result.metrics.val_bpb - best_result.metrics.val_bpb,
                result.metrics.peak_vram_mb / 1000.0
            );
            discarded_count += 1;
        }
        println!();
    }

    // Print final results
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🏆 FINAL RESULTS");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
    println!("📊 Baseline (H.264 CRF 23 medium):");
    println!("   VMAF:     {:.2}", baseline_result.metrics.val_bpb);
    println!("   Size:     {:.1} MB", baseline_result.metrics.peak_vram_mb / 1000.0);
    println!("   Time:     {:.1}s", baseline_result.metrics.training_seconds);
    println!();
    println!("🎯 Best Found:");
    println!("   VMAF:     {:.2} ✨ ({:+.2} improvement)",
        best_result.metrics.val_bpb,
        best_result.metrics.val_bpb - baseline_result.metrics.val_bpb
    );
    println!("   Size:     {:.1} MB ({:.1}% {})",
        best_result.metrics.peak_vram_mb / 1000.0,
        ((best_result.metrics.peak_vram_mb - baseline_result.metrics.peak_vram_mb).abs() / baseline_result.metrics.peak_vram_mb * 100.0),
        if best_result.metrics.peak_vram_mb < baseline_result.metrics.peak_vram_mb { "smaller" } else { "larger" }
    );
    println!("   Time:     {:.1}s", best_result.metrics.training_seconds);
    println!();
    println!("📈 Summary:");
    println!("   Total experiments: {}", configs.len());
    println!("   Kept:              {}", kept_count);
    println!("   Discarded:         {}", discarded_count);
    println!();
    println!("💡 Recommendation:");
    println!("   Use the best configuration for your production encodes!");
    println!();

    Ok(())
}
