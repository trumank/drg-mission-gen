mod clean;
mod cleaned_deep_dive;
mod deep_dive_response;
mod gsg_endpoint;

use anyhow::Context;
use clap::Parser;
use clean::clean_unreal_deep_dive;
use drg_mission_gen_core::gen_deep_dive_pair;
use tracing::*;

use deep_dive_response::DeepDiveResponse;

#[derive(Debug, Parser)]
struct Args {}

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

    let (normal_deep_dive, elite_deep_dive) = gen_deep_dive_pair(seed_v2 as u32);
    debug!(?normal_deep_dive);
    debug!(?elite_deep_dive);

    let (normal_deep_dive, elite_deep_dive) = (
        clean_unreal_deep_dive(&normal_deep_dive)?,
        clean_unreal_deep_dive(&elite_deep_dive)?,
    );
    debug!(?normal_deep_dive);
    debug!(?elite_deep_dive);

    Ok(())
}
