# drg-mission-gen
Piles of mission gen logic reversed from Deep Rock Galatic. Perhaps I will clean
this up and make it usable sometime.
```
cargo run --release 25165
DD = UDeepDive {
    name: "Corroded Plateau",
    biome: BIOME_LushDownpour,
    missions: [
        UGeneratedMission {
            seed: 427681440,
            template: MissionType_EggCollection,
            biome: BIOME_LushDownpour,
            primary_objective: OBJ_1st_Gather_AlienEggs,
            secondary_objectives: [
                OBJ_DD_RepairMinimules,
            ],
            mutators: [],
            warnings: [],
            complexity_limit: None,
            duration_limit: Some(
                MD_Duration_Normal,
            ),
        },
        UGeneratedMission {
            seed: 2311042610,
            template: MissionType_Motherlode,
            biome: BIOME_LushDownpour,
            primary_objective: OBJ_1st_PointExtraction,
            secondary_objectives: [
                OBJ_DD_AlienEggs,
            ],
            mutators: [],
            warnings: [
                WRN_LethalEnemies,
            ],
            complexity_limit: None,
            duration_limit: None,
        },
        UGeneratedMission {
            seed: 3720193574,
            template: MissionType_EggCollection,
            biome: BIOME_LushDownpour,
            primary_objective: OBJ_1st_Gather_AlienEggs,
            secondary_objectives: [
                OBJ_DD_DeepScan,
            ],
            mutators: [],
            warnings: [
                WRN_HeroEnemies,
            ],
            complexity_limit: None,
            duration_limit: Some(
                MD_Duration_Short,
            ),
        },
    ],
}
EDD = UDeepDive {
    name: "Outrageous Reserve",
    biome: BIOME_RadioactiveZone,
    missions: [
        UGeneratedMission {
            seed: 2520774198,
            template: MissionType_Extraction,
            biome: BIOME_RadioactiveZone,
            primary_objective: OBJ_1st_Extraction,
            secondary_objectives: [
                OBJ_DD_DeepScan,
            ],
            mutators: [],
            warnings: [
                WRN_RivalIncursion,
            ],
            complexity_limit: None,
            duration_limit: Some(
                MD_Duration_Normal,
            ),
        },
        UGeneratedMission {
            seed: 2392698723,
            template: MissionType_EggCollection,
            biome: BIOME_RadioactiveZone,
            primary_objective: OBJ_1st_Gather_AlienEggs,
            secondary_objectives: [
                OBJ_DD_MorkiteWell,
            ],
            mutators: [],
            warnings: [
                WRN_RockInfestation,
            ],
            complexity_limit: None,
            duration_limit: Some(
                MD_Duration_Short,
            ),
        },
        UGeneratedMission {
            seed: 4106333831,
            template: MissionType_Elimination,
            biome: BIOME_RadioactiveZone,
            primary_objective: OBJ_Eliminate_Eggs,
            secondary_objectives: [
                OBJ_DD_RepairMinimules,
            ],
            mutators: [],
            warnings: [
                WRN_MacteraCave,
            ],
            complexity_limit: None,
            duration_limit: Some(
                MD_Duration_Normal,
            ),
        },
    ],
}
```
