//! `swordle` is an implementation of the game logic for the classic NYT Wordle game.

pub mod game;
pub mod guess;
pub mod letter_bank;
pub mod placement;
pub mod word;

pub use game::Game;
pub use game::GameOutcome;
pub use game::PlayingGame;
pub use guess::Guess;
pub use placement::Placement;
pub use word::Word;
