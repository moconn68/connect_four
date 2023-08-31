use super::GameManager;
use crate::board::{GameBoard, GamePiece};

/// Manages game logic for local player vs player games.
#[derive(Clone, Copy, Debug, Default)]
pub struct LocalGameManager {
    board: GameBoard,
    next_player: GamePiece,
}

impl GameManager for LocalGameManager {
    fn take_turn(&mut self, column_selection: usize) -> Result<(), super::ManagerError> {
        self.board
            .insert_piece(self.next_player, column_selection)
            .map_err(|e| super::ManagerError::TakeTurn(format!("Invalid move: {e}")))?;

        // Update the next player
        self.next_player = match self.next_player {
            GamePiece::Red => GamePiece::Yellow,
            GamePiece::Yellow => GamePiece::Red,
        };

        Ok(())
    }

    fn check_endgame(&self) -> super::EndgameState {
        if let Some(p) = self.board.is_winner() {
            super::EndgameState::Win(p)
        } else if self.board.is_full() {
            super::EndgameState::Full
        } else {
            super::EndgameState::None
        }
    }

    fn get_board(&self) -> &GameBoard {
        &self.board
    }

    fn get_next_player(&self) -> &GamePiece {
        &self.next_player
    }
}
