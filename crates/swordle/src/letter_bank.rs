use crate::{Guess, Placement};
use colored::Colorize as _;
use std::fmt::Display;

const BANK_LENGTH: usize = 26;

/// A bank of letters that may be in the solution
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LetterBank([Option<Placement>; BANK_LENGTH]);

impl LetterBank {
    #[must_use]
    pub fn new() -> Self {
        LetterBank([None; BANK_LENGTH])
    }

    #[must_use]
    pub fn get(&self, letter: &u8) -> Option<Placement> {
        self.0[byte_to_index(*letter)]
    }

    pub fn report_guess(&mut self, guess: &Guess) {
        match guess {
            Guess::Correct(word) => {
                word.as_bytes()
                    .iter()
                    .copied()
                    .map(byte_to_index)
                    .for_each(|i| self.0[i] = Some(Placement::Correct));

                self.0
                    .iter_mut()
                    .filter(|v| v.is_none())
                    .for_each(|v| *v = Some(Placement::Incorrect));
            }
            Guess::Incorrect(word, placements) => {
                word.as_bytes()
                    .iter()
                    .copied()
                    .map(byte_to_index)
                    .zip(placements.iter().copied())
                    .for_each(|(index, placement)| {
                        self.0[index] = self.0[index]
                            .map(|existing| existing.max(placement))
                            .or(Some(placement));
                    });
            }
        }
    }
}

impl Default for LetterBank {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for LetterBank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let letters = self
            .0
            .iter()
            .enumerate()
            .map(|(i, p)| (index_to_byte(i), p));

        for (letter, placement) in letters {
            let letter_arr = [letter];
            let letter = String::from_utf8_lossy(&letter_arr);
            write!(
                f,
                "{}",
                match placement {
                    Some(Placement::Correct) => letter.bold().green(),
                    Some(Placement::Misplaced) => letter.bold().yellow(),
                    Some(Placement::Incorrect) => letter.white().dimmed(),
                    None => letter.bold().white(),
                }
            )?;
        }

        Ok(())
    }
}

fn byte_to_index(byte: u8) -> usize {
    if byte.is_ascii_lowercase() {
        (byte - b'a') as usize
    } else {
        panic!("called byte_to_index with a non-letter");
    }
}

#[allow(clippy::cast_possible_truncation)]
fn index_to_byte(index: usize) -> u8 {
    if index < BANK_LENGTH {
        index as u8 + b'a'
    } else {
        panic!("called index_to_byte with out-of-range index");
    }
}

#[cfg(test)]
mod tests {
    use crate::Word;

    use super::*;

    #[test]
    fn report_guess_works_for_correct_guess() {
        let mut letter_bank = LetterBank::new();
        let word = Word::new("crane").unwrap();
        let guess = Guess::Correct(word);

        letter_bank.report_guess(&guess);

        for letter in word.as_bytes() {
            assert_eq!(letter_bank.get(letter), Some(Placement::Correct));
        }

        for letter in b'a'..=b'z' {
            if !"crane".as_bytes().contains(&letter) {
                assert_eq!(letter_bank.get(&letter), Some(Placement::Incorrect));
            }
        }
    }

    #[test]
    fn report_guess_works_for_incorrect_guesses() {
        let mut letter_bank = LetterBank::new();
        let solution = Word::new("famed").unwrap();

        let guess = Guess::new(&solution, Word::new("frame").unwrap());
        letter_bank.report_guess(&guess);

        assert_eq!(letter_bank.get(&b'f'), Some(Placement::Correct));
        assert_eq!(letter_bank.get(&b'r'), Some(Placement::Incorrect));
        assert_eq!(letter_bank.get(&b'a'), Some(Placement::Misplaced));
        assert_eq!(letter_bank.get(&b'm'), Some(Placement::Misplaced));
        assert_eq!(letter_bank.get(&b'e'), Some(Placement::Misplaced));

        let guess = Guess::new(&solution, Word::new("fames").unwrap());
        letter_bank.report_guess(&guess);

        assert_eq!(letter_bank.get(&b'f'), Some(Placement::Correct));
        assert_eq!(letter_bank.get(&b'a'), Some(Placement::Correct));
        assert_eq!(letter_bank.get(&b'm'), Some(Placement::Correct));
        assert_eq!(letter_bank.get(&b'e'), Some(Placement::Correct));
        assert_eq!(letter_bank.get(&b's'), Some(Placement::Incorrect));
    }
}
