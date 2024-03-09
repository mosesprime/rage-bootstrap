use rage_bootstrap::{debug, error, info, warn};

fn main() {
    debug!("debug {} {}", "1", 2);
    info!("info");
    warn!("warn");
    error!("error");
}
