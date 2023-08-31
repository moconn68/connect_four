use super::GameManager;

/// TODO Manages game logic for local player vs AI games.
#[derive(Clone, Copy, Debug, Default)]
pub struct AiGameManager {}

#[allow(unused_variables)]
impl GameManager for AiGameManager {
    fn take_turn(&mut self, column_selection: usize) -> Result<(), super::ManagerError> {
        todo!()
    }

    fn check_endgame(&self) -> super::EndgameState {
        todo!()
    }

    fn get_board(&self) -> &crate::board::GameBoard {
        todo!()
    }

    fn get_next_player(&self) -> &crate::board::GamePiece {
        todo!()
    }
}
