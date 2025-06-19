use std::fmt::Display;

use colored::Colorize as _;

use crate::guess::{Guess, Placement};

#[derive(Debug, Clone)]
pub struct Alphabet([(u8, Option<Placement>); 26]);

impl Alphabet {
    #[must_use]
    pub fn new() -> Self {
        let mut alphabet = [(0u8, None); 26];

        for (i, letter) in (b'a'..=b'z').enumerate() {
            alphabet[i].0 = letter;
        }

        Self(alphabet)
    }

    #[must_use]
    pub const fn letters(&self) -> &[(u8, Option<Placement>)] {
        &self.0
    }

    pub fn report_guess(&mut self, guess: &Guess) {
        for (letter, placement) in guess.word().as_bytes().iter().zip(guess.result().iter()) {
            let index = (letter - b'a') as usize;
            if let (_, Some(p)) = self.0[index] {
                self.0[index].1 = Some(p.max(*placement));
            } else {
                self.0[index].1 = Some(*placement);
            }
        }
    }
}

impl Default for Alphabet {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Alphabet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (letter, placement) in self.0 {
            let letter = &[letter];
            let letter = unsafe { str::from_utf8_unchecked(letter) };
            let letter = match placement {
                Some(Placement::Correct) => letter.bold().green(),
                Some(Placement::Misplaced) => letter.bold().yellow(),
                Some(Placement::Incorrect) => letter.bold().dimmed().white(),
                None => letter.bold().white(),
            };

            write!(f, "{letter}")?;
        }

        Ok(())
    }
}
