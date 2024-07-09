use strum::VariantArray;

pub type TArray<T> = Vec<T>;

#[derive(Debug, Clone, Copy, PartialEq, VariantArray)]
pub enum EPlanetZone {
    PZ_Zone01,
    PZ_Zone02,
    PZ_Zone03,
    PZ_Zone04,
}
impl EPlanetZone {
    pub fn get(&self) -> &'static UPlanetZone {
        match self {
            EPlanetZone::PZ_Zone01 => &UPlanetZone {
                biomes: &[EBiome::BIOME_CrystalCaves, EBiome::BIOME_SaltCaves],
                required_missions: &[
                    FRequiredMissionItem {
                        mission_template: EMissionTemplate::MissionType_Extraction,
                        complexity: EMissionComplexity::MD_Complexity_Simple,
                        duration: EMissionDuration::MD_Duration_Short,
                        can_have_mutators: false,
                    },
                    FRequiredMissionItem {
                        mission_template: EMissionTemplate::MissionType_EggCollection,
                        complexity: EMissionComplexity::MD_Complexity_Simple,
                        duration: EMissionDuration::MD_Duration_Short,
                        can_have_mutators: false,
                    },
                    FRequiredMissionItem {
                        mission_template: EMissionTemplate::MissionType_Refinery,
                        complexity: EMissionComplexity::MD_Complexity_Average,
                        duration: EMissionDuration::MD_Duration_Normal,
                        can_have_mutators: false,
                    },
                ],
            },
            EPlanetZone::PZ_Zone02 => &UPlanetZone {
                biomes: &[EBiome::BIOME_RadioactiveZone, EBiome::BIOME_FungusBogs],
                required_missions: &[
                    FRequiredMissionItem {
                        mission_template: EMissionTemplate::MissionType_Extraction,
                        complexity: EMissionComplexity::MD_Complexity_Simple,
                        duration: EMissionDuration::MD_Duration_Normal,
                        can_have_mutators: false,
                    },
                    FRequiredMissionItem {
                        mission_template: EMissionTemplate::MissionType_Salvage,
                        complexity: EMissionComplexity::MD_Complexity_Average,
                        duration: EMissionDuration::MD_Duration_Normal,
                        can_have_mutators: false,
                    },
                ],
            },
            EPlanetZone::PZ_Zone03 => &UPlanetZone {
                biomes: &[
                    EBiome::BIOME_LushDownpour,
                    EBiome::BIOME_IceCaves,
                    EBiome::BIOME_HollowBough,
                ],
                required_missions: &[
                    FRequiredMissionItem {
                        mission_template: EMissionTemplate::MissionType_EggCollection,
                        complexity: EMissionComplexity::MD_Complexity_Average,
                        duration: EMissionDuration::MD_Duration_Normal,
                        can_have_mutators: false,
                    },
                    FRequiredMissionItem {
                        mission_template: EMissionTemplate::MissionType_Motherlode,
                        complexity: EMissionComplexity::MD_Complexity_Complex,
                        duration: EMissionDuration::MD_Duration_Normal,
                        can_have_mutators: false,
                    },
                ],
            },
            EPlanetZone::PZ_Zone04 => &UPlanetZone {
                biomes: &[
                    EBiome::BIOME_MagmaCaves,
                    EBiome::BIOME_SandblastedCorridoors,
                    EBiome::BIOME_AzureWeald,
                ],
                required_missions: &[
                    FRequiredMissionItem {
                        mission_template: EMissionTemplate::MissionType_Elimination,
                        complexity: EMissionComplexity::MD_Complexity_Average,
                        duration: EMissionDuration::MD_Duration_Normal,
                        can_have_mutators: false,
                    },
                    FRequiredMissionItem {
                        mission_template: EMissionTemplate::MissionType_Escort,
                        complexity: EMissionComplexity::MD_Complexity_Average,
                        duration: EMissionDuration::MD_Duration_Normal,
                        can_have_mutators: false,
                    },
                    FRequiredMissionItem {
                        mission_template: EMissionTemplate::MissionType_DeepScan,
                        complexity: EMissionComplexity::MD_Complexity_Average,
                        duration: EMissionDuration::MD_Duration_Short,
                        can_have_mutators: false,
                    },
                ],
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, VariantArray)]
pub enum EBiome {
    BIOME_CrystalCaves,
    BIOME_FungusBogs,
    BIOME_MagmaCaves,
    BIOME_RadioactiveZone,
    BIOME_LushDownpour,
    BIOME_SandblastedCorridoors,
    BIOME_SaltCaves,
    BIOME_IceCaves,
    BIOME_AzureWeald,
    BIOME_HollowBough,
}
impl EBiome {
    pub fn get(&self) -> &'static UBiome {
        match self {
            EBiome::BIOME_CrystalCaves => &UBiome {
                planet_zone_selection_weight: 1.0,
            },
            EBiome::BIOME_FungusBogs => &UBiome {
                planet_zone_selection_weight: 1.0,
            },
            EBiome::BIOME_MagmaCaves => &UBiome {
                planet_zone_selection_weight: 1.0,
            },
            EBiome::BIOME_RadioactiveZone => &UBiome {
                planet_zone_selection_weight: 1.0,
            },
            EBiome::BIOME_LushDownpour => &UBiome {
                planet_zone_selection_weight: 1.0,
            },
            EBiome::BIOME_SandblastedCorridoors => &UBiome {
                planet_zone_selection_weight: 1.0,
            },
            EBiome::BIOME_SaltCaves => &UBiome {
                planet_zone_selection_weight: 1.0,
            },
            EBiome::BIOME_IceCaves => &UBiome {
                planet_zone_selection_weight: 1.0,
            },
            EBiome::BIOME_AzureWeald => &UBiome {
                planet_zone_selection_weight: 2.0,
            },
            EBiome::BIOME_HollowBough => &UBiome {
                planet_zone_selection_weight: 1.0,
            },
        }
    }
}

