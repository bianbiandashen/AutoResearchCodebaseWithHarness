//! Integration with external tools
//!
//! Provides MCP server, Codex CLI integration, and Git operations.

pub mod git;
pub mod mcp;
pub mod codex;

pub use git::GitOps;
pub use mcp::McpServer;
pub use codex::CodexClient;
