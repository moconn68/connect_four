use super::GameManager;
use crate::board::{GameBoard, GamePiece};
use crate::tui;

/// Manages game logic for local player vs player games.
#[derive(Clone, Copy, Debug, Default)]
pub struct LocalGameManager {
    board: GameBoard,
    next_player: GamePiece,
}

impl GameManager for LocalGameManager {
    fn take_turn(&mut self) {
        println!(
            "It is {:?}'s turn - please select a column to place your piece.",
            self.next_player
        );

        loop {
            let col_num = tui::get_column_selection();
            if let Err(e) = self.board.insert_piece(self.next_player, col_num) {
                println!("Invalid move: {e}. Please select a different column:");
            } else {
                break;
            }
        }

        // Update the next player
        self.next_player = match self.next_player {
            GamePiece::Red => GamePiece::Yellow,
            GamePiece::Yellow => GamePiece::Red,
        }
    }

    fn get_board(&self) -> &GameBoard {
        &self.board
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
}
