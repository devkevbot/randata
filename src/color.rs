use clap::ValueEnum;
use rand::Rng;

#[derive(Debug, Clone, ValueEnum, Default)]
pub enum ColorFormat {
    #[default]
    Hex,
    Hsl,
    Rgb,
}

pub fn gen_color_string(fmt: &ColorFormat) -> String {
    let mut rng = rand::thread_rng();

    let red = rng.gen_range(u8::MIN..=u8::MAX);
    let green = rng.gen_range(u8::MIN..=u8::MAX);
    let blue = rng.gen_range(u8::MIN..=u8::MAX);

    let hue = rng.gen_range(0..=360);
    let saturation = rng.gen_range(0..=100);
    let lightness = rng.gen_range(0..=100);

    match fmt {
        ColorFormat::Hex => format!("#{red:02x}{green:02x}{blue:02x}"),
        ColorFormat::Hsl => format!("({hue},{saturation}%,{lightness}%)"),
        ColorFormat::Rgb => format!("({red},{green},{blue})"),
    }
}
