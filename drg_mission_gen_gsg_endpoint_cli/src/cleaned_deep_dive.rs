//! Cleaned Deep Dive information. This is intended to be easier to consume than raw
//! [`UDeepDive`][drg_mission_gen_core::UDeepDive] instances.

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct DeepDive {
    pub(crate) name: String,
    pub(crate) seed: u32,
    pub(crate) biome: Biome,
    pub(crate) missions: Vec<Mission>,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum Biome {
    CrystallineCaverns,
    FungusBogs,
    MagmaCore,
    RadioactiveExclusionZone,
    DenseBiozone,
    SandblastedCorridors,
    SaltPits,
    GlacialStrata,
    AzureWeald,
    HollowBough,
}

impl Biome {
    pub(crate) fn display(self) -> &'static str {
        match self {
            Biome::CrystallineCaverns => "Crystalline Caverns",
            Biome::FungusBogs => "Fungus Bogs",
            Biome::MagmaCore => "Magma Core",
            Biome::RadioactiveExclusionZone => "Radioactive Exclusion Zone",
            Biome::DenseBiozone => "Dense Biozone",
            Biome::SandblastedCorridors => "Sandblasted Corridors",
            Biome::SaltPits => "Salt Pits",
            Biome::GlacialStrata => "Glacial Strata",
            Biome::AzureWeald => "Azure Weald",
            Biome::HollowBough => "Hollow Bough",
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Mission {
    pub(crate) primary_objective: PrimaryObjective,
    pub(crate) secondary_objective: DeepDiveSecondaryObjective,
    pub(crate) mutator: Option<Mutator>,
    pub(crate) warning: Option<Warning>,
    pub(crate) complexity: Complexity,
    pub(crate) duration: Duration,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum PrimaryObjective {
    DeepScan,
    EscortDuty,
    MiningExpedition,
    IndustrialSabotage,
    EggHunt,
    PointExtraction,
    Refinery,
    Salvage,
}

impl PrimaryObjective {
    pub(crate) fn display(self) -> &'static str {
        match self {
            PrimaryObjective::DeepScan => "Deep Scan",
            PrimaryObjective::EscortDuty => "Escort Duty",
            PrimaryObjective::MiningExpedition => "Mining Expedition",
            PrimaryObjective::IndustrialSabotage => "Industrial Sabotage",
            PrimaryObjective::EggHunt => "Egg Hunt",
            PrimaryObjective::PointExtraction => "Point Extraction",
            PrimaryObjective::Refinery => "On-Site Refinery",
            PrimaryObjective::Salvage => "Salvage Operation",
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum DeepDiveSecondaryObjective {
    Eggs,
    DeepScan,
    Blackbox,
    Dreadnought,
    Morkite,
    Pumpjack,
    Minimules,
}

impl DeepDiveSecondaryObjective {
    pub(crate) fn display(self) -> &'static str {
        match self {
            DeepDiveSecondaryObjective::Eggs => "Collect 2 Eggs",
            DeepDiveSecondaryObjective::DeepScan => "Perform 2 Deep Scans",
            DeepDiveSecondaryObjective::Blackbox => "Defend Black Box",
            DeepDiveSecondaryObjective::Dreadnought => "Kill 1 Dreadnought",
            DeepDiveSecondaryObjective::Morkite => "Collect 150 Morkite",
            DeepDiveSecondaryObjective::Pumpjack => "Refine Liquid Morkite",
            DeepDiveSecondaryObjective::Minimules => "Repair 2 Mini-mules",
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum Mutator {
    VolatileGuts,
    RichAtmosphere,
    CriticalWeakness,
    BloodSugar,
    LowGravity,
}

impl Mutator {
    pub(crate) fn display(self) -> &'static str {
        match self {
            Mutator::VolatileGuts => "Volatile Guts",
            Mutator::RichAtmosphere => "Rich Atmosphere",
            Mutator::CriticalWeakness => "Critical Weakness",
            Mutator::BloodSugar => "Blood Sugar",
            Mutator::LowGravity => "Low Gravity",
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum Warning {
    RegenerativeBugs,
    EliteThreat,
    MacteraPlague,
    EboniteOutbreak,
    DuckAndCover,
    CaveLeechCluster,
    LowOxygen,
    LithophageOutbreak,
    ExploderInfestation,
    HauntedCave,
    LethalEnemies,
    ShieldDisruption,
    Parasites,
    Swarmageddon,
    RivalPresence,
}

impl Warning {
    pub(crate) fn display(self) -> &'static str {
        match self {
            Warning::RegenerativeBugs => "Regenerative Bugs",
            Warning::EliteThreat => "Elite Threat",
            Warning::MacteraPlague => "Mactera Plague",
            Warning::EboniteOutbreak => "Ebonite Outbreak",
            Warning::DuckAndCover => "Duck and Cover",
            Warning::CaveLeechCluster => "Cave Leech Cluster",
            Warning::LowOxygen => "Low Oxygen",
            Warning::LithophageOutbreak => "Lithophage Outbreak",
            Warning::ExploderInfestation => "Exploder Infestation",
            Warning::HauntedCave => "Haunted Cave",
            Warning::LethalEnemies => "Lethal Enemies",
            Warning::ShieldDisruption => "Shield Disruption",
            Warning::Parasites => "Parasites",
            Warning::Swarmageddon => "Swarmageddon",
            Warning::RivalPresence => "RivalPresence",
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub(crate) enum Complexity {
    Simple,
    Average,
    Complex,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub(crate) enum Duration {
    Short,
    Normal,
    Long,
}
