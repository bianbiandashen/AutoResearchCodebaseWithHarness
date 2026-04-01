// Multi-layer isolation system inspired by HumanEval and SWE-bench
//
// Layer 1: Container isolation (Docker-like, using bollard crate)
// Layer 2: Process isolation (fork + resource limits)
// Layer 3: Resource monitoring (VRAM, CPU, memory)

use anyhow::{Context, Result};
use std::process::{Command, Stdio};
use std::time::Duration;

/// Isolation level for execution
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsolationLevel {
    /// No isolation (development only)
    None,
    /// Process-level isolation (multiprocessing)
    Process,
    /// Container-level isolation (Docker)
    Container,
    /// Full isolation (Docker + Process)
    Full,
}

/// Resource limits for isolated execution
#[derive(Debug, Clone)]
pub struct ResourceLimits {
    /// Maximum execution time in seconds
    pub timeout: Duration,
    /// Maximum memory in MB
    pub max_memory_mb: u64,
    /// Maximum VRAM in MB (for GPU tasks)
    pub max_vram_mb: u64,
    /// Maximum CPU cores
    pub max_cpu_cores: u32,
    /// Network access enabled
    pub network_enabled: bool,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(300), // 5 minutes
            max_memory_mb: 8192,              // 8GB
            max_vram_mb: 16384,               // 16GB
            max_cpu_cores: 4,
            network_enabled: false,
        }
    }
}

/// Result of isolated execution
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub status: ExecutionStatus,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
    pub duration: Duration,
    pub peak_memory_mb: u64,
    pub peak_vram_mb: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecutionStatus {
    Success,
    Timeout,
    MemoryLimit,
    VramLimit,
    RuntimeError,
    CompilationError,
}

/// Isolated executor combining multiple isolation layers
pub struct IsolatedExecutor {
    level: IsolationLevel,
    limits: ResourceLimits,
}

impl IsolatedExecutor {
    pub fn new(level: IsolationLevel, limits: ResourceLimits) -> Self {
        Self { level, limits }
    }

    /// Execute code with configured isolation
    pub async fn execute(&self, code: &str, test: &str) -> Result<ExecutionResult> {
        match self.level {
            IsolationLevel::None => self.execute_direct(code, test).await,
            IsolationLevel::Process => self.execute_process(code, test).await,
            IsolationLevel::Container => self.execute_container(code, test).await,
            IsolationLevel::Full => {
                // Try container first, fallback to process
                match self.execute_container(code, test).await {
                    Ok(result) => Ok(result),
                    Err(e) => {
                        eprintln!("Container execution failed: {}, falling back to process", e);
                        self.execute_process(code, test).await
                    }
                }
            }
        }
    }

    /// Direct execution (no isolation, development only)
    async fn execute_direct(&self, code: &str, test: &str) -> Result<ExecutionResult> {
        let start = std::time::Instant::now();

        // Write code to temp file
        let temp_dir = tempfile::tempdir()?;
        let code_path = temp_dir.path().join("solution.py");
        std::fs::write(&code_path, code)?;

        // Execute with test
        let output = tokio::time::timeout(
            self.limits.timeout,
            tokio::process::Command::new("python3")
                .arg("-c")
                .arg(format!("{}\n{}", code, test))
                .output(),
        )
        .await
        .context("Timeout")??;

        let duration = start.elapsed();

        Ok(ExecutionResult {
            status: if output.status.success() {
                ExecutionStatus::Success
            } else {
                ExecutionStatus::RuntimeError
            },
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            exit_code: output.status.code(),
            duration,
            peak_memory_mb: 0, // Not tracked in direct mode
            peak_vram_mb: 0,
        })
    }

    /// Process-level isolation using multiprocessing pattern from HumanEval
    async fn execute_process(&self, code: &str, test: &str) -> Result<ExecutionResult> {
        let start = std::time::Instant::now();

        // Create isolated Python script
        let isolated_script = format!(
            r#"
import signal
import resource
import sys

# Set resource limits
def set_limits():
    # Memory limit
    resource.setrlimit(resource.RLIMIT_AS, ({mem_bytes}, {mem_bytes}))
    # CPU time limit
    resource.setrlimit(resource.RLIMIT_CPU, ({cpu_sec}, {cpu_sec}))

# Timeout handler
def timeout_handler(signum, frame):
    raise TimeoutError("Execution timeout")

signal.signal(signal.SIGALRM, timeout_handler)
signal.alarm({timeout_sec})

set_limits()

# Execute code
{code}

# Execute test
{test}
"#,
            mem_bytes = self.limits.max_memory_mb * 1024 * 1024,
            cpu_sec = self.limits.timeout.as_secs(),
            timeout_sec = self.limits.timeout.as_secs(),
            code = code,
            test = test,
        );

        // Execute in isolated process
        let output = tokio::time::timeout(
            self.limits.timeout + Duration::from_secs(5), // Buffer for cleanup
            tokio::process::Command::new("python3")
                .arg("-c")
                .arg(isolated_script)
                .stdin(Stdio::null())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output(),
        )
        .await;

        let duration = start.elapsed();

        match output {
            Ok(Ok(output)) => Ok(ExecutionResult {
                status: if output.status.success() {
                    ExecutionStatus::Success
                } else if output.stderr.contains("MemoryError") {
                    ExecutionStatus::MemoryLimit
                } else if output.stderr.contains("TimeoutError") {
                    ExecutionStatus::Timeout
                } else {
                    ExecutionStatus::RuntimeError
                },
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                exit_code: output.status.code(),
                duration,
                peak_memory_mb: 0, // TODO: Parse from resource usage
                peak_vram_mb: 0,
            }),
            Ok(Err(e)) => Err(e.into()),
            Err(_) => Ok(ExecutionResult {
                status: ExecutionStatus::Timeout,
                stdout: String::new(),
                stderr: "Execution timeout".to_string(),
                exit_code: None,
                duration,
                peak_memory_mb: 0,
                peak_vram_mb: 0,
            }),
        }
    }

    /// Container-level isolation using Docker pattern from SWE-bench
    async fn execute_container(&self, code: &str, _test: &str) -> Result<ExecutionResult> {
        // TODO: Implement Docker container execution
        // This would use bollard crate for Docker API
        anyhow::bail!("Container isolation not yet implemented")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_simple_execution() {
        let executor = IsolatedExecutor::new(
            IsolationLevel::Process,
            ResourceLimits::default(),
        );

        let code = "def add(a, b): return a + b";
        let test = "assert add(2, 3) == 5";

        let result = executor.execute(code, test).await.unwrap();
        assert_eq!(result.status, ExecutionStatus::Success);
    }

    #[tokio::test]
    async fn test_timeout() {
        let executor = IsolatedExecutor::new(
            IsolationLevel::Process,
            ResourceLimits {
                timeout: Duration::from_secs(1),
                ..Default::default()
            },
        );

        let code = "import time\ntime.sleep(10)";
        let test = "";

        let result = executor.execute(code, test).await.unwrap();
        assert_eq!(result.status, ExecutionStatus::Timeout);
    }
}
