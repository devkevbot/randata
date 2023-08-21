use crate::algo::fisher_yates;

pub fn shuffle_in_place(mut input: Vec<String>) -> String {
    match input.len() {
        // If the input is a single value, attempt to shuffle the characters contained within with
        // the input.
        1 => {
            let word = input.first().unwrap();
            let mut chars: Vec<_> = word.chars().collect();
            fisher_yates(&mut chars);
            chars.into_iter().collect()
        }
        // Else if the input has multiple values, attempt to shuffle the order of the input values.
        _ => {
            fisher_yates(&mut input);
            input.join(" ")
        }
    }
}
