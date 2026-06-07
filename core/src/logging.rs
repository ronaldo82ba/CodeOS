use tracing_subscriber::EnvFilter;

static LOG_INIT: std::sync::Once = std::sync::Once::new();

/// Initialize structured logging for CodeCore and system services.
pub fn init_logging() {
    LOG_INIT.call_once(|| {
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env().add_directive("codecore=info".parse().unwrap()))
            .init();
        tracing::info!("CodeCore logging initialized");
    });
}
