use rand::Rng;

/// Performs an in-place Fisher-Yates shuffle on the input
pub fn fisher_yates<T>(seq: &mut Vec<T>) {
    let mut rng = rand::thread_rng();

    for i in (0..seq.len()).rev() {
        let j = rng.gen_range(0..=i);
        seq.swap(i, j)
    }
}
