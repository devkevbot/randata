use crate::algo::fisher_yates;

/// Generates a sequential list of integer, then shuffles them.
pub fn gen_shuffled_seq_string(length: usize, start: usize) -> String {
    let mut seq: Vec<String> = (start..start + length).map(|i| i.to_string()).collect();
    fisher_yates(&mut seq);
    seq.join(" ")
}
