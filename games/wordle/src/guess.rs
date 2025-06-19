use crate::WORD_LENGTH;
use crate::word::Word;
use colored::Colorize as _;

/// An indicator of whether a letter in a guess is in the word.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Placement {
    /// The letter is not in the solution.
    Incorrect = 0,

    /// The letter is in the solution, but not in the given spot.
    Misplaced = 1,

    /// The letter is in the solution in the given spot.
    Correct = 2,
}

/// A guess that was submitted to a game.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Guess {
    /// The word that was guessed.
    word: Word,

    /// The result/outcome for how the letters are placed in the guess.
    result: [Placement; WORD_LENGTH],
}

impl Guess {
    /// Creates a guess from the solution and a guessed word.
    #[must_use]
    pub fn new(solution: &Word, guess: Word) -> Self {
        if solution == &guess {
            return Guess {
                word: guess,
                result: [Placement::Correct; WORD_LENGTH],
            };
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
            // TODO: replace this with an epic let-chain when it hits stable rust
            let Some(count) = sol_counts.get_mut(&guess[i]) else {
                continue;
            };

            if *count > 0 && *placement != Placement::Correct {
                *placement = Placement::Misplaced;
                *count -= 1;
            }
        }

        Guess {
            word: guess,
            result: placements,
        }
    }

    #[must_use]
    pub const fn word(&self) -> &Word {
        &self.word
    }

    #[must_use]
    pub const fn result(&self) -> &[Placement] {
        &self.result
    }

    #[must_use]
    pub fn is_correct(&self) -> bool {
        self.result == [Placement::Correct; WORD_LENGTH]
    }
}

impl std::fmt::Display for Guess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, placement) in self.result.iter().enumerate() {
            let letter = self.word.as_str()[i..=i].bold();
            match placement {
                Placement::Incorrect => write!(f, "{}", letter.white().dimmed())?,
                Placement::Misplaced => write!(f, "{}", letter.yellow())?,
                Placement::Correct => write!(f, "{}", letter.green())?,
            }
        }

        Ok(())
    }
}
