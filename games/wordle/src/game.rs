use crate::MAX_GUESSES;
use crate::{dict::Dictionary, guess::Guess, word::Word};

/// The possible states a Wordle game may be in.
pub enum Game<'a> {
    Active(ActiveGame<'a>),
    Outcome(GameOutcome),
}

/// A game that is currently active/in-progress.
#[derive(Clone)]
pub struct ActiveGame<'d> {
    dictionary: &'d dyn Dictionary,
    solution: Word,
    guesses: Vec<Guess>,
}

impl<'a> ActiveGame<'a> {
    /// Creates a new game using a borrowed dictionary.
    #[must_use]
    pub fn new(dictionary: &'a dyn Dictionary) -> Self {
        ActiveGame {
            dictionary,
            solution: dictionary.random_solution(),
            guesses: Vec::with_capacity(MAX_GUESSES),
        }
    }

    /// Makes a guess and returns the result.
    #[must_use]
    pub fn guess(mut self, guess: Word) -> Game<'a> {
        if self.dictionary.contains(&guess) {
            self.guesses.push(Guess::new(&self.solution, guess));
        }

        if self.guesses.last().is_some_and(Guess::is_correct) || self.guesses.len() >= MAX_GUESSES {
            Game::Outcome(GameOutcome {
                solution: self.solution,
                guesses: self.guesses.into_boxed_slice(),
            })
        } else {
            Game::Active(self)
        }
    }

    #[must_use]
    pub fn guesses(&self) -> &[Guess] {
        &self.guesses
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameOutcome {
    pub solution: Word,
    pub guesses: Box<[Guess]>,
}

impl GameOutcome {
    #[must_use]
    pub fn won(&self) -> bool {
        self.guesses.last().is_some_and(Guess::is_correct)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const WORDS: [&str; 7] = [
        "audio", "crane", "crabs", "crash", "candy", "skill", "scale",
    ];
    const SOLUTION: &str = WORDS[6];

    struct FakeDict;
    impl Dictionary for FakeDict {
        fn contains(&self, word: &Word) -> bool {
            WORDS.iter().any(|&w| w == word.as_str())
        }

        fn random_solution(&self) -> Word {
            Word::new(SOLUTION).unwrap()
        }
    }

    #[test]
    fn it_results_in_win_when_correct_guess_made() {
        let guess = Word::new(SOLUTION).unwrap();
        let Game::Outcome(outcome) = ActiveGame::new(&FakeDict).guess(guess) else {
            panic!("Should result in outcome when solution is guessed")
        };

        assert!(outcome.won(), "Should win after guessing solution");
    }

    #[test]
    fn it_results_in_loss_when_run_out_of_guesses() {
        let mut game = ActiveGame::new(&FakeDict);

        for guess in &WORDS[..5] {
            let Game::Active(g) = game.guess(Word::new(guess).unwrap()) else {
                panic!("Should result in active game when still guesses left");
            };

            game = g;
        }

        let Game::Outcome(outcome) = game.guess(Word::new(WORDS[5]).unwrap()) else {
            panic!("Should result in outcome when no guesses left");
        };

        println!("{}", outcome.guesses.last().unwrap());

        assert!(
            !outcome.won(),
            "Should not win after running out of guesses"
        );
    }
}
