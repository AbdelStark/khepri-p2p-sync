use anyhow::Result;
use log::debug;

fn main() -> Result<()> {
    env_logger::init();
    debug!("Starting khepri-p2p-sync...");
    Ok(())
}
