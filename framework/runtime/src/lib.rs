//! CodeFramework runtime — app lifecycle and sandbox.

mod app;
mod loader;
mod runtime;
mod sandbox;

pub use app::CodeApp;
pub use loader::{AppLoader, LoadedApp, LoadError};
pub use runtime::{AppConfig, AppRuntime, RuntimeError};
pub use sandbox::{SandboxContext, SandboxError, SandboxLevel};
