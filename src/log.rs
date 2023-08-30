use tracing_subscriber::{filter::Directive, util::SubscriberInitExt, EnvFilter};

const DEFAULT_FILTER_LEVEL: &str = if cfg!(debug_assertions) {
    "debug"
} else {
    "info"
};

pub fn logger() -> impl SubscriberInitExt {
    const CARGO_BIN_NAME: &str = env!("CARGO_BIN_NAME");
    let default_directive = format!("{CARGO_BIN_NAME}={DEFAULT_FILTER_LEVEL}")
        .parse::<Directive>()
        .expect("default directive should be valid");

    let env_filter = EnvFilter::builder()
        .with_default_directive(default_directive)
        .from_env_lossy();

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .finish()
}
