use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

// TODO
pub fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("info"))
        )
        .with(
            tracing_subscriber::fmt::layer()
            .with_target(true)
            .with_thread_ids(true)
            .pretty()
        )
        .init()
}