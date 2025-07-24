use swordle::{Guess, Word};

mod strategy;
pub use strategy::Strategy;

#[must_use]
#[derive(Debug, Clone)]
pub enum SolverResult {
    Solving(Solver),
    Solved {
        solution: Word,
        guesses: Box<[Guess]>,
    },
}

#[must_use]
#[derive(Debug, Clone)]
pub struct Solver {
    cursor: strategy::Cursor,
    guesses: Vec<Guess>,
}

impl Solver {
    pub fn new(strategy: Strategy) -> Self {
        Solver {
            cursor: strategy::Cursor::new(strategy),
            guesses: Vec::with_capacity(5),
        }
    }

    pub fn suggest(&self) -> Word {
        self.cursor.suggest()
    }

    pub fn report(mut self, guess: Guess) -> SolverResult {
        match guess {
            Guess::Correct(word) => {
                self.guesses.push(guess);
                SolverResult::Solved {
                    solution: word,
                    guesses: self.guesses.into_boxed_slice(),
                }
            }
            Guess::Incorrect(word, placements) => {
                let cursor = self.cursor.report(placements);
                self.guesses.push(guess);
                if let Some(cursor) = cursor {
                    self.cursor = cursor;
                    SolverResult::Solving(self)
                } else {
                    SolverResult::Solved {
                        solution: word,
                        guesses: self.guesses.into_boxed_slice(),
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suggest_works() {
        let mut solver = Solver::new(Strategy::Normal);

        let suggestions = ["salet", "courd", "gimpy", "funky"];
        let solution = Word::new("hunky").unwrap();

        for suggestion in suggestions {
            let word = solver.suggest();
            assert_eq!(word.as_str(), suggestion);

            let guess = Guess::new(&solution, word);
            let SolverResult::Solving(s) = solver.report(guess) else {
                panic!();
            };

            solver = s;
        }

        assert_eq!(solution, solver.suggest());

        let guess = Guess::Correct(solution);
        let SolverResult::Solved { solution: s, .. } = solver.report(guess) else {
            panic!();
        };

        assert_eq!(solution, s);
    }
}
