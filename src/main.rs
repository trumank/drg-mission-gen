mod data;

use data::EPlanetZone;
use strum::VariantArray;

use crate::data::get_mission_setup;

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
        let mut rand = SRand(deep_dive_seed);
        let biome = rand.rand_item(data::EBiome::VARIANTS);
        dbg!(biome);

        // normal
        let mut rand = SRand(deep_dive_seed ^ 0x929);
        let first = rand.rand_item(data::names_first());
        let last = rand.rand_item(data::names_last());
        println!("{first} {last}");

        // elite
        let mut rand = SRand(deep_dive_seed ^ 0x1300);
        let first = rand.rand_item(data::names_first());
        let last = rand.rand_item(data::names_last());
        println!("{first} {last}");
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
