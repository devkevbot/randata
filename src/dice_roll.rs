use rand::Rng;

pub fn dice_roll(sides: Option<usize>) -> String {
    let sides = sides.unwrap_or(6);
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=sides).to_string()
}
