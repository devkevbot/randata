use crate::algo::fisher_yates;

pub fn shuffle(mut input: Vec<String>) -> String {
    if input.len() == 1 {
        let word = input.first().unwrap();
        let mut chars: Vec<_> = word.chars().collect();
        fisher_yates(&mut chars);
        chars.into_iter().collect()
    } else {
        fisher_yates(&mut input);
        input.join(" ")
    }
}

pub fn numbers(length: usize, start: Option<usize>) -> String {
    let start = start.unwrap_or(1);
    let mut seq: Vec<String> = (start..start + length).map(|i| i.to_string()).collect();
    fisher_yates(&mut seq);
    seq.join(" ")
}

pub fn color(format: Option<crate::color::ColorFormat>) -> String {
    let format = format.unwrap_or_default();
    crate::color::gen_color_string(&format)
}

pub fn ip_addr(format: Option<crate::ip_addr::IpAddrFormat>) -> String {
    let format = format.unwrap_or_default();
    crate::ip_addr::gen_ip_addr(&format)
}

pub fn coin_flip() -> String {
    crate::coin_flip::coin_flip()
}

pub fn dice_roll(sides: Option<usize>) -> String {
    crate::dice_roll::dice_roll(sides)
}
