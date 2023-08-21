mod algo;
mod cli;
mod color;
mod commands;
mod ip_addr;
mod numbers;
mod shuffle;

fn main() {
    println!("{}", cli::run());
}
