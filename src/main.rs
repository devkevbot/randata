mod algo;
mod cli;
mod coin_flip;
mod color;
mod commands;
mod dice_roll;
mod ip_addr;
mod numbers;
mod shuffle;

fn main() {
    println!("{}", cli::run());
}
