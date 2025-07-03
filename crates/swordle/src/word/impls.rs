use super::Word;
use super::WordError;

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

    fn try_from(word: &str) -> Result<Self, WordError> {
        Self::new(word)
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
