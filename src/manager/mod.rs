use crate::board::{GameBoard, GamePiece};

/// Versus AI.
pub(crate) mod ai;
/// Versus friend on same computer.
pub(crate) mod local;
/// Online matchmaking.
pub(crate) mod online;

#[derive(Clone, Copy, Debug, Default)]
pub enum GameMode {
    #[default]
    Ai,
    Local,
    Online,
}

impl TryFrom<i8> for GameMode {
    type Error = ();

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Ai),
            2 => Ok(Self::Local),
            3 => Ok(Self::Online),
            _ => Err(()),
        }
    }
}

/// Defines possible end-game states.
pub enum EndgameState {
    /// A given player represented by a [`GamePiece`] has won.
    Win(GamePiece),
    /// The board is full; draw.
    Full,
    /// No end-game state, play continues.
    None,
}

/// Defines top-level APIs for each game client type.
/// There will be one implementation per [`crate::client::GameMode`].
pub trait GameManager {
    /// Gets the current [`EndgameState`] of the game in progress.
    fn check_endgame(&self) -> EndgameState;
    /// Workflow for a playing taking a turn from start to finish.
    fn take_turn(&mut self);
    /// Returns an immutable reference to the current game board state; mainly used for UI.
    fn get_board(&self) -> &GameBoard;
}
