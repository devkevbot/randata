use rand::Rng;

/// Simulates a fair dice roll, returning a string representation of the die face which was
/// selected. For an n-sided dice, each die face has a 1/n chance of being selected.
pub fn gen_fair_dice_roll(sides: usize) -> String {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=sides).to_string()
}
