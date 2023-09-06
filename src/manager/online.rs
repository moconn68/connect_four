use super::{GameManager, ManagerError};
use crate::game::state::GameState;

/// TODO Manages game logic for online player vs player games.
#[derive(Clone, Copy, Debug, Default)]
pub struct OnlineGameManager {}

#[allow(unused_variables)]
impl GameManager for OnlineGameManager {
    fn take_turn(&mut self, column_selection: usize) -> Result<(), ManagerError> {
        todo!()
    }

    fn get_state(&self) -> &GameState {
        todo!()
    }
}
