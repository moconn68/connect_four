use super::{ClientError, GameClient};
use crate::game::state::GameState;

#[derive(Clone, Copy, Debug, Default)]
pub struct LocalGameClient {
    game_state: GameState,
}

impl GameClient for LocalGameClient {
    fn handle_input_move(&mut self, column_num: usize) -> Result<GameState, ClientError> {
        super::process_move(&mut self.game_state, column_num)
    }

    fn get_current_state(&self) -> GameState {
        self.game_state
    }
}
