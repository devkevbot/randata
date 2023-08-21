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
        Commands::IpAddr { format } => {
            let format = format.unwrap_or_default();
            gen_ip_addr(&format)
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
    // Generates a random IP address.
    #[command(name = "ip")]
    IpAddr {
        #[arg(short, long)]
        format: Option<IpAddrFormat>,
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

#[derive(Debug, Clone, ValueEnum, Default)]
enum IpAddrFormat {
    #[default]
    Ipv4,
    Ipv6,
}

fn gen_ip_addr(fmt: &IpAddrFormat) -> String {
    let mut rng = rand::thread_rng();

    match fmt {
        IpAddrFormat::Ipv4 => {
            let num_groups = 4;
            let mut groups = Vec::with_capacity(num_groups);

            for _ in 0..num_groups {
                let group = rng.gen_range(u8::MIN..=u8::MAX);
                groups.push(group.to_string())
            }

            groups.join(".")
        }
        IpAddrFormat::Ipv6 => {
            let num_groups = 8;
            let mut groups = Vec::with_capacity(num_groups);

            for _ in 0..num_groups {
                let group = rng.gen_range(u16::MIN..=u16::MAX);
                groups.push(format!("{group:04x}"))
            }

            groups.join(":")
        }
    }
}
