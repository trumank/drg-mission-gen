use drg_mission_gen_core::{gen_deep_dive_pair, EMissionDNA, EObjective};

fn main() {
    let mut count = 0;
    for seed in 0..0x20000 {
        let (normal, _hard) = gen_deep_dive_pair(seed);

        if normal.missions[0].primary_objective != EObjective::OBJ_1st_Gather_AlienEggs {
            continue;
        }
        if normal.missions[1].primary_objective != EObjective::OBJ_1st_Extraction {
            continue;
        }
        if normal.missions[1].dna != EMissionDNA::DNA_2_01 {
            continue;
        }
        if normal.missions[2].primary_objective != EObjective::OBJ_1st_Gather_AlienEggs {
            continue;
        }
        if normal
            .missions
            .iter()
            .flat_map(|m| &m.secondary_objectives)
            .any(|s| {
                [
                    EObjective::OBJ_DD_Elimination_Eggs,
                    EObjective::OBJ_DD_Defense,
                ]
                .contains(s)
            })
        {
            continue;
        }

        count += 1;
        println!("DD: {seed} = {normal:#?}");
    }
    println!("found {count} matching seeds");
}
