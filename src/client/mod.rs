pub mod ai;
pub mod local;
pub mod online;

use crate::game::state::GameState;

use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Debug)]
pub enum ClientError {
    InvalidMove(String),
}

impl Display for ClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            ClientError::InvalidMove(m) => write!(f, "Invalid move: {}", m),
        }
    }
}

impl From<ClientError> for String {
    fn from(value: ClientError) -> Self {
        value.to_string()
    }
}

/// Public game-state client APIs to interact with different game-mode servers.
pub trait GameClient {
    /// Yields the current [`GameState`] from the server.
    fn get_current_state(&self) -> GameState;

    /// Handler for user-supplied input move as a column number.
    ///
    /// Returns the updated [`GameState`] upon success, or the corresponding [`ClientError`].
    fn handle_input_move(&mut self, column_num: usize) -> Result<GameState, ClientError>;
}

fn process_move(game_state: &mut GameState, column_num: usize) -> Result<GameState, ClientError> {
    let next_player = *game_state.get_next_player();

    match game_state
        .get_board_mut()
        .insert_piece(next_player, column_num)
    {
        Ok(_) => {
            game_state.toggle_player();
            Ok(*game_state)
        }
        Err(insert_error) => Err(ClientError::InvalidMove(insert_error.to_string())),
    }
}
