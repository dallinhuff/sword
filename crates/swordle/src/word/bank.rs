use crate::Word;
use rand::seq::IndexedRandom as _;
use std::sync::LazyLock;

static SOLUTIONS: LazyLock<Box<[Word]>> =
    LazyLock::new(|| str_to_words(include_str!("../../assets/solutions.txt")));
static GUESSES: LazyLock<Box<[Word]>> =
    LazyLock::new(|| str_to_words(include_str!("../../assets/guesses.txt")));

/// Returns whether a given word is a valid solution/guess in the bank/dictionary.
pub fn contains(word: Word) -> bool {
    [&SOLUTIONS, &GUESSES]
        .iter()
        .any(|c| c.binary_search(&word).is_ok())
}

/// Returns a random word from the solutions bank.
pub fn random_solution() -> Word {
    let Some(solution) = SOLUTIONS.choose(&mut rand::rng()).copied() else {
        unreachable!("Bank should always have a solution")
    };

    solution
}

fn str_to_words(s: &str) -> Box<[Word]> {
    let mut words: Box<[Word]> = s
        .lines()
        .filter_map(|w| Word::new_no_dict(w).ok())
        .collect();

    words.sort_unstable();
    words
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn contains_works_for_solution_words() {
        let cases: &[&str] = &["crane", "audio", "skill"];

        for input in cases {
            let word = Word::new_no_dict(input).unwrap();
            assert!(contains(word));
        }
    }

    #[test]
    fn contains_works_for_guess_words() {
        let cases: &[&str] = &["adieu", "adios"];

        for input in cases {
            let word = Word::new_no_dict(input).unwrap();
            assert!(contains(word));
        }
    }

    #[test]
    fn contains_works_for_invalid_words() {
        let cases: &[&str] = &["scxla", "zzyzx"];

        for input in cases {
            let word = Word::new_no_dict(input).unwrap();
            assert!(!contains(word));
        }
    }

    #[test]
    fn random_solution_is_sufficiently_random() {
        let mut previous: HashSet<Word> = HashSet::new();

        for _ in 0..10 {
            let word = random_solution();
            assert!(!previous.contains(&word));
            previous.insert(word);
        }
    }
}
