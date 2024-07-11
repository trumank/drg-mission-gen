use drg_mission_gen_core::gen_deep_dive_pair;

fn main() {
    let (normal, hard) = gen_deep_dive_pair(
        std::env::args()
            .nth(1)
            .expect("expected seed")
            .parse()
            .expect("failed to parse seed as number"),
    );
    println!("DD = {normal:#?}");
    println!("EDD = {hard:#?}");
}
