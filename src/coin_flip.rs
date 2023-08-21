use rand::Rng;

pub fn coin_flip() -> String {
    let mut rng = rand::thread_rng();

    if rng.gen_bool(0.5) {
        "heads".to_owned()
    } else {
        "tails".to_owned()
    }
}
