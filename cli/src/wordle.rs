use std::io::{BufRead as _, Write as _};

use clap::{Parser, Subcommand};
use sword_wordle::{Game, Word};

#[derive(Parser, Debug)]
pub struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Play a game
    Play,
}

pub fn run(args: &Args) -> std::io::Result<()> {
    match args.command {
        Command::Play => {
            let mut game = Game::default();
            while let Game::Playing(g) = game {
                println!("----------------------");
                for guess in g.guesses() {
                    println!("{guess}");
                }
                println!("{}", g.alphabet());
                print!("Enter guess: ");
                std::io::stdout().flush()?;

                let mut input = String::with_capacity(64);
                std::io::stdin().lock().read_line(&mut input)?;
                println!();

                let Ok(guess) = Word::new(&input) else {
                    eprintln!("Invalid word: {input}");
                    game = Game::Playing(g);
                    continue;
                };

                game = g.guess(guess);
            }

            let Game::Over(outcome) = game else {
                unreachable!();
            };

            println!("----------------------");
            for guess in outcome.guesses() {
                println!("{guess}");
            }

            if outcome.won() {
                println!("You won!");
            } else {
                println!("You lost! The solution was {}", outcome.solution());
            }

            Ok(())
        }
    }
}
