use std::sync::LazyLock;

use crate::word::Word;

const DEFAULT_SOLUTIONS: &str = include_str!("../assets/solutions.txt");
const DEFAULT_ADDITIONAL_GUESSES: &str = include_str!("../assets/additional_guesses.txt");

/// A Wordle dictionary that can verify if a word is a valid guess and produce a new random solution
pub trait Dictionary {
    /// Checks if a given word is a valid guess
    fn contains(&self, word: &Word) -> bool;

    /// Returns a random solution from the allowed list of words
    fn random_solution(&self) -> Word;
}

/// Errors that may occur when constructing a dictionary.
#[derive(Debug, Clone, thiserror::Error)]
pub enum DictionaryErr {
    /// Occurs when a dictionary is cannot be created because there are no solutions
    /// (i.e., playing a game with the dictionary would be impossible)
    #[error("No solutions in the dictionary")]
    NoSolutions,
}

mod hash_dict;
pub use hash_dict::HashDict;

mod slice_dict;
pub use slice_dict::SliceDict;

/// Returns a reference to a default/sensible dictionary implementation.
///
/// If consumers don't need ownership of the dict or a custom set of allowed words,
/// they should prefer this method over constructing a concrete implementation.
///
/// The current implementation is backed by a [`SliceDict`] that uses the list of official Wordle
/// solutions and guesses.
#[must_use]
pub fn default() -> &'static impl Dictionary {
    static DEFAULT_DICT: LazyLock<SliceDict> = LazyLock::new(SliceDict::default);
    &*DEFAULT_DICT
}
