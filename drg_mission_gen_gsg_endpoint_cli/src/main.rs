use tracing::*;

fn main() {
    drg_mission_gen_tracing::setup_logging();
    debug!("hello world");
}
