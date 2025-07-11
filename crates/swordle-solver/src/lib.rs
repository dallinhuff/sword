use swordle::{Placement, Word, word::WORD_LENGTH};

const STRATEGY: &str = include_str!("../assets/strategy-normal.txt");

#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct Solver {
    row_offset: usize,
    num_guesses: usize,
}

impl Solver {
    pub fn new() -> Self {
        Solver {
            row_offset: 0,
            num_guesses: 0,
        }
    }

    const fn col_offset(&self) -> usize {
        ((WORD_LENGTH + 1) * 2 + 1) * self.num_guesses
    }

    fn search_space(&self) -> impl std::iter::Iterator<Item = &'static str> {
        STRATEGY.lines().skip(self.row_offset)
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn suggest(&self) -> Word {
        self.search_space()
            .map(|l| &l[self.col_offset()..=self.col_offset() + WORD_LENGTH])
            .map(Word::new)
            .find_map(Result::ok)
            .expect("all lines of `STRATEGY` are valid words")
    }

    pub fn report(self, outcome: [Placement; WORD_LENGTH]) -> Self {
        let search_str: String = outcome
            .iter()
            .map(|p| match p {
                Placement::Incorrect => 'B',
                Placement::Misplaced => 'Y',
                Placement::Correct => 'G',
            })
            .chain((self.num_guesses + 1).to_string().chars().take(1))
            .collect();

        let advance_by = self
            .search_space()
            .enumerate()
            .find(|(_, l)| l.contains(&search_str))
            .map_or(0, |(i, _)| i);

        Solver {
            num_guesses: self.num_guesses + 1,
            row_offset: self.row_offset + advance_by,
        }
    }
}

impl Default for Solver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suggest_works() {
        let solver = Solver::new();
        let word = solver.suggest();
        assert_eq!(word.as_str(), "salet");

        let solver = solver.report([Placement::Incorrect; 5]);
        let word = solver.suggest();
        assert_eq!(word.as_str(), "courd");

        let solver = solver.report([
            Placement::Incorrect,
            Placement::Incorrect,
            Placement::Misplaced,
            Placement::Incorrect,
            Placement::Incorrect,
        ]);
        let word = solver.suggest();
        assert_eq!(word.as_str(), "gimpy");

        let solver = solver.report([
            Placement::Incorrect,
            Placement::Incorrect,
            Placement::Incorrect,
            Placement::Incorrect,
            Placement::Correct,
        ]);
        let word = solver.suggest();
        assert_eq!(word.as_str(), "funky");

        let solver = solver.report([
            Placement::Incorrect,
            Placement::Correct,
            Placement::Correct,
            Placement::Correct,
            Placement::Correct,
        ]);
        let word = solver.suggest();
        assert_eq!(word.as_str(), "hunky");

        // TODO: figure out a good API so that once the solution is found you can't make more
        // guesses.
        // As is, this solver would panic if another guess was attempted.
    }
}