pub struct UBiome {
    pub planet_zone_selection_weight: f32,
}

#[derive(Debug)]
pub struct UPlanetZone {
    pub biomes: &'static [EBiome],
    pub required_missions: &'static [FRequiredMissionItem],
}

#[derive(Debug, Clone)]
pub struct FRequiredMissionItem {
    pub mission_template: EMissionTemplate,
    pub complexity: EMissionComplexity,
    pub duration: EMissionDuration,
    pub can_have_mutators: bool,
}
#[derive(Debug)]
pub struct FMissionTemplateItem {
    pub mission_template: UMissionTemplate,
    pub rarity: f32,
}

#[derive(Debug)]
pub struct UMissionTemplate {
    pub primary_objective: EObjective,
    pub deep_dive_objectives: &'static [EObjective],
}

#[derive(Debug, Clone, Copy, PartialEq, VariantArray)]
pub enum EMissionTemplate {
    MissionType_Extraction,
    MissionType_Motherlode,
    MissionType_EggCollection,
    MissionType_Elimination,
    MissionType_Salvage,
    MissionType_Escort,
    MissionType_Refinery,
    MissionType_Facility,
    MissionType_DeepScan,
}
impl EMissionTemplate {
    pub fn get(&self) -> &'static FMissionTemplateItem {
        match self {
            EMissionTemplate::MissionType_Extraction => &FMissionTemplateItem {
                mission_template: UMissionTemplate {
                    primary_objective: EObjective::OBJ_1st_Extraction,
                    deep_dive_objectives: &[
                        EObjective::OBJ_DD_AlienEggs,
                        EObjective::OBJ_DD_Elimination_Eggs,
                        EObjective::OBJ_DD_Defense,
                        EObjective::OBJ_DD_RepairMinimules,
                        EObjective::OBJ_DD_DeepScan,
                        EObjective::OBJ_DD_MorkiteWell,
                    ],
                },
                rarity: 1.0,
            },
            EMissionTemplate::MissionType_Motherlode => &FMissionTemplateItem {
                mission_template: UMissionTemplate {
                    primary_objective: EObjective::OBJ_1st_PointExtraction,
                    deep_dive_objectives: &[
                        EObjective::OBJ_DD_AlienEggs,
                        EObjective::OBJ_DD_Morkite,
                        EObjective::OBJ_DD_Elimination_Eggs,
                        EObjective::OBJ_DD_Defense,
                        EObjective::OBJ_DD_DeepScan,
                        EObjective::OBJ_DD_MorkiteWell,
                    ],
                },
                rarity: 0.5,
            },
            EMissionTemplate::MissionType_EggCollection => &FMissionTemplateItem {
                mission_template: UMissionTemplate {
                    primary_objective: EObjective::OBJ_1st_Gather_AlienEggs,
                    deep_dive_objectives: &[
                        EObjective::OBJ_DD_Morkite,
                        EObjective::OBJ_DD_Elimination_Eggs,
                        EObjective::OBJ_DD_Defense,
                        EObjective::OBJ_DD_RepairMinimules,
                        EObjective::OBJ_DD_DeepScan,
                        EObjective::OBJ_DD_MorkiteWell,
                    ],
                },
                rarity: 0.5,
            },
            EMissionTemplate::MissionType_Elimination => &FMissionTemplateItem {
                mission_template: UMissionTemplate {
                    primary_objective: EObjective::OBJ_Eliminate_Eggs,
                    deep_dive_objectives: &[
                        EObjective::OBJ_DD_AlienEggs,
                        EObjective::OBJ_DD_Morkite,
                        EObjective::OBJ_DD_Defense,
                        EObjective::OBJ_DD_RepairMinimules,
                        EObjective::OBJ_DD_DeepScan,
                        EObjective::OBJ_DD_MorkiteWell,
                    ],
                },
                rarity: 0.5,
            },
            EMissionTemplate::MissionType_Salvage => &FMissionTemplateItem {
                mission_template: UMissionTemplate {
                    primary_objective: EObjective::OBJ_1st_Salvage,
                    deep_dive_objectives: &[
                        EObjective::OBJ_DD_AlienEggs,
                        EObjective::OBJ_DD_Morkite,
                        EObjective::OBJ_DD_Elimination_Eggs,
                        EObjective::OBJ_DD_DeepScan,
                        EObjective::OBJ_DD_MorkiteWell,
                    ],
                },
                rarity: 0.5,
            },
            EMissionTemplate::MissionType_Escort => &FMissionTemplateItem {
                mission_template: UMissionTemplate {
                    primary_objective: EObjective::OBJ_1st_Escort,
                    deep_dive_objectives: &[
                        EObjective::OBJ_DD_AlienEggs,
                        EObjective::OBJ_DD_Morkite,
                        EObjective::OBJ_DD_RepairMinimules,
                        EObjective::OBJ_DD_DeepScan,
                        EObjective::OBJ_DD_MorkiteWell,
                    ],
                },
                rarity: 0.5,
            },
            EMissionTemplate::MissionType_Refinery => &FMissionTemplateItem {
                mission_template: UMissionTemplate {
                    primary_objective: EObjective::OBJ_1st_Refinery,
                    deep_dive_objectives: &[
                        EObjective::OBJ_DD_AlienEggs,
                        EObjective::OBJ_DD_Morkite,
                        EObjective::OBJ_DD_Elimination_Eggs,
                        EObjective::OBJ_DD_Defense,
                        EObjective::OBJ_DD_DeepScan,
                    ],
                },
                rarity: 0.5,
            },
            EMissionTemplate::MissionType_Facility => &FMissionTemplateItem {
                mission_template: UMissionTemplate {
                    primary_objective: EObjective::OBJ_1st_Facility,
                    deep_dive_objectives: &[
                        EObjective::OBJ_DD_AlienEggs,
                        EObjective::OBJ_DD_Morkite,
                        EObjective::OBJ_DD_Elimination_Eggs,
                        EObjective::OBJ_DD_RepairMinimules,
                        EObjective::OBJ_DD_DeepScan,
                        EObjective::OBJ_DD_MorkiteWell,
                    ],
                },
                rarity: 0.0,
            },
            EMissionTemplate::MissionType_DeepScan => &FMissionTemplateItem {
                mission_template: UMissionTemplate {
                    primary_objective: EObjective::OBJ_1st_DeepScan,
                    deep_dive_objectives: &[
                        EObjective::OBJ_DD_Morkite,
                        EObjective::OBJ_DD_Elimination_Eggs,
                        EObjective::OBJ_DD_Defense,
                        EObjective::OBJ_DD_RepairMinimules,
                        EObjective::OBJ_DD_AlienEggs,
                        EObjective::OBJ_DD_MorkiteWell,
                    ],
                },
                rarity: 1.0,
            },
        }
    }
}

