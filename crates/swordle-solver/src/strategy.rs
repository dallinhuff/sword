use swordle::{Placement, Word, word::WORD_LENGTH};

/// The strategy/rule set to use when solving a wordle puzzle.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Strategy {
    /// The normal/standard rules.
    /// Using this strategy, the solver can solve any canonical wordle within 5 guesses.
    #[default]
    Normal,

    /// Stricter guessing rules where once a placement/letter is known,
    /// subsequent guesses cannot contradict known information.
    /// Using this strategy, the solver can solve any canonical wordle within 6 guesses.
    Hard,
}

impl Strategy {
    const NORMAL_STR: &str = include_str!("../assets/strategy-normal.txt");
    const HARD_STR: &str = ""; // TODO: get the asset file

    const fn as_str(self) -> &'static str {
        match self {
            Strategy::Normal => Self::NORMAL_STR,
            Strategy::Hard => Self::HARD_STR,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Cursor {
    strategy: Strategy,
    row_offset: usize,
    num_guesses: usize,
}

impl Cursor {
    pub fn new(strategy: Strategy) -> Self {
        Cursor {
            strategy,
            row_offset: 0,
            num_guesses: 0,
        }
    }

    /// Suggests the next word to guess according to the strategy.
    #[allow(clippy::missing_panics_doc)]
    pub fn suggest(&self) -> Word {
        self.search_space()
            .map(|l| &l[self.col_offset()..=self.col_offset() + WORD_LENGTH])
            .map(Word::new)
            .find_map(Result::ok)
            .expect("all suggestions are valid words")
    }

    /// Reports the placements/outcome of the last suggestion.
    ///
    /// If the solution was found, consumes the cursor and returns `None`.
    /// If the solution was not found, advances the cursor and returns `Some`
    /// containing the new cursor.
    pub fn report(self, placements: [Placement; WORD_LENGTH]) -> Option<Self> {
        let search_str: String = placements
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

        self.search_space().next()?;

        Some(Cursor {
            strategy: self.strategy,
            num_guesses: self.num_guesses + 1,
            row_offset: self.row_offset + advance_by,
        })
    }

    const fn col_offset(&self) -> usize {
        ((WORD_LENGTH + 1) * 2 + 1) * self.num_guesses
    }

    fn search_space(&self) -> impl std::iter::Iterator<Item = &'static str> {
        self.strategy.as_str().lines().skip(self.row_offset)
    }
}
