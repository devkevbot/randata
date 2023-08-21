use clap::{Parser, Subcommand, ValueEnum};
use rand::Rng;

fn main() {
    let cli = Cli::parse();

    let output = match cli.command {
        Commands::Shuffle { mut input } => {
            if input.len() == 1 {
                let word = input.first().unwrap();
                let mut chars: Vec<_> = word.chars().collect();
                shuffle(&mut chars);
                chars.into_iter().collect()
            } else {
                shuffle(&mut input);
                input.join(" ")
            }
        }
        Commands::Numbers { length, start } => {
            let start = start.unwrap_or(1);
            let mut seq: Vec<String> = (start..start + length).map(|i| i.to_string()).collect();
            shuffle(&mut seq);
            seq.join(" ")
        }
        Commands::Color { format } => {
            let format = format.unwrap_or_default();
            gen_color_string(&format)
        }
    };

    println!("{output}");
}

/// Performs an in-place Fisher-Yates shuffle on the input
fn shuffle<T>(seq: &mut Vec<T>) {
    let mut rng = rand::thread_rng();

    for i in (0..seq.len()).rev() {
        let j = rng.gen_range(0..=i);
        seq.swap(i, j)
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Generates a shuffled version of the input.
    Shuffle { input: Vec<String> },
    /// Generates a shuffled integer sequence.
    Numbers {
        #[arg(short, long)]
        length: usize,

        #[arg(short, long)]
        start: Option<usize>,
    },
    // Generate a random color value.
    Color {
        #[arg(short, long)]
        format: Option<ColorFormat>,
    },
}

#[derive(Debug, Clone, ValueEnum, Default)]
enum ColorFormat {
    #[default]
    Hex,
    Hsl,
    Rgb,
}

fn gen_color_string(fmt: &ColorFormat) -> String {
    let mut rng = rand::thread_rng();

    let red = rng.gen_range(0..=255);
    let green = rng.gen_range(0..=255);
    let blue = rng.gen_range(0..=255);

    let hue = rng.gen_range(0..=360);
    let saturation = rng.gen_range(0..=100);
    let lightness = rng.gen_range(0..=100);

    match fmt {
        ColorFormat::Hex => format!("#{red:02x}{green:02x}{blue:02x}"),
        ColorFormat::Hsl => format!("({hue},{saturation}%,{lightness}%)"),
        ColorFormat::Rgb => format!("({red},{green},{blue})"),
    }
}