#[derive(Debug)]
pub struct UMissionComplexity {}
#[derive(Debug, Clone, Copy, PartialEq, VariantArray)]
pub enum EMissionComplexity {
    MD_Complexity_Complex,
    MD_Complexity_Average,
    MD_Complexity_Simple,
}

#[derive(Debug)]
pub struct UMissionDuration {}
#[derive(Debug, Clone, Copy, PartialEq, VariantArray)]
pub enum EMissionDuration {
    MD_Duration_Long,
    MD_Duration_Normal,
    MD_Duration_Short,
}

#[derive(Debug, Clone, Copy)]
pub struct FIRandRange {
    pub min: i32,
    pub max: i32,
}
impl FIRandRange {
    pub fn rand(&self, rand: &mut crate::SRand) -> i32 {
        // TODO verify correctness
        rand.rand_range(self.min, self.max)
    }
}

#[derive(Debug)]
pub struct UMissionSetup {
    pub extra_biomes: FIRandRange,
    pub global_required_missions: &'static [FRequiredMissionItem],
}
pub fn get_mission_setup() -> &'static UMissionSetup {
    &UMissionSetup {
        extra_biomes: FIRandRange { min: 2, max: 2 },
        global_required_missions: &[
            FRequiredMissionItem {
                mission_template: EMissionTemplate::MissionType_Facility,
                complexity: EMissionComplexity::MD_Complexity_Simple,
                duration: EMissionDuration::MD_Duration_Normal,
                can_have_mutators: false,
            },
            FRequiredMissionItem {
                mission_template: EMissionTemplate::MissionType_Facility,
                complexity: EMissionComplexity::MD_Complexity_Average,
                duration: EMissionDuration::MD_Duration_Normal,
                can_have_mutators: true,
            },
        ],
    }
}

