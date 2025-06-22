use crate::{Word, guess::Guess};

/// The resulting outcome after
#[derive(Debug, Clone)]
pub struct GameOutcome {
    pub(super) solution: Word,
    pub(super) guesses: Box<[Guess]>,
}

impl GameOutcome {
    #[must_use]
    pub fn won(&self) -> bool {
        self.guesses
            .last()
            .is_some_and(|g| g.word() == &self.solution)
    }

    #[must_use]
    pub fn solution(&self) -> &Word {
        &self.solution
    }

    #[must_use]
    pub fn guesses(&self) -> &[Guess] {
        &self.guesses
    }
}
