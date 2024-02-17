use std::env;

fn init() {
    let env_key = "RUST_LOG";
    env::set_var(env_key, "debug");
    tracing_subscriber::fmt::init();
}

fn main() {
    init();
    tracing::info!("INFO");
    tracing::debug!("DEBUF");
    tracing::error!("ERROR");
}
