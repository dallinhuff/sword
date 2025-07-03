use super::{Game, GameOutcome};
use crate::{Guess, Word, letter_bank::LetterBank};

const MAX_GUESSES: usize = 6;

/// A game currently in-progress.
/// When in this state, the player has not yet guessed the solution.
#[derive(Clone)]
pub struct PlayingGame {
    solution: Word,
    guesses: Vec<Guess>,
    letter_bank: LetterBank,
}

impl PlayingGame {
    /// Creates a new game from a [`Dictionary`].
    #[must_use]
    pub fn new() -> Self {
        Self {
            solution: Word::random(),
            guesses: Vec::with_capacity(MAX_GUESSES),
            letter_bank: LetterBank::new(),
        }
    }

    /// Returns the guesses the player has made already.
    #[must_use]
    pub fn guesses(&self) -> &[Guess] {
        &self.guesses
    }

    #[must_use]
    pub fn letter_bank(&self) -> &LetterBank {
        &self.letter_bank
    }

    /// Makes a guess using a given [`Word`].
    /// Returns the resulting [`Game`] state.
    pub fn guess(mut self, word: Word) -> Game {
        let guess = Guess::new(&self.solution, word);
        self.letter_bank.report_guess(&guess);
        self.guesses.push(guess);

        if self.guesses.last().is_some_and(Guess::is_correct) || self.guesses.len() >= MAX_GUESSES {
            let outcome = GameOutcome::new(self.solution, self.guesses.into_boxed_slice());
            return Game::Over(outcome);
        }

        Game::Playing(self)
    }
}

impl Default for PlayingGame {
    fn default() -> Self {
        Self::new()
    }
}
