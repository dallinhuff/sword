use crate::Word;

mod outcome;
pub use outcome::GameOutcome;

mod playing;
pub use playing::PlayingGame;

/// States a wordle game may be in
#[derive(Clone)]
pub enum Game {
    /// The game is currently in progress. The player has not yet guessed the solution,
    /// but may make another guess.
    Playing(PlayingGame),

    /// The game is over, either because the player correctly guessed the solution or
    /// the player ran out of guesses.
    Over(GameOutcome),
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum GameError {
    #[error("Invalid word: {0}")]
    InvalidWord(Word),
}

impl Game {
    #[must_use]
    pub fn new() -> Self {
        Self::Playing(PlayingGame::new())
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
