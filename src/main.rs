mod data;

use data::{
    get_hard_template, get_normal_template, EBiome, EMissionMutator, EMissionWarning, EObjective,
    EPlanetZone, FDeepDiveTemplateItem, FRandInterval, UDeepDive, UDeepDiveTemplate,
    UGeneratedMission,
};
use strum::VariantArray;

use crate::data::{
    get_deep_dive_settings, get_mission_setup, EMissionComplexity, EMissionDuration,
    EMissionTemplate,
};

fn main() {
    test2();
    //brute_force();
}

fn test2() {
    //let mut r = SRand(2171992800);
    for i in 0..10 {
        let mut r = SRand(i);
        r.mutate();
        println!("{}", r.get_unsigned_int() % 100);
    }
}
fn test() {
    let mut r = SRand(2171992800);
    r.mutate();
    println!("{}", r.rand_helper(221 - 1));
    r.mutate();
    println!("{}", r.rand_helper(237 - 1));
}

fn brute_force() {
    let mut r = SRand(0);
    //r.mutate();
    //while r.0 != 0 {
    for _ in 0..u32::MAX {
        let seed = r.0;
        r.mutate();
        //while r.next_seed() > 1000000000 { r.mutate() };
        if 195 == r.rand_helper(221 - 1) {
            r.mutate();
            if 15 == r.rand_helper(237 - 1) {
                println!("found match {}", seed);
            }
        }
        //println!("{}", r.rand_range(0, 1));
    }
    // mythic 195
    // full 194
    // loaded 193

    // cavity 15
}

struct SRand(u32);

impl SRand {
    fn next_seed(&self) -> u32 {
        self.0.wrapping_mul(0xbb38435).wrapping_add(0x3619636b)
    }
    fn mutate(&mut self) {
        self.0 = self.next_seed();
        //println!("MUTATE SEED {}", self.0);
    }
    fn get_fraction(&mut self) -> f32 {
        self.mutate();
        f32::from_bits(0x3f800000 | self.0 as u32 >> 9) - 1.0
    }
    fn get_unsigned_int(&mut self) -> u32 {
        //self.mutate();
        self.0
    }
    fn rand_helper(&mut self, max: i32) -> i32 {
        if max > 0 {
            (self.get_fraction() * (max as f32)) as i32
        } else {
            0
        }
    }
    fn rand_range(&mut self, min: i32, max: i32) -> i32 {
        min + self.rand_helper(max - min + 1)
    }
    fn rand_item<'a, T>(&mut self, slice: &'a [T]) -> &'a T {
        let i = self.rand_helper(slice.len() as i32) as usize;
        &slice[i]
    }
    fn rand_remove<'a, T>(&mut self, vec: &mut Vec<T>) -> T {
        let i = self.rand_helper(vec.len() as i32) as usize;
        vec.swap_remove(i)
    }
}

fn sample_zones(rand: &mut SRand, zone: EPlanetZone) -> data::EBiome {
    let total: f32 = zone
        .get()
        .biomes
        .iter()
        .map(|b| b.get().planet_zone_selection_weight)
        .sum();

    let mut sum = 0.0;
    let select = rand.get_fraction() * total;

    *zone
        .get()
        .biomes
        .iter()
        .find(|b| {
            sum += b.get().planet_zone_selection_weight;
            sum >= select
        })
        .unwrap()
}

fn sample_rand_interval(rand: &mut SRand, interval: &FRandInterval) -> i32 {
    let total: f32 = interval.intervals.iter().map(|b| b.weight).sum();

    let mut sum = 0.0;
    let select = rand.get_fraction() * total;

    interval
        .intervals
        .iter()
        .find_map(|i| {
            sum += i.weight;
            (sum >= select).then(|| rand.rand_range(i.range.min, i.range.max))
        })
        .unwrap_or_default()
}

#[derive(Debug)]
struct FPlanetZoneItem {
    biomes: Vec<data::EBiome>,
    missions: Vec<data::UGeneratedMission>,
    zone: &'static EPlanetZone,
    plague_MAYBE: bool,
    picked_MAYBE: bool,
}

