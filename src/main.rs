use clap::{Parser, Subcommand};
use rand::Rng;

fn main() {
    let cli = Cli::parse();

    let output = match cli.command {
        Commands::Word { word } => {
            // Screw non-scalar values, we ball
            let mut chars: Vec<_> = word.chars().collect();
            fisher_yates(&mut chars);
            chars.into_iter().collect()
        }
        Commands::Seq { mut seq } => {
            fisher_yates(&mut seq);
            seq.join(" ")
        }
    };
    println!("{output}");
}

/// Performs an in-place Fisher-Yates shuffle on the input
fn fisher_yates<T>(seq: &mut Vec<T>) {
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
    /// Shuffle the letters in the word and return the scrambled output
    Word { word: String },
    /// Shuffle the numbers in the sequence and return the output
    Seq { seq: Vec<String> },
}
