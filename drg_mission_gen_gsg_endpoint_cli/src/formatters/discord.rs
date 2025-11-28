use drg_mission_gen_core::EDreadnought;

use crate::cleaned_deep_dive::{
    Complexity, DeepDive, DeepDiveSecondaryObjective, Duration, Mission, Mutator, PrimaryObjective,
    Warning,
};
use crate::deep_dive_pair::DeepDivePair;
use crate::Dates;

/// Format the deep dive information according to what the usual #deep-dive-discussion weekly
/// post uses. Uses DRG main discord emojis.
pub(crate) fn format_discord(pair: &DeepDivePair, dates: Option<&Dates>) -> String {
    let start_date = dates
        .map(|d| d.start_datetime.date().to_string())
        .unwrap_or_else(|| "N/A".to_string());
    let end_date = dates
        .map(|d| d.end_datetime.date().to_string())
        .unwrap_or_else(|| "N/A".to_string());
    let end_timestamp = dates
        .map(|d| d.end_datetime.unix_timestamp())
        .unwrap_or_default();

    let seed = pair.normal.seed;
    let dd_info = format_dive(&pair.normal);
    let edd_info = format_dive(&pair.elite);

    format!(
        "\
        Weekly Deep Dives information for **{start_date} to {end_date}**.\n\
        Deep Dives will reset **<t:{end_timestamp}:F>** (<t:{end_timestamp}:R>)\n\
        Seed: {seed} \n\n\
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
    #[allow(clippy::useless_format)]
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
        PrimaryObjective::Elimination { ref targets } => {
            let target_str = format_elimination_targets(targets);
            format!(
                ":dreadegg: {} {}",
                obj.display_detailed(complexity, duration),
                target_str
            )
        }
    }
}

fn format_secondary_objective(obj: &DeepDiveSecondaryObjective) -> String {
    match obj {
        DeepDiveSecondaryObjective::Eggs => ":gegg: 2 Eggs".to_string(),
        DeepDiveSecondaryObjective::DeepScan => ":pingdrg: Perform 2 Deep Scans".to_string(),
        DeepDiveSecondaryObjective::Blackbox => ":uplink: Black Box".to_string(),
        DeepDiveSecondaryObjective::Dreadnought { targets } => {
            let target_str = format_elimination_targets(targets);
            format!(":dreadegg: Dreadnought ({})", target_str)
        }
        DeepDiveSecondaryObjective::Morkite => ":morkite: 150 Morkite".to_string(),
        DeepDiveSecondaryObjective::Pumpjack => ":refinerywell: Connect 1 Pumpjack".to_string(),
        DeepDiveSecondaryObjective::Minimules => ":molly: Repair 2 Mini-mules".to_string(),
    }
}

fn format_mutator(mutator: Mutator) -> String {
    format!(":rocknstone: **{}**", mutator.display())
}

fn format_warning(warning: Warning) -> String {
    format!(":tothebone: **{}**", warning.display())
}

fn format_enemy_descriptor(descriptor: EDreadnought) -> &'static str {
    match descriptor {
        EDreadnought::Dreadnought => "OG",
        EDreadnought::Hiveguard => "H",
        EDreadnought::Twins => "T",
    }
}

fn format_elimination_targets(targets: &[EDreadnought]) -> String {
    if targets.is_empty() {
        return String::new();
    }
    let target_strs: Vec<&str> = targets
        .iter()
        .map(|t| format_enemy_descriptor(*t))
        .collect();
    format!("({})", target_strs.join("+"))
}
