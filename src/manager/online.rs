use super::GameManager;

/// TODO Manages game logic for online player vs player games.
#[derive(Clone, Copy, Debug, Default)]
pub struct OnlineGameManager {}

impl GameManager for OnlineGameManager {
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
