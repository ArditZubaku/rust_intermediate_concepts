use log::{debug, error, info, trace, warn};

// RUST_LOG=trace cargo run
fn main() {
    env_logger::init();

    debug!("debug");
    error!("error");
    info!("info");
    trace!("trace");
    warn!("warn");
}
