use crate::placement::Placement;
use crate::word::{WORD_LENGTH, Word};
use colored::Colorize as _;
use std::fmt::Display;

const CORRECT_PLACEMENTS: &[Placement; WORD_LENGTH] = &[Placement::Correct; WORD_LENGTH];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Guess {
    Correct(Word),
    Incorrect(Word, [Placement; WORD_LENGTH]),
}

impl Guess {
    #[must_use]
    pub fn new(solution: &Word, guess: Word) -> Self {
        if solution == &guess {
            return Guess::Correct(guess);
        }

        let mut placements = [Placement::Incorrect; WORD_LENGTH];
        let mut sol_counts = std::collections::HashMap::with_capacity(WORD_LENGTH);

        for (i, placement) in placements.iter_mut().enumerate() {
            if solution[i] == guess[i] {
                *placement = Placement::Correct;
            } else {
                *sol_counts.entry(solution[i]).or_insert(0) += 1;
            }
        }

        for (i, placement) in placements.iter_mut().enumerate() {
            if let Some(count) = sol_counts.get_mut(&guess[i])
                && *count > 0
                && *placement != Placement::Correct
            {
                *placement = Placement::Misplaced;
                *count -= 1;
            }
        }

        Guess::Incorrect(guess, placements)
    }

    #[must_use]
    pub const fn is_correct(&self) -> bool {
        matches!(self, Guess::Correct(_))
    }

    #[must_use]
    pub const fn word(&self) -> &Word {
        match self {
            Guess::Correct(w) | Guess::Incorrect(w, _) => w,
        }
    }

    #[must_use]
    pub const fn placements(&self) -> &[Placement] {
        match self {
            Guess::Correct(_) => CORRECT_PLACEMENTS,
            Guess::Incorrect(_, g) => g,
        }
    }
}

impl Display for Guess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.placements()
            .iter()
            .enumerate()
            .map(|(i, placement)| {
                let letter = self.word().as_str()[i..=i].bold();
                match placement {
                    Placement::Incorrect => letter.white().dimmed(),
                    Placement::Misplaced => letter.yellow(),
                    Placement::Correct => letter.green(),
                }
            })
            .try_for_each(|p| write!(f, "{p}"))
    }
}
