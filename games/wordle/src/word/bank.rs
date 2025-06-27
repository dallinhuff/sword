use super::Word;

const DEFAULT_SOLUTIONS: &str = include_str!("../../assets/solutions.txt");
const DEFAULT_ADDITIONAL_GUESSES: &str = include_str!("../../assets/additional_guesses.txt");

pub(super) struct WordBank {
    pub(super) solutions: Vec<Word>,
    additional_guesses: Vec<Word>,
}

impl WordBank {
    pub(super) fn new() -> Self {
        let mut solutions: Vec<Word> = DEFAULT_SOLUTIONS
            .lines()
            .filter_map(|w| Word::new_no_dict(w).ok())
            .collect();
        let mut additional_guesses: Vec<Word> = DEFAULT_ADDITIONAL_GUESSES
            .lines()
            .filter_map(|w| Word::new_no_dict(w).ok())
            .collect();

        solutions.sort_unstable();
        additional_guesses.sort_unstable();

        Self {
            solutions,
            additional_guesses,
        }
    }

    pub(super) fn contains(&self, word: Word) -> bool {
        [&self.solutions, &self.additional_guesses]
            .iter()
            .any(|ws| ws.binary_search(&word).is_ok())
    }
}
