use rand::Rng;

/// Simulates a fair coin flip, returning either "heads" or "tails" with 50% probability of either.
pub fn gen_fair_coin_flip() -> String {
    let mut rng = rand::thread_rng();

    if rng.gen_bool(0.5) {
        "heads".to_owned()
    } else {
        "tails".to_owned()
    }
}
