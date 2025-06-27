use crate::{Game, MAX_GUESSES, Word, alphabet::Alphabet, game::GameOutcome, guess::Guess};

/// A game currently in-progress.
#[derive(Clone)]
pub struct PlayingGame {
    solution: Word,
    guesses: Vec<Guess>,
    alphabet: Alphabet,
}

impl PlayingGame {
    /// Creates a new game from a [`Dictionary`].
    #[must_use]
    pub fn new() -> Self {
        Self {
            solution: Word::random(),
            guesses: Vec::with_capacity(MAX_GUESSES),
            alphabet: Alphabet::new(),
        }
    }

    /// Returns the guesses the player has made already.
    #[must_use]
    pub fn guesses(&self) -> &[Guess] {
        &self.guesses
    }

    /// Returns the alphabet state of the game, indicating the known letter positions of all
    /// possible letters (i.e., the state used to display the "keyboard" on the web/original
    /// wordle)
    #[must_use]
    pub fn alphabet(&self) -> &Alphabet {
        &self.alphabet
    }

    /// Makes a guess using a given [`Word`].
    /// Returns the resulting [`Game`] state.
    pub fn guess(mut self, word: Word) -> Game {
        let guess = Guess::new(&self.solution, word);
        self.alphabet.report_guess(&guess);
        self.guesses.push(guess);

        if self.guesses.last().is_some_and(Guess::is_correct) || self.guesses.len() >= MAX_GUESSES {
            return Game::Over(GameOutcome {
                solution: self.solution,
                guesses: self.guesses.into_boxed_slice(),
            });
        }

        Game::Playing(self)
    }
}

impl Default for PlayingGame {
    fn default() -> Self {
        Self::new()
    }
}
