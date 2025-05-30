use clap::Parser;

/// Program for creating, playing, and solving word puzzle games
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {}

fn main() {
    let _args = Args::parse();
}
