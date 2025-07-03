use crate::guess::Guess;
use crate::word::Word;

#[derive(Debug, Clone)]
pub struct GameOutcome {
    solution: Word,
    guesses: Box<[Guess]>,
}

impl GameOutcome {
    #[must_use]
    pub fn new(solution: Word, guesses: Box<[Guess]>) -> Self {
        GameOutcome { solution, guesses }
    }

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
