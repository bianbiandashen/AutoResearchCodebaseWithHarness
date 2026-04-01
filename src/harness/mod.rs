//! Execution harness for running experiments with resource constraints
//!
//! Provides sandboxed execution, resource monitoring, and timeout enforcement.

pub mod isolation;
pub mod cache;

use crate::types::*;
use anyhow::Result;
use std::time::Duration;

pub struct Harness {
    time_budget: Duration,
    resource_limits: ResourceLimits,
}

impl Harness {
    pub fn builder() -> HarnessBuilder {
        HarnessBuilder::default()
    }

    /// Run an experiment and return results
    pub async fn run(&self, experiment: Experiment) -> Result<ExperimentResult> {
        use std::time::Instant;

        println!("🚀 Running experiment: {}", experiment.hypothesis.description);

        let start = Instant::now();

        // Apply code changes (placeholder)
        for change in &experiment.code_changes {
            println!("  📝 Applying change to: {}", change.file_path);
        }

        // Simulate experiment execution
        // In production: launch actual subprocess, monitor resources
        tokio::time::sleep(Duration::from_secs(1)).await;

        let duration = start.elapsed();

        // Collect metrics (placeholder values)
        let metrics = Metrics {
            val_bpb: 0.95,
            training_seconds: duration.as_secs_f64(),
            peak_vram_mb: 35000.0,
            mfu_percent: 38.5,
            total_tokens_m: 450.0,
            simplicity_score: Some(120),
        };

        println!("  ✓ Completed in {:.2}s", duration.as_secs_f64());

        Ok(ExperimentResult {
            experiment_id: experiment.id,
            status: ExperimentStatus::Success,
            metrics,
            duration,
        })
    }

    /// Check if experiment improved over baseline
    pub fn improved(&self, result: &ExperimentResult, baseline: &Metrics) -> bool {
        result.metrics.improved_over(baseline)
    }
}

#[derive(Default)]
pub struct HarnessBuilder {
    time_budget: Option<Duration>,
    max_vram_gb: Option<f64>,
    max_memory_gb: Option<f64>,
}

impl HarnessBuilder {
    pub fn time_budget(mut self, duration: Duration) -> Self {
        self.time_budget = Some(duration);
        self
    }

    pub fn max_vram(mut self, gb: f64) -> Self {
        self.max_vram_gb = Some(gb);
        self
    }

    pub fn max_memory(mut self, gb: f64) -> Self {
        self.max_memory_gb = Some(gb);
        self
    }

    pub fn build(self) -> Result<Harness> {
        let time_budget = self.time_budget.unwrap_or(Duration::from_secs(300));
        Ok(Harness {
            time_budget,
            resource_limits: ResourceLimits {
                max_vram_gb: self.max_vram_gb,
                max_time_seconds: time_budget.as_secs(),
                max_memory_gb: self.max_memory_gb,
            },
        })
    }
}
