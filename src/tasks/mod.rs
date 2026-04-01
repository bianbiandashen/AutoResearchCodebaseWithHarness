// Task abstraction and registry system inspired by lm-evaluation-harness
//
// Key patterns:
// 1. Task trait with doc_to_* methods for data transformation
// 2. YAML-based configuration for task definitions
// 3. Central registry for task discovery and loading
// 4. Instance abstraction for individual evaluation items

pub mod registry;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Single evaluation instance
#[derive(Debug, Clone)]
pub struct Instance {
    /// Source document
    pub doc: serde_json::Value,
    /// Task name
    pub task_name: String,
    /// Instance index
    pub idx: usize,
    /// Generated request (prompt)
    pub request: Option<String>,
    /// Model response
    pub response: Option<String>,
    /// Evaluation result
    pub result: Option<EvaluationResult>,
}

impl Instance {
    pub fn new(doc: serde_json::Value, task_name: String, idx: usize) -> Self {
        Self {
            doc,
            task_name,
            idx,
            request: None,
            response: None,
            result: None,
        }
    }
}

/// Task evaluation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationResult {
    pub passed: bool,
    pub score: f64,
    pub details: HashMap<String, serde_json::Value>,
}

/// Task configuration (loaded from YAML)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskConfig {
    /// Task name (unique identifier)
    pub task_name: String,

    /// Dataset source
    #[serde(default)]
    pub dataset_path: Option<String>,

    #[serde(default)]
    pub dataset_name: Option<String>,

    /// Output type: "generate" or "select"
    #[serde(default = "default_output_type")]
    pub output_type: String,

    /// Number of few-shot examples
    #[serde(default)]
    pub num_fewshot: usize,

    /// Metrics to compute
    #[serde(default)]
    pub metric_list: Vec<String>,

    /// Execution timeout
    #[serde(default = "default_timeout")]
    pub timeout: f64,

    /// Additional task-specific config
    #[serde(default)]
    pub extra: HashMap<String, serde_json::Value>,
}

fn default_output_type() -> String {
    "generate".to_string()
}

fn default_timeout() -> f64 {
    10.0
}

impl TaskConfig {
    /// Load task config from YAML file
    pub fn from_yaml(path: &str) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Ok(serde_yaml::from_str(&content)?)
    }
}

/// Base task trait
///
/// All tasks must implement this trait to define how documents are
/// transformed into prompts, expected outputs, and test cases.
pub trait Task: Send + Sync {
    /// Get task configuration
    fn config(&self) -> &TaskConfig;

    /// Convert document to prompt text
    fn doc_to_text(&self, doc: &serde_json::Value) -> String;

    /// Extract expected output from document
    fn doc_to_target(&self, doc: &serde_json::Value) -> String;

    /// Extract test cases (for code tasks)
    fn doc_to_test(&self, doc: &serde_json::Value) -> Option<String> {
        doc.get("test")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    }

    /// Build all evaluation instances
    fn build_all_instances(&self, limit: Option<usize>) -> Result<Vec<Instance>> {
        let docs = self.load_dataset()?;
        let limited = if let Some(limit) = limit {
            docs.into_iter().take(limit).collect()
        } else {
            docs
        };

        Ok(limited
            .into_iter()
            .enumerate()
            .map(|(idx, doc)| Instance::new(doc, self.config().task_name.clone(), idx))
            .collect())
    }

    /// Load dataset (override in subclasses)
    fn load_dataset(&self) -> Result<Vec<serde_json::Value>> {
        // Default implementation for tasks without dataset
        Ok(vec![])
    }

    /// Evaluate a single instance
    fn evaluate(&self, instance: &mut Instance) -> Result<EvaluationResult>;
}

/// Example: Code generation task
pub struct CodeGenerationTask {
    config: TaskConfig,
    dataset: Option<Vec<serde_json::Value>>,
}

impl CodeGenerationTask {
    pub fn new(config: TaskConfig) -> Self {
        Self {
            config,
            dataset: None,
        }
    }

    pub fn from_yaml(path: &str) -> Result<Self> {
        Ok(Self::new(TaskConfig::from_yaml(path)?))
    }
}

impl Task for CodeGenerationTask {
    fn config(&self) -> &TaskConfig {
        &self.config
    }

    fn doc_to_text(&self, doc: &serde_json::Value) -> String {
        doc.get("prompt")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string()
    }

    fn doc_to_target(&self, doc: &serde_json::Value) -> String {
        doc.get("canonical_solution")
            .or_else(|| doc.get("solution"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string()
    }

    fn doc_to_test(&self, doc: &serde_json::Value) -> Option<String> {
        doc.get("test")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    }

    fn evaluate(&self, instance: &mut Instance) -> Result<EvaluationResult> {
        // TODO: Integrate with isolation.rs for code execution
        let response = instance.response.as_ref()
            .ok_or_else(|| anyhow::anyhow!("No response to evaluate"))?;

        let test = self.doc_to_test(&instance.doc)
            .unwrap_or_default();

        // For now, simple string matching
        let passed = !response.is_empty() && !test.is_empty();

        Ok(EvaluationResult {
            passed,
            score: if passed { 1.0 } else { 0.0 },
            details: HashMap::new(),
        })
    }

    fn load_dataset(&self) -> Result<Vec<serde_json::Value>> {
        // TODO: Load from HuggingFace datasets or local files
        if let Some(ref dataset) = self.dataset {
            Ok(dataset.clone())
        } else {
            Ok(vec![])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_config_parsing() {
        let yaml = r#"
task_name: humaneval
dataset_path: openai_humaneval
output_type: generate
num_fewshot: 0
metric_list:
  - pass_at_k
timeout: 10.0
"#;

        let config: TaskConfig = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.task_name, "humaneval");
        assert_eq!(config.num_fewshot, 0);
        assert_eq!(config.metric_list, vec!["pass_at_k"]);
    }

    #[test]
    fn test_code_generation_task() {
        let config = TaskConfig {
            task_name: "test".to_string(),
            dataset_path: None,
            dataset_name: None,
            output_type: "generate".to_string(),
            num_fewshot: 0,
            metric_list: vec![],
            timeout: 10.0,
            extra: HashMap::new(),
        };

        let task = CodeGenerationTask::new(config);

        let doc = serde_json::json!({
            "prompt": "Write a function to add two numbers",
            "canonical_solution": "def add(a, b): return a + b",
            "test": "assert add(2, 3) == 5"
        });

        assert_eq!(task.doc_to_text(&doc), "Write a function to add two numbers");
        assert_eq!(task.doc_to_target(&doc), "def add(a, b): return a + b");
        assert_eq!(task.doc_to_test(&doc), Some("assert add(2, 3) == 5".to_string()));
    }
}
