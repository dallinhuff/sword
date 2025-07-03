mod outcome;
pub use outcome::GameOutcome;

mod playing;
pub use playing::PlayingGame;

/// The states a wordle game may be in.
#[derive(Clone)]
pub enum Game {
    Playing(PlayingGame),
    Over(GameOutcome),
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
