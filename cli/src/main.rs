use clap::{Parser, Subcommand};

mod wordle;

/// Program for creating, playing, and solving word puzzle games
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CliArgs {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Play, solve, and analyze Wordle games
    Wordle(wordle::Args),
}

fn main() -> std::io::Result<()> {
    let args = CliArgs::parse();

    match &args.command {
        Command::Wordle(args) => wordle::run(args),
    }
}
