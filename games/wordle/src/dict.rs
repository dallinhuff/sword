use crate::word::Word;
use rand::{Rng, seq::IndexedRandom};

const DEFAULT_SOLUTIONS: &str = include_str!("../assets/solutions.txt");
const DEFAULT_ADDITIONAL_GUESSES: &str = include_str!("../assets/additional_guesses.txt");

/// A Wordle dictionary that can verify if a word is a valid guess and produce a new random solution
pub trait Dictionary {
    /// Check if a given word is a valid guess
    fn contains(&self, word: &Word) -> bool;

    /// Get a random solution from the allowed list of words
    fn random_solution(&self) -> Word;
}

/// Errors that may occur when constructing a dictionary.
#[derive(Debug, Clone, thiserror::Error)]
pub enum DictionaryErr {
    #[error("No solutions in the dictionary")]
    NoSolutions,
}

/// An instance of [`Dictionary`] that uses sorted slices of words as the backing representation.
///
/// Lookups are logarithmic and solution generation is a constant time operation.
///
/// For smaller/average dictionaries (n < 10k), this will be the most performant option as the
/// entries are stored in contiguous memory blocks and lookups consist of two binary searches.
pub struct SliceDict {
    solutions: Box<[Word]>,
    additional_guesses: Box<[Word]>,
}

impl SliceDict {
    /// Creates a new `SliceDict`.
    ///
    /// # Errors
    ///
    /// Returns a `DictionaryError::NoSolutions` error if solutions is empty.
    ///
    /// ```
    /// # use sword_wordle::dict::{DictionaryErr, SliceDict};
    /// let result = SliceDict::new([], []);
    /// assert!(matches!(result, Err(DictionaryErr::NoSolutions)));
    /// ```
    pub fn new(
        solutions: impl IntoIterator<Item = Word>,
        additional_guesses: impl IntoIterator<Item = Word>,
    ) -> Result<Self, DictionaryErr> {
        let solutions = solutions.into_sorted_slice();
        if solutions.is_empty() {
            return Err(DictionaryErr::NoSolutions);
        }

        Ok(SliceDict {
            solutions,
            additional_guesses: additional_guesses.into_sorted_slice(),
        })
    }
}

impl Default for SliceDict {
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

impl Dictionary for SliceDict {
    fn contains(&self, word: &Word) -> bool {
        [&self.solutions, &self.additional_guesses]
            .iter()
            .any(|v| v.binary_search(word).is_ok())
    }

    fn random_solution(&self) -> Word {
        self.solutions
            .choose(&mut rand::rng())
            .copied()
            .expect("dict should not be empty")
    }
}

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

trait IntoSortedSlice<Item: Ord> {
    fn into_sorted_slice(self) -> Box<[Item]>;
}

impl<Collection, Item> IntoSortedSlice<Item> for Collection
where
    Collection: IntoIterator<Item = Item>,
    Item: Ord,
{
    fn into_sorted_slice(self) -> Box<[Item]> {
        let mut collected: Vec<Item> = self.into_iter().collect();
        collected.sort_unstable();
        collected.into_boxed_slice()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_has_words_from_assets_files() {
        let dict = SliceDict::default();

        for word in ["berry", "fifth", "truss"] {
            assert!(
                dict.contains(&Word::new(word).unwrap()),
                "dict should contain word from solutions: {word}",
            );
        }

        for word in ["belay", "boxes"] {
            assert!(
                dict.contains(&Word::new(word).unwrap()),
                "dict should contain word from additional_guesses: {word}",
            );
        }
    }
}
