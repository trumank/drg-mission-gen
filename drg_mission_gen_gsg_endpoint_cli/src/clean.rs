//! Cleans a [`UDeepDive`][drg_mission_gen_core::UDeepDive] into a
//! [`DeepDive`][crate::cleaned_deep_dive::DeepDive] that's easier to consume.

use thiserror::Error;

use drg_mission_gen_core::{
    EBiome, EMissionComplexity, EMissionDuration, EMissionMutator, EMissionWarning, EObjective,
    UDeepDive, UGeneratedMission,
};

use crate::cleaned_deep_dive::{
    Biome, Complexity, DeepDive, DeepDiveSecondaryObjective, Duration, Mission, Mutator,
    PrimaryObjective, Warning,
};

#[derive(Debug, Error)]
pub enum CleanError {
    #[error("unexpected primary objective: `{0}`")]
    UnexpectedPrimaryObjective(&'static str),

    #[error("unexpected secondary (deep dive) objective: `{0}`")]
    UnexpectedSecondaryObjective(&'static str),

    #[error("only expected 1 secondary objective, but was given {count}")]
    SecondaryObjectivesCountMismatch { count: usize },

    #[error("only expected at most 1 mutator, but was given {count}")]
    TooManyMutators { count: usize },

    #[error("unexpected mutator: `{0}`")]
    UnexpectedMutator(&'static str),

    #[error("only expected at most 1 warning, but was given {count}")]
    TooManyWarnings { count: usize },
}

pub(crate) fn clean_unreal_deep_dive(dd: &UDeepDive) -> Result<DeepDive, CleanError> {
    let name = dd.name.clone();
    let biome = map_biome(&dd.biome);
    let missions = dd
        .missions
        .iter()
        .map(|mission| map_mission(mission))
        .collect::<Result<Vec<Mission>, CleanError>>()?;
    assert_eq!(
        missions.len(),
        3,
        "expected deep dives to have exactly 3 missions"
    );
    let seed = dd.missions[0].seed;

    Ok(DeepDive {
        name,
        seed,
        biome,
        missions,
    })
}

pub(crate) fn map_biome(biome: &EBiome) -> Biome {
    match biome {
        EBiome::BIOME_CrystalCaves => Biome::CrystallineCaverns,
        EBiome::BIOME_FungusBogs => Biome::FungusBogs,
        EBiome::BIOME_MagmaCaves => Biome::MagmaCore,
        EBiome::BIOME_RadioactiveZone => Biome::RadioactiveExclusionZone,
        EBiome::BIOME_LushDownpour => Biome::DenseBiozone,
        EBiome::BIOME_SandblastedCorridors => Biome::SandblastedCorridors,
        EBiome::BIOME_SaltCaves => Biome::SaltPits,
        EBiome::BIOME_IceCaves => Biome::GlacialStrata,
        EBiome::BIOME_AzureWeald => Biome::AzureWeald,
        EBiome::BIOME_HollowBough => Biome::HollowBough,
    }
}

pub(crate) fn map_mission(mission: &UGeneratedMission) -> Result<Mission, CleanError> {
    let primary_objective = map_primary_objective(&mission.primary_objective)?;
    let secondary_objective = map_secondary_objective(&mission.secondary_objectives)?;
    let mutator = map_mutator(&mission.mutators)?;
    let warning = map_warning(&mission.warnings)?;

    // FIXME(jieyouxu): which complexity/duration takes precedence? are they guaranteed to be
    // self-consistent?
    let mission_dna = mission.dna.get();
    let complexity = map_complexity(&mission.complexity_limit.unwrap_or(mission_dna.complexity));
    let duration = map_duration(&mission.duration_limit.unwrap_or(mission_dna.duration));

    Ok(Mission {
        primary_objective,
        secondary_objective,
        mutator,
        warning,
        complexity,
        duration,
    })
}

pub(crate) fn map_primary_objective(obj: &EObjective) -> Result<PrimaryObjective, CleanError> {
    let obj = match obj {
        EObjective::OBJ_1st_DeepScan => PrimaryObjective::DeepScan,
        EObjective::OBJ_1st_Escort => PrimaryObjective::EscortDuty,
        EObjective::OBJ_1st_Extraction => PrimaryObjective::MiningExpedition,
        EObjective::OBJ_1st_Facility => PrimaryObjective::IndustrialSabotage,
        EObjective::OBJ_1st_Gather_AlienEggs => PrimaryObjective::EggHunt,
        EObjective::OBJ_1st_PointExtraction => PrimaryObjective::PointExtraction,
        EObjective::OBJ_1st_Refinery => PrimaryObjective::Refinery,
        EObjective::OBJ_1st_Salvage => PrimaryObjective::Salvage,
        unexpected_obj => {
            return Err(CleanError::UnexpectedPrimaryObjective(
                unexpected_obj.into(),
            ))
        }
    };
    Ok(obj)
}

pub(crate) fn map_secondary_objective(
    objs: &[EObjective],
) -> Result<DeepDiveSecondaryObjective, CleanError> {
    let [obj] = objs else {
        return Err(CleanError::SecondaryObjectivesCountMismatch { count: objs.len() });
    };

    let obj = match obj {
        EObjective::OBJ_DD_AlienEggs => DeepDiveSecondaryObjective::Eggs,
        EObjective::OBJ_DD_DeepScan => DeepDiveSecondaryObjective::DeepScan,
        EObjective::OBJ_DD_Defense => DeepDiveSecondaryObjective::Blackbox,
        EObjective::OBJ_DD_Elimination_Eggs => DeepDiveSecondaryObjective::Dreadnought,
        EObjective::OBJ_DD_Morkite => DeepDiveSecondaryObjective::Morkite,
        EObjective::OBJ_DD_MorkiteWell => DeepDiveSecondaryObjective::Pumpjack,
        EObjective::OBJ_DD_RepairMinimules => DeepDiveSecondaryObjective::Minimules,
        unexpected_obj => {
            return Err(CleanError::UnexpectedSecondaryObjective(
                unexpected_obj.into(),
            ))
        }
    };
    Ok(obj)
}

pub(crate) fn map_mutator(mutators: &[EMissionMutator]) -> Result<Option<Mutator>, CleanError> {
    match mutators {
        [] => Ok(None),
        [mutator] => {
            let mutator = match mutator {
                EMissionMutator::MMUT_ExplosiveEnemies => Mutator::VolatileGuts,
                EMissionMutator::MMUT_OxygenRich => Mutator::RichAtmosphere,
                EMissionMutator::MMUT_Weakspot => Mutator::CriticalWeakness,
                EMissionMutator::MMUT_BloodSugar => Mutator::BloodSugar,
                EMissionMutator::MMUT_LowGravity => Mutator::LowGravity,
                unexpected_mutator => {
                    return Err(CleanError::UnexpectedMutator(unexpected_mutator.into()))
                }
            };
            Ok(Some(mutator))
        }
        _ => Err(CleanError::TooManyMutators {
            count: mutators.len(),
        }),
    }
}

pub(crate) fn map_warning(warnings: &[EMissionWarning]) -> Result<Option<Warning>, CleanError> {
    match warnings {
        [] => Ok(None),
        [warning] => {
            let warning = match warning {
                EMissionWarning::WRN_RegenerativeEnemies => Warning::RegenerativeBugs,
                EMissionWarning::WRN_HeroEnemies => Warning::EliteThreat,
                EMissionWarning::WRN_MacteraCave => Warning::MacteraPlague,
                EMissionWarning::WRN_RockInfestation => Warning::EboniteOutbreak,
                EMissionWarning::WRN_BulletHell => Warning::DuckAndCover,
                EMissionWarning::WRN_CaveLeechDen => Warning::CaveLeechCluster,
                EMissionWarning::WRN_NoOxygen => Warning::LowOxygen,
                EMissionWarning::WRN_Plague => Warning::LithophageOutbreak,
                EMissionWarning::WRN_ExploderInfestation => Warning::ExploderInfestation,
                EMissionWarning::WRN_Ghost => Warning::HauntedCave,
                EMissionWarning::WRN_LethalEnemies => Warning::LethalEnemies,
                EMissionWarning::WRN_NoShields => Warning::ShieldDisruption,
                EMissionWarning::WRN_InfestedEnemies => Warning::Parasites,
                EMissionWarning::WRN_Swarmagedon => Warning::Swarmageddon,
                EMissionWarning::WRN_RivalIncursion => Warning::RivalPresence,
            };
            Ok(Some(warning))
        }
        _ => Err(CleanError::TooManyWarnings {
            count: warnings.len(),
        }),
    }
}

pub(crate) fn map_complexity(complexity: &EMissionComplexity) -> Complexity {
    match complexity {
        EMissionComplexity::MD_Complexity_Complex => Complexity::Complex,
        EMissionComplexity::MD_Complexity_Average => Complexity::Average,
        EMissionComplexity::MD_Complexity_Simple => Complexity::Simple,
    }
}

pub(crate) fn map_duration(duration: &EMissionDuration) -> Duration {
    match duration {
        EMissionDuration::MD_Duration_Long => Duration::Long,
        EMissionDuration::MD_Duration_Normal => Duration::Normal,
        EMissionDuration::MD_Duration_Short => Duration::Short,
    }
}
