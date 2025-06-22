use crate::dict::Dictionary;
use crate::{Word, dict};

mod outcome;
pub use outcome::GameOutcome;

mod playing;
pub use playing::PlayingGame;

/// States a wordle game may be in
#[derive(Clone)]
pub enum Game<'d> {
    /// The game is currently in progress. The player has not yet guessed the solution,
    /// but may make another guess.
    Playing(PlayingGame<'d>),

    /// The game is over, either because the player correctly guessed the solution or
    /// the player ran out of guesses.
    Over(GameOutcome),

    /// The game is currently in progress, but the previous action would have resulted in an
    /// invalid state if it had been applied.
    Invalid {
        reason: GameError,
        previous: PlayingGame<'d>,
    },
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum GameError {
    #[error("Invalid word: {0}")]
    InvalidWord(Word),
}

impl<'d> Game<'d> {
    pub fn new(dictionary: &'d dyn Dictionary) -> Self {
        Self::Playing(PlayingGame::new(dictionary))
    }
}

impl Default for Game<'_> {
    fn default() -> Self {
        Self::new(dict::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Word;

    const WORDS: [&str; 7] = [
        "audio", "crane", "crabs", "crash", "candy", "skill", "scale",
    ];
    const SOLUTION: &str = WORDS[6];

    struct FakeDict;
    impl Dictionary for FakeDict {
        fn contains(&self, word: &Word) -> bool {
            WORDS.iter().any(|&w| w == word.as_str())
        }

        fn random_solution(&self) -> Word {
            Word::new(SOLUTION).unwrap()
        }
    }

    #[test]
    fn it_results_in_win_when_correct_guess_made() {
        let guess = Word::new(SOLUTION).unwrap();
        let Game::Over(outcome) = PlayingGame::new(&FakeDict).guess(guess) else {
            panic!("Should result in outcome when solution is guessed")
        };

        assert!(outcome.won(), "Should win after guessing solution");
    }

    #[test]
    fn it_results_in_loss_when_run_out_of_guesses() {
        let mut game = PlayingGame::new(&FakeDict);

        for guess in &WORDS[..5] {
            let Game::Playing(g) = game.guess(Word::new(guess).unwrap()) else {
                panic!("Should result in active game when still guesses left");
            };

            game = g;
        }

        let Game::Over(outcome) = game.guess(Word::new(WORDS[5]).unwrap()) else {
            panic!("Should result in outcome when no guesses left");
        };

        assert!(
            !outcome.won(),
            "Should not win after running out of guesses"
        );
    }
}
