use crate::WORD_LENGTH;

/// A sequence of letters that may be submitted as a guess for a game.
///
/// `Word` ensures that the sequence of letters it contains are syntactically & semantically valid,
/// but it does not make any guarantees that the word exists in a particular
/// [`Dictionary`](crate::dict::dictionary).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Word([u8; WORD_LENGTH]);

/// Errors that may occur when constructing a [`Word`].
#[derive(Debug, Clone, thiserror::Error)]
pub enum WordError {
    #[error("Word must be exactly {WORD_LENGTH} letters")]
    InvalidLen,
    #[error("Word must only contain ascii letters")]
    InvalidLetters,
}

impl Word {
    /// Attempts to create a new word from a string, trimming whitespace and performing case
    /// conversion.
    ///
    /// # Errors
    ///
    /// Returns a [`WordError::InvalidLen`] if the string is not the correct length.
    ///
    /// Returns a [`WordError::InvalidLetters`] if the string contains a non-ascii letter.
    pub fn new(word: &str) -> Result<Self, WordError> {
        let bytes = word.trim().as_bytes();

        if bytes.len() != WORD_LENGTH {
            return Err(WordError::InvalidLen);
        }

        if bytes.iter().any(|b| !b.is_ascii_alphabetic()) {
            return Err(WordError::InvalidLetters);
        }

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

impl std::fmt::Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::hash::Hash for Word {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state);
    }
}

impl std::ops::Index<usize> for Word {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl TryFrom<&str> for Word {
    type Error = WordError;

    fn try_from(word: &str) -> Result<Self, Self::Error> {
        Self::new(word)
    }
}

impl TryFrom<String> for Word {
    type Error = WordError;

    fn try_from(word: String) -> Result<Self, Self::Error> {
        Self::new(&word)
    }
}

impl From<Word> for String {
    fn from(word: Word) -> Self {
        word.as_str().to_owned()
    }
}

impl AsRef<str> for Word {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<[u8]> for Word {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}
