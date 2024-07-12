mod deep_dive_response;
mod gsg_endpoint;

use anyhow::Context;
use time::Duration;
use tracing::*;

use deep_dive_response::DeepDiveResponse;

fn main() -> anyhow::Result<()> {
    drg_mission_gen_tracing::setup_logging();

    let ref deep_dive_response @ DeepDiveResponse {
        seed_v2,
        ref expiration_datetime,
        ..
    } = gsg_endpoint::query_gsg_deep_dive_endpoint().context("querying GSG deep dive endpoint")?;
    debug!(?deep_dive_response);

    let release_datetime = expiration_datetime.release_datetime();
    let (release_date, expiration_date) = (release_datetime.date(), expiration_datetime.date());
    debug!(
        release_date=%release_date,
        expiration_date=%expiration_date
    );

    Ok(())
}
