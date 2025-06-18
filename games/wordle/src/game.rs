use std::marker::PhantomData;

use crate::{MAX_GUESSES, dict};
use crate::{dict::Dictionary, guess::Guess, word::Word};

mod private {
    /// A un-implementable marker trait to make [`State`](super::State) a sealed trait
    pub trait Sealed {}
}

/// A marker trait to implement the type-state pattern for [`Game`]
pub trait State: private::Sealed {}

/// State marker indicating that a [`Game`] is still playing/in-progress
pub enum Playing {}
impl private::Sealed for Playing {}
impl State for Playing {}

/// State marker indicating that a [`Game`] is over and additional guesses cannot be made
pub enum Over {}
impl private::Sealed for Over {}
impl State for Over {}

/// A Wordle game, either in-progress or complete
pub struct Game<'d, S: State> {
    /// The dictionary of [`Words`](Word) that are allowed to be guesses
    dictionary: &'d dyn Dictionary,

    /// The solution to the puzzle the player is attempting to guess
    solution: Word,

    /// Guesses the player has made (so far)
    guesses: Vec<Guess>,

    /// The state the game is in ([`Playing`] or [`Over`])
    state: PhantomData<S>,
}

impl Default for Game<'_, Playing> {
    fn default() -> Self {
        Self::new(dict::default())
    }
}

impl<S: State> Game<'_, S> {
    /// Returns a list of guesses the player has made (so far)
    #[must_use]
    pub fn guesses(&self) -> &[Guess] {
        &self.guesses
    }
}

impl Game<'_, Over> {
    /// Returns the solution to the puzzle
    #[must_use]
    pub fn solution(&self) -> &Word {
        &self.solution
    }

    /// Returns whether the player successfully guessed the solution
    pub fn won(&self) -> bool {
        self.guesses().last().is_some_and(Guess::is_correct)
    }
}

impl<'d> Game<'d, Playing> {
    /// Creates a new game instance from a dictionary of allowed [`Words`](Word).
    pub fn new(dictionary: &'d dyn Dictionary) -> Self {
        Game {
            dictionary,
            solution: dictionary.random_solution(),
            guesses: Vec::with_capacity(MAX_GUESSES),
            state: PhantomData,
        }
    }

    /// Attempts to submit a [`Word`] as a guess, returning the resulting `Game` state
    /// discriminated by a tag indicating whether the guess was succesfully submitted
    pub fn guess(mut self, word: Word) -> GuessResult<'d> {
        if !self.dictionary.contains(&word) {
            return GuessResult::InvalidGuess(self);
        }

        self.guesses.push(Guess::new(&self.solution, word));

        if self.guesses().last().is_some_and(Guess::is_correct) || self.guesses.len() >= MAX_GUESSES
        {
            GuessResult::Over(Game {
                dictionary: self.dictionary,
                solution: self.solution,
                guesses: self.guesses,
                state: PhantomData,
            })
        } else {
            GuessResult::Playing(self)
        }
    }
}

/// The possible outcomes from attempting to submit a guess with [`Game::guess`]
pub enum GuessResult<'d> {
    /// The guess was successfully submitted, but it was not the solution. The player still has
    /// more opportunities to guess the solution
    Playing(Game<'d, Playing>),

    /// The guess was invalid because it is not in the game's dictionary. The player still has more
    /// opportunities to guess the solution
    InvalidGuess(Game<'d, Playing>),

    /// The guess was successfully submitted and the game is over, either because the solution was
    /// guessed or the player ran out of guesses
    Over(Game<'d, Over>),
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let GuessResult::Over(outcome) = Game::new(&FakeDict).guess(guess) else {
            panic!("Should result in outcome when solution is guessed")
        };

        assert!(outcome.won(), "Should win after guessing solution");
    }

    #[test]
    fn it_results_in_loss_when_run_out_of_guesses() {
        let mut game = Game::new(&FakeDict);

        for guess in &WORDS[..5] {
            let GuessResult::Playing(g) = game.guess(Word::new(guess).unwrap()) else {
                panic!("Should result in active game when still guesses left");
            };

            game = g;
        }

        let GuessResult::Over(outcome) = game.guess(Word::new(WORDS[5]).unwrap()) else {
            panic!("Should result in outcome when no guesses left");
        };

        println!("{}", outcome.guesses.last().unwrap());

        assert!(
            !outcome.won(),
            "Should not win after running out of guesses"
        );
    }
}
