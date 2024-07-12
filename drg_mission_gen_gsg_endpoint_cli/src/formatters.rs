use tabled::settings::Style;
use tabled::{Table, Tabled};
use time::OffsetDateTime;

use crate::cleaned_deep_dive::{DeepDive, Mission};
use crate::deep_dive_pair::DeepDivePair;

#[derive(Tabled)]
#[tabled(rename_all = "PascalCase")]
struct Stage {
    stage: usize,
    primary: &'static str,
    secondary: &'static str,
    warning: &'static str,
    mutator: &'static str,
}

fn mission_to_stage(i: usize, m: &Mission) -> Stage {
    Stage {
        stage: i + 1,
        primary: m
            .primary_objective
            .display_detailed(m.complexity, m.duration),
        secondary: m.secondary_objective.display(),
        warning: m.warning.map(|w| w.display()).unwrap_or(""),
        mutator: m.mutator.map(|mt| mt.display()).unwrap_or(""),
    }
}

pub(crate) fn format_plain(
    pair: &DeepDivePair,
    start_datetime: OffsetDateTime,
    end_datetime: OffsetDateTime,
) -> String {
    let header = format!("=== Deep Dive Info ===");
    let start_date = start_datetime.date();
    let end_date = end_datetime.date();
    let week_start = format!("Start: {start_date}");
    let week_end = format!("End: {end_date}");
    let seed = format!("Seed: {}", pair.elite.seed);

    let normal_dd_title = format!("=== Normal Deep Dive ===");
    let normal_info = format_plain_dd(&pair.normal);

    let elite_dd_title = format!("=== Elite Deep Dive ===");
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

pub(crate) fn format_fancy(
    pair: &DeepDivePair,
    start_datetime: OffsetDateTime,
    end_datetime: OffsetDateTime,
) -> String {
    unimplemented!(
        "this is intended to be the #deep-dive-discussion format, but is not yet implemented"
    )
}
