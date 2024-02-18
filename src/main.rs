use std::env;
mod app;
mod constants;
mod hash;

fn init() {
    let env_key = "RUST_LOG";
    env::set_var(env_key, "info");
    tracing_subscriber::fmt::init();
}

fn main() -> Result<(), eframe::Error> {
    init();
    app::hasher::run_hasher()
}
