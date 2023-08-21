use clap::{Parser, Subcommand};
use rand::Rng;

fn main() {
    let cli = Cli::parse();

    let output = match cli.command {
        Commands::Word { word } => {
            // Screw non-scalar values, we ball
            let mut chars: Vec<_> = word.chars().collect();
            shuffle(&mut chars);
            chars.into_iter().collect()
        }
        Commands::Seq { mut seq } => {
            shuffle(&mut seq);
            seq.join(" ")
        }
        Commands::Numbers { length, start } => {
            let start = start.unwrap_or(1);
            let mut seq: Vec<String> = (start..start + length).map(|i| i.to_string()).collect();
            shuffle(&mut seq);
            seq.join(" ")
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
    /// Shuffles the letters in the input word and returns the output.
    Word { word: String },
    /// Shuffles the numbers in the input sequence and return the output.
    Seq { seq: Vec<String> },
    /// Generates a shuffled integer sequence.
    Numbers {
        #[arg(short, long)]
        length: usize,

        #[arg(short, long)]
        start: Option<usize>,
    },
}
