// Task registry for central task discovery and loading
//
// Inspired by lm-evaluation-harness and OpenAI Evals registry pattern

use super::*;
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Task factory function type
type TaskFactory = Box<dyn Fn() -> Result<Box<dyn Task>> + Send + Sync>;

/// Central task registry
pub struct TaskRegistry {
    /// Registered task factories
    tasks: Arc<RwLock<HashMap<String, TaskFactory>>>,

    /// YAML config paths for config-based tasks
    configs: Arc<RwLock<HashMap<String, String>>>,
}

impl TaskRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(RwLock::new(HashMap::new())),
            configs: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a task by factory function
    pub fn register<F>(&self, name: impl Into<String>, factory: F)
    where
        F: Fn() -> Result<Box<dyn Task>> + Send + Sync + 'static,
    {
        let name = name.into();
        let mut tasks = self.tasks.write().unwrap();
        tasks.insert(name, Box::new(factory));
    }

    /// Register a task by YAML config path
    pub fn register_yaml(&self, name: impl Into<String>, config_path: impl Into<String>) {
        let name = name.into();
        let config_path = config_path.into();
        let mut configs = self.configs.write().unwrap();
        configs.insert(name, config_path);
    }

    /// Load a task by name
    pub fn load(&self, name: &str) -> Result<Box<dyn Task>> {
        // Try factory first
        if let Some(factory) = self.tasks.read().unwrap().get(name) {
            return factory();
        }

        // Try YAML config
        if let Some(config_path) = self.configs.read().unwrap().get(name) {
            return self.load_from_yaml(config_path);
        }

        anyhow::bail!("Task '{}' not found in registry", name)
    }

    /// Load task from YAML config
    fn load_from_yaml(&self, config_path: &str) -> Result<Box<dyn Task>> {
        let config = TaskConfig::from_yaml(config_path)
            .context(format!("Failed to load task config from {}", config_path))?;

        // For now, only support CodeGenerationTask
        // In production, would match on task type field in config
        Ok(Box::new(CodeGenerationTask::new(config)))
    }

    /// List all registered task names
    pub fn list_tasks(&self) -> Vec<String> {
        let mut names = Vec::new();

        names.extend(self.tasks.read().unwrap().keys().cloned());
        names.extend(self.configs.read().unwrap().keys().cloned());

        names.sort();
        names.dedup();
        names
    }

    /// Scan directory for YAML task configs and register them
    pub fn scan_yaml_dir(&self, dir: &str) -> Result<usize> {
        let mut count = 0;

        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("yaml")
                || path.extension().and_then(|s| s.to_str()) == Some("yml")
            {
                if let Ok(config) = TaskConfig::from_yaml(path.to_str().unwrap()) {
                    self.register_yaml(&config.task_name, path.to_str().unwrap());
                    count += 1;
                }
            }
        }

        Ok(count)
    }
}

impl Default for TaskRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Global task registry singleton
lazy_static::lazy_static! {
    pub static ref GLOBAL_REGISTRY: TaskRegistry = {
        let registry = TaskRegistry::new();

        // Register built-in tasks
        registry.register("humaneval", || {
            let config = TaskConfig {
                task_name: "humaneval".to_string(),
                dataset_path: Some("openai_humaneval".to_string()),
                dataset_name: None,
                output_type: "generate".to_string(),
                num_fewshot: 0,
                metric_list: vec!["pass_at_k".to_string()],
                timeout: 10.0,
                extra: HashMap::new(),
            };
            Ok(Box::new(CodeGenerationTask::new(config)))
        });

        registry.register("mbpp", || {
            let config = TaskConfig {
                task_name: "mbpp".to_string(),
                dataset_path: Some("mbpp".to_string()),
                dataset_name: None,
                output_type: "generate".to_string(),
                num_fewshot: 3,
                metric_list: vec!["pass_at_k".to_string()],
                timeout: 10.0,
                extra: HashMap::new(),
            };
            Ok(Box::new(CodeGenerationTask::new(config)))
        });

        registry
    };
}

/// Convenience function to load task from global registry
pub fn load_task(name: &str) -> Result<Box<dyn Task>> {
    GLOBAL_REGISTRY.load(name)
}

/// Convenience function to list all tasks
pub fn list_tasks() -> Vec<String> {
    GLOBAL_REGISTRY.list_tasks()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_basic() {
        let registry = TaskRegistry::new();

        registry.register("test_task", || {
            let config = TaskConfig {
                task_name: "test_task".to_string(),
                dataset_path: None,
                dataset_name: None,
                output_type: "generate".to_string(),
                num_fewshot: 0,
                metric_list: vec![],
                timeout: 10.0,
                extra: HashMap::new(),
            };
            Ok(Box::new(CodeGenerationTask::new(config)))
        });

        let task = registry.load("test_task").unwrap();
        assert_eq!(task.config().task_name, "test_task");
    }

    #[test]
    fn test_global_registry() {
        let tasks = list_tasks();
        assert!(tasks.contains(&"humaneval".to_string()));
        assert!(tasks.contains(&"mbpp".to_string()));

        let task = load_task("humaneval").unwrap();
        assert_eq!(task.config().task_name, "humaneval");
    }

    #[test]
    fn test_list_tasks() {
        let registry = TaskRegistry::new();
        registry.register("task1", || {
            let config = TaskConfig {
                task_name: "task1".to_string(),
                dataset_path: None,
                dataset_name: None,
                output_type: "generate".to_string(),
                num_fewshot: 0,
                metric_list: vec![],
                timeout: 10.0,
                extra: HashMap::new(),
            };
            Ok(Box::new(CodeGenerationTask::new(config)))
        });

        registry.register("task2", || {
            let config = TaskConfig {
                task_name: "task2".to_string(),
                dataset_path: None,
                dataset_name: None,
                output_type: "generate".to_string(),
                num_fewshot: 0,
                metric_list: vec![],
                timeout: 10.0,
                extra: HashMap::new(),
            };
            Ok(Box::new(CodeGenerationTask::new(config)))
        });

        let mut tasks = registry.list_tasks();
        tasks.sort();
        assert_eq!(tasks, vec!["task1", "task2"]);
    }
}
