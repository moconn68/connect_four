use super::GameManager;

/// TODO Manages game logic for local player vs AI games.
#[derive(Clone, Copy, Debug, Default)]
pub struct AiGameManager {}

impl GameManager for AiGameManager {
    fn take_turn(&mut self) {
        todo!()
    }

    fn get_board(&self) -> &crate::board::GameBoard {
        todo!()
    }

    fn check_endgame(&self) -> super::EndgameState {
        todo!()
    }
}
