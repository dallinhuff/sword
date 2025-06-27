//! [`sword-wordle`](crate) provides the library functionality for Wordle.
//!
//! The types exposed by this crate seek to make invalid states unrepresentable while still
//! allowing some degree of customization of behavior.

pub mod alphabet;
pub mod game;
pub mod guess;
pub mod word;

pub use game::Game;
pub use word::Word;

const WORD_LENGTH: usize = 5;
const MAX_GUESSES: usize = 6;
