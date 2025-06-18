use std::io::{BufRead as _, Write as _};

use clap::{Parser, Subcommand};
use sword_wordle::{Game, Word, game::GuessResult};

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
            let mut game = GuessResult::Playing(Game::default());

            while let GuessResult::Playing(g) = game {
                println!("----------------------");
                for guess in g.guesses() {
                    println!("{guess}");
                }
                print!("Enter guess: ");
                std::io::stdout().flush()?;

                let mut input = String::with_capacity(64);
                std::io::stdin().lock().read_line(&mut input)?;
                println!();

                let Ok(guess) = Word::new(&input) else {
                    eprintln!("Bruh, that's not a real word");
                    game = GuessResult::Playing(g);
                    continue;
                };

                game = match g.guess(guess) {
                    GuessResult::InvalidGuess(game) => {
                        eprintln!("Bruh, that's not a real word");
                        GuessResult::Playing(game)
                    }
                    GuessResult::Playing(game) => GuessResult::Playing(game),
                    GuessResult::Over(game) => GuessResult::Over(game),
                }
            }

            let GuessResult::Over(outcome) = game else {
                panic!("Bruh");
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