#[derive(Debug)]
pub struct UDeepDiveSettings {
    pub mutators: &'static [EMissionMutator],
    pub warnings: &'static [EMissionWarning],
}
pub fn get_deep_dive_settings() -> &'static UDeepDiveSettings {
    &UDeepDiveSettings {
        mutators: &[
            EMissionMutator::MMUT_LowGravity,
            EMissionMutator::MMUT_OxygenRich,
            EMissionMutator::MMUT_ExplosiveEnemies,
            EMissionMutator::MMUT_Weakspot,
            EMissionMutator::MMUT_BloodSugar,
        ],
        warnings: &[
            EMissionWarning::WRN_RegenerativeEnemies,
            EMissionWarning::WRN_LethalEnemies,
            EMissionWarning::WRN_NoOxygen,
            EMissionWarning::WRN_InfestedEnemies,
            EMissionWarning::WRN_Ghost,
            EMissionWarning::WRN_CaveLeechDen,
            EMissionWarning::WRN_ExploderInfestation,
            EMissionWarning::WRN_MacteraCave,
            EMissionWarning::WRN_NoShields,
            EMissionWarning::WRN_HeroEnemies,
            EMissionWarning::WRN_Swarmagedon,
            EMissionWarning::WRN_RivalIncursion,
            EMissionWarning::WRN_BulletHell,
            EMissionWarning::WRN_RockInfestation,
            EMissionWarning::WRN_BulletHell,
        ],
    }
}

#[derive(Debug)]
pub struct UDeepDiveTemplate {
    pub missions: &'static [FDeepDiveTemplateItem],
    pub mutator_count: FRandInterval,
    pub warning_count: FRandInterval,
}

#[derive(Debug)]
pub struct FRandInterval {
    pub intervals: &'static [FRandIntervalItem],
}
#[derive(Debug)]
pub struct FRandIntervalItem {
    pub weight: f32,
    pub range: FIRandRange,
}

#[derive(Debug)]
pub struct FDeepDiveTemplateItem {
    pub mission: EMissionTemplate,
    pub probability: u32,
    pub allowed_durations: &'static [EMissionDuration],
    pub allowed_complexities: &'static [EMissionComplexity],
    pub can_only_appear_once: bool,
    pub can_only_appear_once_per_deep_dive_set: bool,
}

