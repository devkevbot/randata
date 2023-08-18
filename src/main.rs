use clap::{Parser, Subcommand};
use rand::Rng;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Word { word } => {
            let result = fisher_yates(word.as_str());
            println!("{result}")
        }
    }
}

/// Performs a Fisher-Yates shuffle of the characters in the input `&str`.
fn fisher_yates(word: &str) -> String {
    let mut rng = rand::thread_rng();

    // Screw non-scalar values, we ball
    let mut chars: Vec<_> = word.chars().collect();

    for i in (0..word.len()).rev() {
        let j = rng.gen_range(0..=i);
        chars.swap(i, j)
    }

    chars.into_iter().collect()
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
}
