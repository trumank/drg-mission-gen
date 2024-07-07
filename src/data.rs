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
pub struct UMissionTemplate {}

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
                mission_template: UMissionTemplate {},
                rarity: 1.0,
            },
            EMissionTemplate::MissionType_Motherlode => &FMissionTemplateItem {
                mission_template: UMissionTemplate {},
                rarity: 0.5,
            },
            EMissionTemplate::MissionType_EggCollection => &FMissionTemplateItem {
                mission_template: UMissionTemplate {},
                rarity: 0.5,
            },
            EMissionTemplate::MissionType_Elimination => &FMissionTemplateItem {
                mission_template: UMissionTemplate {},
                rarity: 0.5,
            },
            EMissionTemplate::MissionType_Salvage => &FMissionTemplateItem {
                mission_template: UMissionTemplate {},
                rarity: 0.5,
            },
            EMissionTemplate::MissionType_Escort => &FMissionTemplateItem {
                mission_template: UMissionTemplate {},
                rarity: 0.5,
            },
            EMissionTemplate::MissionType_Refinery => &FMissionTemplateItem {
                mission_template: UMissionTemplate {},
                rarity: 0.5,
            },
            EMissionTemplate::MissionType_Facility => &FMissionTemplateItem {
                mission_template: UMissionTemplate {},
                rarity: 0.0,
            },
            EMissionTemplate::MissionType_DeepScan => &FMissionTemplateItem {
                mission_template: UMissionTemplate {},
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
    pub biome: EBiome,
}
