use clap::{Parser, Subcommand};

use crate::{color::ColorFormat, commands, ip_addr::IpAddrFormat};

pub fn run() -> String {
    execute(Cli::parse())
}

fn execute(cli: Cli) -> String {
    match cli.command {
        Commands::Shuffle { input } => commands::shuffle(input),
        Commands::Numbers { length, start } => commands::numbers(length, start),
        Commands::Color { format } => commands::color(format),
        Commands::IpAddr { format } => commands::ip_addr(format),
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
