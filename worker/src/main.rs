use sdk::logging;

fn main() {
    let _ = logging::init_logging();
    tracing::info!(event = "worker_startup", "StellarAid worker starting");
}