pub fn get_normal_template() -> &'static UDeepDiveTemplate {
    &UDeepDiveTemplate {
        mutator_count: FRandInterval {
            intervals: &[
                FRandIntervalItem {
                    weight: 1.0,
                    range: FIRandRange { min: 0, max: 0 },
                },
                FRandIntervalItem {
                    weight: 1.0,
                    range: FIRandRange { min: 1, max: 1 },
                },
            ],
        },
        warning_count: FRandInterval {
            intervals: &[
                FRandIntervalItem {
                    weight: 1.0,
                    range: FIRandRange { min: 2, max: 2 },
                },
                FRandIntervalItem {
                    weight: 1.0,
                    range: FIRandRange { min: 2, max: 2 },
                },
            ],
        },
        missions: &[
            FDeepDiveTemplateItem {
                mission: EMissionTemplate::MissionType_Elimination,
                probability: 10,
                allowed_durations: &[EMissionDuration::MD_Duration_Normal],
                allowed_complexities: &[],
                can_only_appear_once: false,
                can_only_appear_once_per_deep_dive_set: false,
            },
            FDeepDiveTemplateItem {
                mission: EMissionTemplate::MissionType_Extraction,
                probability: 10,
                allowed_durations: &[
                    EMissionDuration::MD_Duration_Normal,
                    EMissionDuration::MD_Duration_Short,
                ],
                allowed_complexities: &[],
                can_only_appear_once: false,
                can_only_appear_once_per_deep_dive_set: false,
            },
            FDeepDiveTemplateItem {
                mission: EMissionTemplate::MissionType_Salvage,
                probability: 10,
                allowed_durations: &[],
                allowed_complexities: &[],
                can_only_appear_once: false,
                can_only_appear_once_per_deep_dive_set: false,
            },
            FDeepDiveTemplateItem {
                mission: EMissionTemplate::MissionType_EggCollection,
                probability: 10,
                allowed_durations: &[
                    EMissionDuration::MD_Duration_Normal,
                    EMissionDuration::MD_Duration_Short,
                ],
                allowed_complexities: &[],
                can_only_appear_once: false,
                can_only_appear_once_per_deep_dive_set: false,
            },
            FDeepDiveTemplateItem {
                mission: EMissionTemplate::MissionType_Motherlode,
                probability: 10,
                allowed_durations: &[],
                allowed_complexities: &[],
                can_only_appear_once: false,
                can_only_appear_once_per_deep_dive_set: false,
            },
            FDeepDiveTemplateItem {
                mission: EMissionTemplate::MissionType_Refinery,
                probability: 10,
                allowed_durations: &[EMissionDuration::MD_Duration_Normal],
                allowed_complexities: &[],
                can_only_appear_once: false,
                can_only_appear_once_per_deep_dive_set: false,
            },
            FDeepDiveTemplateItem {
                mission: EMissionTemplate::MissionType_Escort,
                probability: 7,
                allowed_durations: &[EMissionDuration::MD_Duration_Normal],
                allowed_complexities: &[],
                can_only_appear_once: true,
                can_only_appear_once_per_deep_dive_set: false,
            },
            FDeepDiveTemplateItem {
                mission: EMissionTemplate::MissionType_Facility,
                probability: 5,
                allowed_durations: &[],
                allowed_complexities: &[EMissionComplexity::MD_Complexity_Simple],
                can_only_appear_once: false,
                can_only_appear_once_per_deep_dive_set: true,
            },
            FDeepDiveTemplateItem {
                mission: EMissionTemplate::MissionType_DeepScan,
                probability: 10,
                allowed_durations: &[],
                allowed_complexities: &[],
                can_only_appear_once: false,
                can_only_appear_once_per_deep_dive_set: false,
            },
        ],
    }
}
pub fn get_hard_template() -> &'static UDeepDiveTemplate {
    &UDeepDiveTemplate {
        mutator_count: FRandInterval {
            intervals: &[
                FRandIntervalItem {
                    weight: 1.0,
                    range: FIRandRange { min: 0, max: 0 },
                },
                FRandIntervalItem {
                    weight: 1.0,
                    range: FIRandRange { min: 1, max: 1 },
                },
            ],
        },
        warning_count: FRandInterval {
            intervals: &[FRandIntervalItem {
                weight: 1.0,
                range: FIRandRange { min: 2, max: 3 },
            }],
        },
        missions: &[
            FDeepDiveTemplateItem {
                mission: EMissionTemplate::MissionType_Elimination,
                probability: 10,
                allowed_durations: &[EMissionDuration::MD_Duration_Normal],
                allowed_complexities: &[],
                can_only_appear_once: false,
                can_only_appear_once_per_deep_dive_set: false,
            },
            FDeepDiveTemplateItem {
                mission: EMissionTemplate::MissionType_Extraction,
                probability: 10,
                allowed_durations: &[
                    EMissionDuration::MD_Duration_Normal,
                    EMissionDuration::MD_Duration_Short,
                ],
                allowed_complexities: &[],
                can_only_appear_once: false,
                can_only_appear_once_per_deep_dive_set: false,
            },
            FDeepDiveTemplateItem {
                mission: EMissionTemplate::MissionType_Salvage,
                probability: 10,
                allowed_durations: &[],
                allowed_complexities: &[],
                can_only_appear_once: false,
                can_only_appear_once_per_deep_dive_set: false,
            },
            FDeepDiveTemplateItem {
                mission: EMissionTemplate::MissionType_EggCollection,
                probability: 10,
                allowed_durations: &[
                    EMissionDuration::MD_Duration_Normal,
                    EMissionDuration::MD_Duration_Short,
                ],
                allowed_complexities: &[],
                can_only_appear_once: false,
                can_only_appear_once_per_deep_dive_set: false,
            },
            FDeepDiveTemplateItem {
                mission: EMissionTemplate::MissionType_Motherlode,
                probability: 10,
                allowed_durations: &[],
                allowed_complexities: &[],
                can_only_appear_once: false,
                can_only_appear_once_per_deep_dive_set: false,
            },
            FDeepDiveTemplateItem {
                mission: EMissionTemplate::MissionType_Refinery,
                probability: 10,
                allowed_durations: &[EMissionDuration::MD_Duration_Normal],
                allowed_complexities: &[],
                can_only_appear_once: false,
                can_only_appear_once_per_deep_dive_set: false,
            },
            FDeepDiveTemplateItem {
                mission: EMissionTemplate::MissionType_Escort,
                probability: 7,
                allowed_durations: &[EMissionDuration::MD_Duration_Normal],
                allowed_complexities: &[],
                can_only_appear_once: true,
                can_only_appear_once_per_deep_dive_set: false,
            },
            FDeepDiveTemplateItem {
                mission: EMissionTemplate::MissionType_Facility,
                probability: 5,
                allowed_durations: &[],
                allowed_complexities: &[EMissionComplexity::MD_Complexity_Simple],
                can_only_appear_once: false,
                can_only_appear_once_per_deep_dive_set: true,
            },
            FDeepDiveTemplateItem {
                mission: EMissionTemplate::MissionType_DeepScan,
                probability: 10,
                allowed_durations: &[],
                allowed_complexities: &[],
                can_only_appear_once: false,
                can_only_appear_once_per_deep_dive_set: false,
            },
        ],
    }
}