fn init_helpers(rand: &mut SRand) -> Vec<FPlanetZoneItem> {
    let mut helpers = vec![];
    for zone in data::EPlanetZone::VARIANTS {
        helpers.push(FPlanetZoneItem {
            biomes: vec![sample_zones(rand, *zone)],
            missions: vec![],
            zone,
            plague_MAYBE: false,
            picked_MAYBE: false,
        })
    }
    helpers
}

fn shuffle<T>(rand: &mut SRand, vec: &mut Vec<T>) {
    for i in 0..vec.len() {
        //println!("SEED = {:X}", rand.0);
        let swap_index = rand.rand_helper(vec.len() as i32) as usize;
        println!("swap {} {}", i, swap_index);
        vec.swap(i, swap_index);
    }
}

fn randomly_shrink<T>(rand: &mut SRand, size: usize, vec: &mut Vec<T>) {
    while vec.len() > size {
        vec.swap_remove(rand.rand_helper(vec.len() as i32) as usize);
    }
}

#[derive(Debug)]
struct FGlobalMissionSeed {
    random_seed: i32,
    season: i32,
    map_key: i32,
}

fn get_missions(seed: &FGlobalMissionSeed) {
    let mut rand = SRand(seed.random_seed as u32);
    let season = data::ESeason::from_index(seed.season as usize);

    let mut helpers = init_helpers(&mut rand);

    let mut extra_biomes = {
        let range = data::get_mission_setup().extra_biomes;
        let delta = range.max - range.min + 1;
        if delta <= 0 {
            0
        } else {
            range.rand(&mut rand)
        }
    };

    rand.mutate(); // TODO unused?
    let saved = rand.0; // surely there is some logical explanation here... forking rand stream?
    let rand_helper = (rand.get_fraction() * helpers.len() as f32) as usize;
    rand.0 = saved;
    dbg!(rand_helper);

    println!("{:X}", rand.0);
    dbg!(extra_biomes);

    match season.get().mission_map_event_zone_type {
        data::ESeasonMissionMapOverlayType::None => {
            let pick = &mut helpers[rand_helper];

            pick.picked_MAYBE = true;

            for biome in pick.zone.get().biomes {
                if extra_biomes <= 0 {
                    break;
                }

                if pick.biomes.contains(biome) {
                    continue;
                }

                pick.biomes.push(*biome);

                extra_biomes -= 1;
            }
        }
        data::ESeasonMissionMapOverlayType::Plague => {
            todo!("plague regions")
        }
    }

    dbg!(&helpers);

    // TODO verify when global_missions.len() > zones
    let mut global_missions: Vec<_> = get_mission_setup()
        .global_required_missions
        .into_iter()
        .map(|m| Some(m))
        .chain(std::iter::repeat(None))
        .take(helpers.len())
        .collect();

    dbg!(&global_missions);
    shuffle(&mut rand, &mut global_missions);
    dbg!(&global_missions);
}

fn deep_dive_get_mission(
    templates: &[FDeepDiveTemplateItem],
    used_missions: &mut Vec<EMissionTemplate>,
    existing_missions: &[UGeneratedMission],
    rand: &mut SRand,
) -> (
    EMissionTemplate,
    Option<EMissionDuration>,
    Option<EMissionComplexity>,
) {
    #[derive(Debug)]
    struct HelperStructA {
        mission: EMissionTemplate,
        duration: Option<EMissionDuration>,
        complexity: Option<EMissionComplexity>,
        probability: u32,
        can_only_appear_once_per_deep_dive_set: bool,
    }

    let mut items = vec![];
    for template in templates {
        if let Some(last) = existing_missions.last() {
            if last.template == template.mission {
                // do not allow consecutive mission of same type
                continue;
            }
        }
        if template.can_only_appear_once
            && existing_missions
                .iter()
                .any(|m| m.template == template.mission)
        {
            continue;
        }
        if template.can_only_appear_once_per_deep_dive_set
            && used_missions.iter().any(|m| *m == template.mission)
        {
            continue;
        }
        if template.allowed_durations.is_empty() {
            if template.allowed_complexities.is_empty() {
                items.push(HelperStructA {
                    mission: template.mission,
                    duration: None,
                    complexity: None,
                    probability: template.probability,
                    can_only_appear_once_per_deep_dive_set: template
                        .can_only_appear_once_per_deep_dive_set,
                });
            } else {
                for complexity in template.allowed_complexities {
                    items.push(HelperStructA {
                        mission: template.mission,
                        duration: None,
                        complexity: Some(*complexity),
                        probability: template.probability,
                        can_only_appear_once_per_deep_dive_set: template
                            .can_only_appear_once_per_deep_dive_set,
                    });
                }
            }
        } else {
            for duration in template.allowed_durations {
                items.push(HelperStructA {
                    mission: template.mission,
                    duration: Some(*duration),
                    complexity: None,
                    probability: template.probability,
                    can_only_appear_once_per_deep_dive_set: template
                        .can_only_appear_once_per_deep_dive_set,
                });
            }
        }
    }

    let d = items
        .iter()
        .enumerate()
        .map(|(i, item)| format!("{i}: {:?}", item.mission))
        .collect::<Vec<_>>();
    dbg!(d);

    let mut probabilities = vec![];
    for (i, item) in items.iter().enumerate() {
        for _ in 0..item.probability {
            probabilities.push(i);
        }
    }
    let i = rand.rand_item(&probabilities);
    //dbg!(&items);
    //println!("prob = {probabilities:?}");
    let selected = &items[*i];

    if selected.can_only_appear_once_per_deep_dive_set {
        used_missions.push(selected.mission);
    }

    (selected.mission, selected.duration, selected.complexity)
}

