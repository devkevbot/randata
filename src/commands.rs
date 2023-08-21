pub fn shuffle(input: Vec<String>) -> String {
    crate::shuffle::shuffle_in_place(input)
}

pub fn numbers(length: usize, start: Option<usize>) -> String {
    let default_start = 1;
    let start = start.unwrap_or(default_start);
    crate::numbers::gen_shuffled_seq_string(length, start)
}

pub fn color(format: Option<crate::color::ColorFormat>) -> String {
    let format = format.unwrap_or_default();
    crate::color::gen_color_string(&format)
}

pub fn ip_addr(format: Option<crate::ip_addr::IpAddrFormat>) -> String {
    let format = format.unwrap_or_default();
    crate::ip_addr::gen_ip_addr_string(&format)
}

pub fn coin_flip() -> String {
    crate::coin_flip::gen_fair_coin_flip()
}

pub fn dice_roll(sides: Option<usize>) -> String {
    let default_sides = 6;
    let sides = sides.unwrap_or(default_sides);
    crate::dice_roll::gen_fair_dice_roll(sides)
}
