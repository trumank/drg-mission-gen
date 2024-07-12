use std::iter;

use time::OffsetDateTime;

use crate::cleaned_deep_dive::{
    Complexity, DeepDive, DeepDiveSecondaryObjective, Duration, Mission, Mutator, PrimaryObjective,
    Warning,
};
use crate::deep_dive_pair::DeepDivePair;

/// Format the deep dive information according to what the usual #deep-dive-discussion weekly
/// post uses. Uses DRG main discord emojis.
pub(crate) fn format_discord(
    pair: &DeepDivePair,
    start_datetime: OffsetDateTime,
    end_datetime: OffsetDateTime,
) -> String {
    let start_date = start_datetime.date();
    let end_date = end_datetime.date();
    let end_timestamp = end_datetime.unix_timestamp();

    let dd_info = format_dive(&pair.normal);
    let edd_info = format_dive(&pair.elite);

    format!(
        "\
        Weekly Deep Dives information for **{start_date} to {end_date}**.\n\
        Deep Dives will reset **<t:{end_timestamp}:F>** (<t:{end_timestamp}:R>)\n\n\
        :Deep_Dive: __**DEEP DIVE**__ :Deep_Dive:\n\
        {dd_info}\n\n\
        :Deep_Dive: __**ELITE DEEP DIVE**__ :Deep_Dive:\n\
        {edd_info}\n\
        "
    )
}

fn format_dive(dive: &DeepDive) -> String {
    let DeepDive {
        name,
        biome,
        missions,
        ..
    } = dive;

    let basic_info = format!(
        "Region: **{biome}** | Code Name: **{codename}**",
        biome = biome.display(),
        codename = name
    );

    let [stage_1, stage_2, stage_3] = &missions[..] else {
        unreachable!("expected deep dives to have 3 stages");
    };

    let stage_1 = format_stage(1, stage_1);
    let stage_2 = format_stage(2, stage_2);
    let stage_3 = format_stage(3, stage_3);

    [basic_info, stage_1, stage_2, stage_3].join("\n")
}

fn format_stage(stage: usize, mission: &Mission) -> String {
    let primary_objective = format_primary_objective(
        &mission.primary_objective,
        mission.complexity,
        mission.duration,
    );
    let secondary_objective = format_secondary_objective(&mission.secondary_objective);

    let extra_inner = match (mission.mutator, mission.warning) {
        (Some(mutator), Some(warning)) => {
            let mutator = format_mutator(mutator);
            let warning = format_warning(warning);
            Some(format!("{mutator} {warning}"))
        }
        (Some(mutator), None) => Some(format_mutator(mutator)),
        (None, Some(warning)) => Some(format_warning(warning)),
        (None, None) => None,
    };
    let extra = extra_inner
        .map(|inner| format!(" | {inner}"))
        .unwrap_or_default();

    format!(
        "\
        Stage {stage}: \
        **{primary_objective}** + \
        **{secondary_objective}**\
        {extra}\
        "
    )
}

fn format_primary_objective(
    obj: &PrimaryObjective,
    complexity: Complexity,
    duration: Duration,
) -> String {
    match obj {
        PrimaryObjective::DeepScan => {
            format!(":pingdrg: {}", obj.display_detailed(complexity, duration))
        }
        PrimaryObjective::EscortDuty => format!(":drill: Escort Duty"),
        PrimaryObjective::MiningExpedition => {
            format!(":morkite: {}", obj.display_detailed(complexity, duration))
        }
        PrimaryObjective::IndustrialSabotage => format!(":caretaker: Industrial Sabotage"),
        PrimaryObjective::EggHunt => {
            format!(":gegg: {}", obj.display_detailed(complexity, duration))
        }
        PrimaryObjective::PointExtraction => {
            format!(":aquarq: {}", obj.display_detailed(complexity, duration))
        }
        PrimaryObjective::Refinery => {
            format!(":refinerywell: On-Site Refinery")
        }
        PrimaryObjective::Salvage => {
            format!(":molly: {}", obj.display_detailed(complexity, duration))
        }
    }
}

fn format_secondary_objective(obj: &DeepDiveSecondaryObjective) -> &'static str {
    match obj {
        DeepDiveSecondaryObjective::Eggs => ":gegg: 2 Eggs",
        DeepDiveSecondaryObjective::DeepScan => ":pingdrg: Perform 2 Deep Scans",
        DeepDiveSecondaryObjective::Blackbox => ":uplink: Black Box",
        DeepDiveSecondaryObjective::Dreadnought => ":dreadegg: Dreadnought",
        DeepDiveSecondaryObjective::Morkite => ":morkite: 150 Morkite",
        DeepDiveSecondaryObjective::Pumpjack => ":refinerywell: Connect 1 Pumpjack",
        DeepDiveSecondaryObjective::Minimules => ":molly: Repair 2 Mini-mules",
    }
}

fn format_mutator(mutator: Mutator) -> String {
    format!(":rocknstone: **{}**", mutator.display())
}

fn format_warning(warning: Warning) -> String {
    format!(":tothebone: **{}**", warning.display())
}
