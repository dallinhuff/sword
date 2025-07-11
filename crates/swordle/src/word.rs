mod bank;
mod impls;

#[cfg(test)]
mod tests;

pub const WORD_LENGTH: usize = 5;

/// A sequence of letters that may be submitted as a guess for a game.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Word([u8; WORD_LENGTH]);

impl Word {
    /// Attempts to create a new word from a string, trimming whitespace and performing case
    /// conversion.
    ///
    /// # Errors
    ///
    /// Returns a [`WordError`] when a syntactically and semantically valid `Word` cannot be parsed
    /// from the provided string.
    /// ```
    pub fn new(word: &str) -> Result<Self, WordError> {
        let word = Self::new_no_dict(word)?;
        if bank::contains(word) {
            Ok(word)
        } else {
            Err(WordError::Bank)
        }
    }

    /// Generates a random word (for use as a solution in a game).
    pub fn random() -> Self {
        bank::random_solution()
    }

    fn new_no_dict(word: &str) -> Result<Self, WordError> {
        let bytes = word.trim().as_bytes();
        validate_word_bytes(bytes)?;

        let mut arr = [0u8; WORD_LENGTH];
        arr.copy_from_slice(bytes);
        arr.make_ascii_lowercase();

        Ok(Word(arr))
    }

    #[must_use]
    pub const fn as_str(&self) -> &str {
        // Safety: safe because we validate ASCII at construction
        unsafe { std::str::from_utf8_unchecked(&self.0) }
    }

    #[must_use]
    pub const fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn chars(&self) -> impl Iterator<Item = char> {
        self.0.iter().map(|&b| b as char)
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        WORD_LENGTH
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        false
    }
}

fn validate_word_bytes(bytes: &[u8]) -> Result<(), WordError> {
    // Could just do the length check first, but validating that the first few bytes are all
    // ascii letters is usually a better user experience because it will report an invalid
    // letter error instead of an invalid length error for 5-letter words with multi-byte
    // graphemes in them.
    if bytes
        .iter()
        .take(WORD_LENGTH)
        .any(|b| !b.is_ascii_alphabetic())
    {
        return Err(WordError::Letter);
    }

    if bytes.len() != WORD_LENGTH {
        return Err(WordError::Length);
    }

    Ok(())
}

/// Errors that may occur when constructing a [`Word`].
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum WordError {
    /// Occurs when attempting to construct a `Word` of an incorrect length.
    #[error("Word must be exactly {WORD_LENGTH} letters")]
    Length,

    /// Occurs when attempting to construct a `Word` from a string that contains non-acii and/or
    /// non-alphabetic characters.
    #[error("Word must only contain ascii letters")]
    Letter,

    /// Occurs when attempting to construct a `Word` that is syntactically valid, but is not a
    /// real/recognized English word.
    #[error("Word must be a valid english word in the word bank")]
    Bank,
}
