//! # AutoCodeHarness
//!
//! Universal Rust-based autonomous research framework that combines automated research
//! workflows with codebase intelligence.
//!
//! ## Architecture
//!
//! The framework is organized into four main modules:
//!
//! - [`research`]: Research workflow engine for hypothesis generation and experiment design
//! - [`harness`]: Execution harness with resource management and monitoring
//! - [`codebase`]: Codebase analysis and intelligence layer
//! - [`integration`]: Integration with external tools (MCP, Codex, Git)
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use autocodeharness::{research::ResearchEngine, harness::Harness, codebase::CodebaseAnalyzer};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // Initialize components
//!     let analyzer = CodebaseAnalyzer::new(".")?;
//!     let engine = ResearchEngine::new(analyzer);
//!     let harness = Harness::builder()
//!         .time_budget(std::time::Duration::from_secs(300))
//!         .build()?;
//!
//!     // Run autonomous research loop
//!     loop {
//!         let hypothesis = engine.generate_hypothesis().await?;
//!         let experiment = engine.design_experiment(&hypothesis)?;
//!         let result = harness.run(experiment).await?;
//!
//!         if result.improved() {
//!             engine.commit(&hypothesis)?;
//!         } else {
//!             engine.revert()?;
//!         }
//!     }
//! }
//! ```

pub mod research;
pub mod harness;
pub mod codebase;
pub mod integration;
pub mod tasks;

pub use anyhow::{Result, Error};

/// Core types used across the framework
pub mod types {
    use serde::{Deserialize, Serialize};
    use std::time::Duration;

    /// Unique identifier for an experiment
    #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct ExperimentId(pub String);

    /// Research hypothesis with justification
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Hypothesis {
        pub id: String,
        pub description: String,
        pub rationale: String,
        pub expected_impact: f64,
        pub risk_level: RiskLevel,
    }

    /// Risk classification for experiments
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum RiskLevel {
        Low,      // No expected failures, safe to run
        Medium,   // Might OOM or timeout
        High,     // Could corrupt state, use isolation
    }

    /// Experiment configuration
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Experiment {
        pub id: ExperimentId,
        pub hypothesis: Hypothesis,
        pub code_changes: Vec<CodeChange>,
        pub time_budget: Duration,
        pub resource_limits: ResourceLimits,
    }

    /// Code modification for an experiment
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CodeChange {
        pub file_path: String,
        pub original_content: String,
        pub modified_content: String,
        pub change_type: ChangeType,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum ChangeType {
        Architecture,  // Model structure changes
        Hyperparameter, // Config/param changes
        Optimization,  // Optimizer/learning rate changes
        Other,
    }

    /// Resource constraints for experiments
    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub struct ResourceLimits {
        pub max_vram_gb: Option<f64>,
        pub max_time_seconds: u64,
        pub max_memory_gb: Option<f64>,
    }

    /// Experiment results
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ExperimentResult {
        pub experiment_id: ExperimentId,
        pub status: ExperimentStatus,
        pub metrics: Metrics,
        pub duration: Duration,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum ExperimentStatus {
        Success,
        Crashed,
        Timeout,
        OOM,
    }

    /// Performance metrics
    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub struct Metrics {
        pub val_bpb: f64,
        pub training_seconds: f64,
        pub peak_vram_mb: f64,
        pub mfu_percent: f64,
        pub total_tokens_m: f64,
        pub simplicity_score: Option<u32>,
    }

    impl Metrics {
        /// Compare two metrics: returns true if self is better than other
        pub fn improved_over(&self, other: &Metrics) -> bool {
            // Primary: val_bpb lower is better
            if self.val_bpb < other.val_bpb {
                return true;
            }
            // Secondary: if equal perf, lower complexity is better
            if (self.val_bpb - other.val_bpb).abs() < 0.0001 {
                if let (Some(self_score), Some(other_score)) =
                    (self.simplicity_score, other.simplicity_score) {
                    return self_score < other_score;
                }
            }
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::types::*;

    #[test]
    fn test_metrics_comparison() {
        let baseline = Metrics {
            val_bpb: 1.0,
            training_seconds: 300.0,
            peak_vram_mb: 40000.0,
            mfu_percent: 35.0,
            total_tokens_m: 500.0,
            simplicity_score: Some(100),
        };

        let improved = Metrics {
            val_bpb: 0.95,
            ..baseline
        };

        assert!(improved.improved_over(&baseline));
        assert!(!baseline.improved_over(&improved));
    }

    #[test]
    fn test_simplicity_tiebreaker() {
        let complex = Metrics {
            val_bpb: 1.0,
            training_seconds: 300.0,
            peak_vram_mb: 40000.0,
            mfu_percent: 35.0,
            total_tokens_m: 500.0,
            simplicity_score: Some(200),
        };

        let simple = Metrics {
            val_bpb: 1.0,
            simplicity_score: Some(150),
            ..complex
        };

        assert!(simple.improved_over(&complex));
    }
}
