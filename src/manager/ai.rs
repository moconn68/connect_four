use super::{GameManager, ManagerError};
use crate::game::state::GameState;

/// TODO Manages game logic for local player vs AI games.
#[derive(Clone, Copy, Debug, Default)]
pub struct AiGameManager {}

#[allow(unused_variables)]
impl GameManager for AiGameManager {
    fn take_turn(&mut self, column_selection: usize) -> Result<(), ManagerError> {
        todo!()
    }

    fn get_state(&self) -> &GameState {
        todo!()
    }
}