#[derive(Debug, Clone, Copy, PartialEq, VariantArray)]
pub enum EObjective {
    OBJ_1st_DeepScan,
    OBJ_1st_Escort,
    OBJ_1st_Extraction,
    OBJ_1st_Facility,
    OBJ_1st_Gather_AlienEggs,
    OBJ_1st_PointExtraction,
    OBJ_1st_Refinery,
    OBJ_1st_Salvage,
    OBJ_1st_Tutorial,
    OBJ_2nd_DestroyBhaBarnacles,
    OBJ_2nd_DestroyEggs,
    OBJ_2nd_Find_ApocaBloom,
    OBJ_2nd_Find_BooloCap,
    OBJ_2nd_Find_Ebonut,
    OBJ_2nd_Find_Fossil,
    OBJ_2nd_Find_Gunkseed,
    OBJ_2nd_KillFleas,
    OBJ_2nd_Mine_Dystrum,
    OBJ_2nd_Mine_Hollomite,
    OBJ_DD_AlienEggs,
    OBJ_DD_DeepScan,
    OBJ_DD_Defense,
    OBJ_DD_Elimination_Eggs,
    OBJ_DD_Morkite,
    OBJ_DD_MorkiteWell,
    OBJ_DD_RepairMinimules,
    OBJ_Eliminate_Eggs,
    OBJ_Elimination_Base,
    OBJ_Extraction_Base,
    OBJ_FindItems_Base,
    OBJ_Gather_Gems_Base,
    OBJ_WRN_Plague,
}

#[derive(Debug, Clone, Copy, PartialEq, VariantArray)]
pub enum EMissionMutator {
    MMUT_ExplosiveEnemies,
    MMUT_ExterminationContract,
    MMUT_SecretSecondary,
    MMUT_XXXP,
    MMUT_GoldRush,
    MMUT_OxygenRich,
    MMUT_RichInMinerals,
    MMUT_Weakspot,
    MMUT_BloodSugar,
    MMUT_LowGravity,
}

#[derive(Debug, Clone, Copy, PartialEq, VariantArray)]
pub enum EMissionWarning {
    WRN_RegenerativeEnemies,
    WRN_HeroEnemies,
    WRN_MacteraCave,
    WRN_RockInfestation,
    WRN_BulletHell,
    WRN_CaveLeechDen,
    WRN_NoOxygen,
    WRN_Plague,
    WRN_ExploderInfestation,
    WRN_Ghost,
    WRN_LethalEnemies,
    WRN_NoShields,
    WRN_InfestedEnemies,
    WRN_Swarmagedon,
    WRN_RivalIncursion,
}
impl EMissionMutator {
    pub fn is_banned_objective(self, obj: EObjective) -> bool {
        match self {
            EMissionMutator::MMUT_BloodSugar => [EObjective::OBJ_1st_Facility].as_slice(),
            _ => &[],
        }
        .contains(&obj)
    }
}
impl EMissionWarning {
    pub fn is_banned_objective(self, obj: EObjective) -> bool {
        match self {
            EMissionWarning::WRN_Plague => [EObjective::OBJ_1st_Escort].as_slice(),
            EMissionWarning::WRN_NoOxygen => [EObjective::OBJ_1st_Salvage].as_slice(),
            EMissionWarning::WRN_CaveLeechDen => [
                EObjective::OBJ_1st_PointExtraction,
                EObjective::OBJ_1st_Facility,
            ]
            .as_slice(),
            EMissionWarning::WRN_RockInfestation => [
                EObjective::OBJ_1st_Salvage,
                EObjective::OBJ_1st_PointExtraction,
                EObjective::OBJ_1st_Refinery,
            ]
            .as_slice(),
            EMissionWarning::WRN_Ghost => [
                EObjective::OBJ_1st_Salvage,
                EObjective::OBJ_DD_Defense,
                EObjective::OBJ_1st_Escort,
                EObjective::OBJ_1st_Facility,
                EObjective::OBJ_1st_DeepScan,
            ]
            .as_slice(),
            EMissionWarning::WRN_RivalIncursion => {
                [EObjective::OBJ_1st_Facility, EObjective::OBJ_1st_Escort].as_slice()
            }
            _ => &[],
        }
        .contains(&obj)
    }
    pub fn is_banned_mutator(self, mutator: EMissionMutator) -> bool {
        match self {
            EMissionWarning::WRN_NoOxygen => [EMissionMutator::MMUT_OxygenRich].as_slice(),
            _ => &[],
        }
        .contains(&mutator)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, VariantArray)]