fn select_mutator(
    mutators: &[EMissionMutator],
    primary_objective: EObjective,
    secondary_objectives: &[EObjective],
    rand: &mut SRand,
) -> EMissionMutator {
    let mut pool = mutators.to_vec();
    let mut i = pool.len() - 1;
    loop {
        let m = pool[i];
        let incompatible = m.is_banned_objective(primary_objective)
            || secondary_objectives
                .iter()
                .any(|s| m.is_banned_objective(*s));

        if incompatible {
            pool.swap_remove(i);
        }

        if i == 0 {
            break;
        }
        i -= 1;
    }

    *rand.rand_item(&pool)
}
fn select_warning(
    warnings: &[EMissionWarning],
    mutator: Option<EMissionMutator>,
    primary_objective: EObjective,
    secondary_objectives: &[EObjective],
    rand: &mut SRand,
) -> EMissionWarning {
    let mut pool = warnings.to_vec();
    let mut i = pool.len() - 1;
    loop {
        let w = pool[i];
        let incompatible = w.is_banned_objective(primary_objective)
            || secondary_objectives
                .iter()
                .any(|s| w.is_banned_objective(*s))
            || mutator.map(|m| w.is_banned_mutator(m)).unwrap_or_default();

        if incompatible {
            pool.swap_remove(i);
        }

        if i == 0 {
            break;
        }
        i -= 1;
    }

    *rand.rand_item(&pool)
}

fn gen_deep_dive(
    template: &UDeepDiveTemplate,
    seed: u32,
    biome: EBiome,
    used_missions: &mut Vec<EMissionTemplate>,
) -> UDeepDive {
    let mut rand = SRand(seed);
    let first = rand.rand_item(data::names_first());
    let last = rand.rand_item(data::names_last());
    let name = format!("{first} {last}");

    // mutators
    let mut mutator_indexes = vec![0, 1, 2];
    let mutator_count = sample_rand_interval(&mut rand, &template.mutator_count);
    randomly_shrink(&mut rand, mutator_count as usize, &mut mutator_indexes);

    // warnings
    let mut warning_indexes = vec![0, 1, 2];
    let warning_count = sample_rand_interval(&mut rand, &template.warning_count);
    randomly_shrink(&mut rand, warning_count as usize, &mut warning_indexes);

    let mut mutators = get_deep_dive_settings().mutators.to_vec();
    let mut warnings = get_deep_dive_settings().warnings.to_vec();

    let mut stages = vec![];
    for i in 0..3 {
        let stage_template =
            deep_dive_get_mission(template.missions, used_missions, &stages, &mut rand);

        rand.mutate();
        let mission_seed = rand.0;
        let mut mission_rand = SRand(mission_seed);
        let mission_template = &stage_template.0.get().mission_template;
        let primary_objective = mission_template.primary_objective;
        let secondary_objectives =
            vec![*mission_rand.rand_item(mission_template.deep_dive_objectives)];

        let mut mutator = None;
        let mut warning = None;

        if mutator_indexes.contains(&i) {
            let r = select_mutator(
                &mutators,
                primary_objective,
                &secondary_objectives,
                &mut rand,
            );
            mutators.swap_remove(mutators.iter().rposition(|i| *i == r).unwrap());
            mutator = Some(r)
        }
        if warning_indexes.contains(&i) {
            let r = select_warning(
                &warnings,
                mutator,
                primary_objective,
                &secondary_objectives,
                &mut rand,
            );
            warnings.swap_remove(warnings.iter().rposition(|i| *i == r).unwrap());
            warning = Some(r)
        }

        if i != 0 {
            // loader sequence randomness for stages 2 & 3
            rand.mutate();
        }

        let stage = UGeneratedMission {
            seed: mission_seed,
            template: stage_template.0,
            biome,
            primary_objective,
            secondary_objectives,
            warnings: warning.into_iter().collect(),
            mutators: mutator.into_iter().collect(),
            complexity_limit: stage_template.2,
            duration_limit: stage_template.1,
        };

        stages.push(stage);
    }

    UDeepDive {
        name,
        missions: stages,
        biome,
    }
}

