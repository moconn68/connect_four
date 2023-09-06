use super::{GameManager, ManagerError};
use crate::game::state::GameState;

/// Manages game logic for local player vs player games.
#[derive(Clone, Copy, Debug, Default)]
pub struct LocalGameManager {
    state: GameState,
}

impl GameManager for LocalGameManager {
    fn take_turn(&mut self, column_selection: usize) -> Result<(), ManagerError> {
        let piece = *self.state.get_next_player();
        self.state
            .get_board_mut()
            .insert_piece(piece, column_selection)
            .map_err(|e| ManagerError::TakeTurn(format!("Invalid move: {e}")))?;

        self.state.toggle_player();

        Ok(())
    }

    fn get_state(&self) -> &GameState {
        &self.state
    }
}
