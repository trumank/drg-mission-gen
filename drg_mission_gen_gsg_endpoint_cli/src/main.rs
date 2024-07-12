mod clean;
mod cleaned_deep_dive;
mod deep_dive_pair;
mod deep_dive_response;
mod formatters;
mod gsg_endpoint;

use anyhow::Context;
use clap::Parser;
use clean::clean_unreal_deep_dive;
use deep_dive_pair::DeepDivePair;
use drg_mission_gen_core::gen_deep_dive_pair;
use tracing::*;

use deep_dive_response::DeepDiveResponse;

#[derive(Debug, Parser)]
pub struct Args {
    /// What do you want the output format to be.
    #[clap(value_enum, default_value_t = Format::Json)]
    #[arg(short, long)]
    pub format: Format,
}

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone, clap::ValueEnum)]
pub enum Format {
    Json,
    /// Simple human-friendly table format.
    Plain,
    /// Fancy version which uses Discord emojis available in the main DRG Discord server.
    Fancy,
}

pub fn main() -> anyhow::Result<()> {
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
    let deep_dive_pair = DeepDivePair {
        normal: normal_deep_dive,
        elite: elite_deep_dive,
    };

    let args = Args::parse();

    let formatted_deep_dive = match args.format {
        Format::Json => serde_json::to_string_pretty(&deep_dive_pair)?,
        Format::Plain => {
            formatters::format_plain(&deep_dive_pair, release_datetime, **expiration_datetime)
        }
        Format::Fancy => {
            formatters::format_fancy(&deep_dive_pair, release_datetime, **expiration_datetime)
        }
    };

    println!("{}", formatted_deep_dive);

    Ok(())
}