fn gen_deep_dive_pair(seed: u32) -> (UDeepDive, UDeepDive) {
    let deep_dive_seed = seed & 0x1ffff;

    let mut rand = SRand(deep_dive_seed);
    let mut biomes = data::EBiome::VARIANTS.to_vec();

    let mut used_missions = vec![];

    let normal = gen_deep_dive(
        get_normal_template(),
        deep_dive_seed ^ 0x929,
        rand.rand_remove(&mut biomes),
        &mut used_missions,
    );
    let hard = gen_deep_dive(
        get_hard_template(),
        deep_dive_seed ^ 0x1300,
        rand.rand_remove(&mut biomes),
        &mut used_missions,
    );
    (normal, hard)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gen() {
        let seed = FGlobalMissionSeed {
            random_seed: 2,
            season: 5,
            map_key: 5,
        };

        get_missions(&seed);
    }

    #[test]
    fn test_rand1() {
        let n = 111;
        let mut u: u32 = 0;
        let mut i: i32 = 0;
        for _ in 0..n {
            u = u.wrapping_mul(0xbb38435).wrapping_add(0x3619636b);
            i = i.wrapping_mul(0xbb38435).wrapping_add(0x3619636b);
            assert_eq!(u, i as u32);
            println!("{i} {u}");
        }
    }

    #[test]
    fn test_rand2() {
        let mut rand = SRand(0x3FC2580F);
        for i in 0..4 {
            rand.mutate();
            println!("next = {:X}", rand.next_seed());
        }
    }

    #[test]
    fn test_deep_dive() {
        let seed_v2 = 83255885;
        let deep_dive_seed = seed_v2 & 0x1ffff;
        let deep_dive_seed = 122106; // prev week
                                     //let deep_dive_seed = 37731; // prev week
                                     //let deep_dive_seed = 71169; // prev week

        //for seed in 0..1000 {
        //    let (normal, hard) = gen_deep_dive_pair(seed);

        //    let a = normal
        //        .missions
        //        .iter()
        //        .any(|m| m.template == EMissionTemplate::MissionType_Facility);
        //    let b = hard
        //        .missions
        //        .iter()
        //        .any(|m| m.template == EMissionTemplate::MissionType_Facility);
        //    if a && b {
        //        dbg!(normal);
        //        dbg!(hard);
        //        println!("seed = {seed}");
        //        break;
        //    }
        //}

        let (normal, hard) = gen_deep_dive_pair(26);

        dbg!(normal);
        dbg!(hard);
    }

    #[test]
    fn test_global_mission_seed() {
        let now = time::OffsetDateTime::now_utc();
        let month = now.month() as u32;
        let day = now.day() as u32;
        let year = now.year() as u32;
        let hour = now.hour() as u32;
        let minute = now.minute() as u32;

        let seed = ((year * 0x2a90af)
            ^ (month * 0x4f9ffb7)
            ^ (day * 0x73387)
            ^ (hour * 0x5b53f5)
            ^ (minute / 30))
            % 100000;
        dbg!(seed);
    }
}
