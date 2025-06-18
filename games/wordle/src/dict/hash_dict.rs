use rand::Rng as _;

use crate::{
    Word,
    dict::{DEFAULT_ADDITIONAL_GUESSES, DEFAULT_SOLUTIONS, Dictionary, DictionaryErr},
};

/// An instance of [`Dictionary`] that uses hashsets of words as the backing representation.
///
/// Lookups are constant and solution generation is a linear time operation.
///
/// For larger dictionaries, this may be more performant for lookups, but the fact that
/// solution generation is a linear operation is a significant tradeoff.
pub struct HashDict {
    solutions: std::collections::HashSet<Word>,
    additional_guesses: std::collections::HashSet<Word>,
}

impl HashDict {
    /// Creates a new `HashDict`
    ///
    /// # Errors
    ///
    /// Returns a `DictionaryError::NoSolutions` error if solutions is empty.
    ///
    /// ```
    /// # use sword_wordle::dict::{DictionaryErr, HashDict};
    /// let result = HashDict::new([], []);
    /// assert!(matches!(result, Err(DictionaryErr::NoSolutions)));
    /// ```
    pub fn new(
        solutions: impl IntoIterator<Item = Word>,
        additional_guesses: impl IntoIterator<Item = Word>,
    ) -> Result<Self, DictionaryErr> {
        let solutions: std::collections::HashSet<_> = solutions.into_iter().collect();
        if solutions.is_empty() {
            return Err(DictionaryErr::NoSolutions);
        }

        Ok(HashDict {
            solutions,
            additional_guesses: additional_guesses.into_iter().collect(),
        })
    }
}

impl Default for HashDict {
    fn default() -> Self {
        Self::new(
            DEFAULT_SOLUTIONS.lines().filter_map(|w| Word::new(w).ok()),
            DEFAULT_ADDITIONAL_GUESSES
                .lines()
                .filter_map(|w| Word::new(w).ok()),
        )
        .expect("default solutions should be non-empty")
    }
}

impl Dictionary for HashDict {
    fn contains(&self, word: &Word) -> bool {
        self.solutions.contains(word) || self.additional_guesses.contains(word)
    }

    fn random_solution(&self) -> Word {
        self.solutions
            .iter()
            .nth(rand::rng().random_range(0..self.solutions.len()))
            .copied()
            .expect("dict should not be empty")
    }
}
