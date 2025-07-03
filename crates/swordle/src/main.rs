use std::io::{self, BufRead as _, Write as _};

use swordle::{Game, Word};

fn main() -> io::Result<()> {
    let mut game = Game::default();
    while let Game::Playing(g) = game {
        println!("----------------------");
        for guess in g.guesses() {
            println!("{guess}");
        }
        print!("Enter guess: ");
        io::stdout().flush()?;

        let mut input = String::with_capacity(64);
        io::stdin().lock().read_line(&mut input)?;
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
