use tabled::settings::Style;
use tabled::Table;
use time::OffsetDateTime;

use crate::cleaned_deep_dive::DeepDive;
use crate::deep_dive_pair::DeepDivePair;

use super::mission_to_stage;

pub(crate) fn format_plain(
    pair: &DeepDivePair,
    start_datetime: OffsetDateTime,
    end_datetime: OffsetDateTime,
) -> String {
    let header = "=== Deep Dive Info ===".to_string();
    let start_date = start_datetime.date();
    let end_date = end_datetime.date();
    let week_start = format!("Start: {start_date}");
    let week_end = format!("End: {end_date}");
    let seed = format!("Seed: {}", pair.elite.seed);

    let normal_dd_title = "=== Normal Deep Dive ===".to_string();
    let normal_info = format_plain_dd(&pair.normal);

    let elite_dd_title = "=== Elite Deep Dive ===".to_string();
    let elite_info = format_plain_dd(&pair.elite);

    [
        header,
        week_start,
        week_end,
        seed,
        String::new(),
        normal_dd_title,
        normal_info,
        String::new(),
        elite_dd_title,
        elite_info,
        String::new(),
    ]
    .join("\n")
}

fn format_plain_dd(dd: &DeepDive) -> String {
    let codename = format!("Codename: {}", dd.name);
    let biome = format!("Biome: {}", dd.biome.display());
    let rows = dd
        .missions
        .iter()
        .enumerate()
        .map(|(i, m)| mission_to_stage(i, m))
        .collect::<Vec<_>>();
    let mut table = Table::new(rows);
    table.with(Style::ascii());

    [
        codename,
        biome,
        String::new(),
        table.to_string(),
        String::new(),
    ]
    .join("\n")
}