pub enum ESeason {
    Season0,
    Season1,
    Season2,
    Season3,
    Season4,
    Season5,
}
impl ESeason {
    pub fn from_index(index: usize) -> Self {
        Self::VARIANTS[index]
    }
    pub fn get(&self) -> &'static USeason {
        match self {
            ESeason::Season0 => &USeason {
                mission_map_event_zone_type: ESeasonMissionMapOverlayType::None,
            },
            ESeason::Season1 => &USeason {
                mission_map_event_zone_type: ESeasonMissionMapOverlayType::None,
            },
            ESeason::Season2 => &USeason {
                mission_map_event_zone_type: ESeasonMissionMapOverlayType::None,
            },
            ESeason::Season3 => &USeason {
                mission_map_event_zone_type: ESeasonMissionMapOverlayType::Plague,
            },
            ESeason::Season4 => &USeason {
                mission_map_event_zone_type: ESeasonMissionMapOverlayType::Plague,
            },
            ESeason::Season5 => &USeason {
                mission_map_event_zone_type: ESeasonMissionMapOverlayType::None,
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ESeasonMissionMapOverlayType {
    None,
    Plague,
}

#[derive(Debug)]
pub struct USeason {
    pub mission_map_event_zone_type: ESeasonMissionMapOverlayType,
}

#[derive(Debug)]
pub struct UGeneratedMission {
    pub seed: u32,
    pub template: EMissionTemplate,
    pub biome: EBiome,
    pub primary_objective: EObjective,
    pub secondary_objectives: Vec<EObjective>,
    pub mutators: Vec<EMissionMutator>,
    pub warnings: Vec<EMissionWarning>,
    pub complexity_limit: Option<EMissionComplexity>,
    pub duration_limit: Option<EMissionDuration>,
}

#[derive(Debug)]
pub struct UDeepDive {
    pub name: String,
    pub missions: Vec<UGeneratedMission>,
    pub biome: EBiome,
}

pub fn names_first() -> &'static [&'static str] {
    &[
        "Angry",
        "Deep",
        "Burning",
        "Corrosive",
        "Frozen",
        "Sudden",
        "Abyssal",
        "Bronze",
        "Snowy",
        "Karl's",
        "Murderous",
        "Evil",
        "Infernal",
        "Jealous",
        "Deadly",
        "Tainted",
        "First",
        "Sacred",
        "Wicked",
        "Cold",
        "Rough",
        "Bleak",
        "Wild",
        "Dead",
        "Green",
        "Forgotten",
        "Raw",
        "Forbidden",
        "Endless",
        "Brutal",
        "Shattered",
        "Fractured",
        "Sharp",
        "Last",
        "Second",
        "Infinity",
        "Heavy",
        "Empty",
        "Sunken",
        "Low",
        "Fathomless",
        "Wide",
        "Buried",
        "Distant",
        "Shallow",
        "Dreadful",
        "Awful",
        "Bad",
        "Spiked",
        "Broken",
        "Jagged",
        "Dark",
        "Shining",
        "Naked",
        "Bare",
        "Natural",
        "Bared",
        "Peeled",
        "Open",
        "Unveiled",
        "Exposed",
        "Crying",
        "Unrobed",
        "Uncovered",
        "Loud",
        "Everlasting",
        "Burning",
        "Dusty",
        "Dirty",
        "Clouded",
        "Weeping",
        "Devil's",
        "Rejected",
        "Xehn's",
        "Sad",
        "Loaded",
        "True",
        "Worthy",
        "Divine",
        "Pure",
        "Solid",
        "Imperfect",
        "Noble",
        "Rippled",
        "Secret",
        "Hidden",
        "Covered",
        "Unknown",
        "Foggy",
        "Dim",
        "Murky",
        "Silent",
        "Crimson",
        "Cimmerian",
        "Emerald",
        "Petrified",
        "Coward's",
        "Bloodstained",
        "Final",
        "Darkest",
        "Blunt",
        "Fragile",
        "Crumbly",
        "Bright",
        "Glowing",
        "Illuminated",
        "Whalepiper's",
        "Spineless",
        "Fearless",
        "Fearful",
        "Gutless",
        "Frightened",
        "Bold",
        "Rapid",
        "Brisk",
        "Brave",
        "Cruel",
        "Blue",
        "Abandoned",
        "Alien",
        "Strange",
        "Creeping",
        "Warrior's",
        "Decayed",
        "Eternal",
        "Petrified",
        "Morning",
        "Forsaken",
        "Giant",
        "Gargantuan",
        "Mammoth",
        "Colossal",
        "Infested",
        "Titanic",
        "Bloody",
        "Stinking",
        "Hideous",
        "Corrupt",
        "Gunner's",
        "Calm",
        "Wild",
        "Crazy",
        "Insane",
        "Beginner's",
        "Mad",
        "Madmen's",
        "Lucky",
        "Deadman's",
        "Loser's",
        "Weak",
        "Crumbling",
        "Fragile",
        "Freak",
        "Desperate",
        "Furious",
        "Secure",
        "Sharp",
        "Meek",
        "Fool's",
        "Mild",
        "Foul",
        "Rancid",
        "High",
        "Hard",
        "Impure",
        "Stale",
        "Unhealthy",
        "Clean",
        "Purified",
        "Mighty",
        "Raging",
        "Great",
        "Leaf Lover's",
        "Corrosive",
        "Bitter",
        "Hermit's",
        "Solitary",
        "Pale",
        "Rotten",
        "Corroded",
        "Corrupt",
        "Ancient",
        "Old",
        "Shattered",
        "Ruptured",
        "Compact",
        "Total",
        "Stony",
        "Rocky",
        "Solid",
        "Big",
        "Massive",
        "Broken",
        "Loaded",
        "Full",
        "Mythic",
        "Scarred",
        "Defect",
        "Exposed",
        "Useless",
        "Hunter's",
        "Ranger's",
        "Scout's",
        "Engineer's",
        "Driller's",
        "Fast",
        "Bedazzled",
        "Dry",
        "Fierce",
        "Carnal",
        "Bestial",
        "Feral",
        "Irrational",
        "Barbarous",
        "Outrageous",
        "Dastardly",
        "Duplicitous",
        "Absolute",
        "Gangrenous",
        "Covetous",
        "Carnivorous",
    ]
}

pub fn names_last() -> &'static [&'static str] {
    &[
        "Overhang",
        "Shroud",
        "Core",
        "Depths",
        "Carve",
        "Thunder",
        "Strike",
        "Force",
        "Anvil",
        "Killing",
        "Shaft",
        "Shelf",
        "Downfall",
        "Avalanche",
        "Skull",
        "Cavity",
        "Breeze",
        "Hole",
        "Earth",
        "Walk",
        "Scream",
        "Hate",
        "Impact",
        "Hammer",
        "Point",
        "Grave",
        "Edge",
        "Blade",
        "Chasm",
        "End",
        "Dawn",
        "Legacy",
        "Descent",
        "Bottom",
        "Cap",
        "Key",
        "Ledge",
        "Land",
        "Arm",
        "Senit",
        "Plateau",
        "Level",
        "Abyss",
        "Hollow",
        "Shelter",
        "Hideout",
        "Crypt",
        "Caves",
        "Sanctuary",
        "Den",
        "Axe",
        "Rock",
        "Echo",
        "Sadness",
        "Fear",
        "Needle",
        "Wrath",
        "Fury",
        "Madness",
        "Rage",
        "Vault",
        "Cellar",
        "Cell",
        "Tomb",
        "Crater",
        "Armpit",
        "Pit",
        "Void",
        "Decay",
        "Basin",
        "Base",
        "Chamber",
        "Gap",
        "Pocket",
        "Trench",
        "Bed",
        "Reserve",
        "Well",
        "Ditch",
        "Trail",
        "Belly",
        "Slope",
        "Womb",
        "Drop",
        "Expanse",
        "Clearance",
        "Claw",
        "Passage",
        "Path",
        "Outpost",
        "Clearing",
        "Territory",
        "Pass",
        "Crosscut",
        "Contact",
        "Track",
        "Gate",
        "Hell",
        "Inferno",
        "Nightmare",
        "Agony",
        "Sorrow",
        "Gods",
        "Rising",
        "Base",
        "Hope",
        "Betrayal",
        "Let-down",
        "Pitfall",
        "Honor",
        "Heart",
        "Relief",
        "Field",
        "Wreck",
        "Hulrum",
        "Mork",
        "Face",
        "Hand",
        "Foot",
        "Death",
        "Carcass",
        "Citadel",
        "Palace",
        "Catacomb",
        "Boneyard",
        "Cemetery",
        "Vault",
        "Grotto",
        "Memorial",
        "Night",
        "Dusk",
        "Eclipse",
        "Desert",
        "Wilderness",
        "Summit",
        "Outback",
        "Wasteland",
        "Barrens",
        "Fence",
        "Soil",
        "Badland",
        "Ground",
        "Elevation",
        "Dome",
        "Drift",
        "Bluff",
        "Lodge",
        "Bedrock",
        "Breach",
        "Split",
        "Burrow",
        "Covert",
        "Pocket",
        "Mouth",
        "Break",
        "Lair",
        "Nest",
        "Gorge",
        "Blank",
        "Ravine",
        "Fissure",
        "Keep",
        "Enclosure",
        "Inland",
        "Head",
        "Arm",
        "Crown",
        "Overlook",
        "Darkness",
        "Blackout",
        "Blackness",
        "Nightfall",
        "Haunt",
        "Oddness",
        "Habitat",
        "Digs",
        "Crib",
        "Cover",
        "Shelter",
        "Retreat",
        "Sleep",
        "Fangs",
        "Chance",
        "Luck",
        "Shock",
        "Outpost",
        "Border",
        "Roof",
        "Doom",
        "Ceiling",
        "Ghost",
        "Run",
        "Raid",
        "End",
        "Dream",
        "Trail",
        "Dump",
        "Patrol",
        "Pursuit",
        "Risk",
        "Bones",
        "Grin",
        "Look",
        "View",
        "Valley",
        "Thickening",
        "Trick",
        "Tongue",
        "Helix",
        "Hook",
        "Hunt",
        "Catch",
        "Jewel",
        "Find",
        "Reserve",
        "Pearl",
        "Wealth",
        "Unfortune",
        "Feast",
        "Salute",
        "Joy",
        "Prize",
        "Ideal",
        "Perfection",
        "Hold",
        "Fort",
        "Goods",
        "Guts",
        "Interest",
        "Position",
        "Leverage",
        "Benefit",
        "Return",
        "Comeback",
        "Derail",
        "Delight",
        "Tunnel",
    ]
}
