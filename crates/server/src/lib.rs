//! Публичная библиотека сервера: конфиг, HTTP-транспорт, MCP-диспетчер,
//! PID-замок и сборка источников имён — для тестов и embedding-сценариев.
//! Бинарь `bsl-context-rs` использует эти же модули.
//!
//! ```
//! let cfg = bsl_context_server::config::Config::default();
//! assert!(!cfg.allowed_hosts.is_empty());
//! ```

pub mod config;
pub mod http;
pub mod mcp_server;
pub mod pid_lock;
pub mod sources;
