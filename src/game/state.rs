use crate::game::board::{GameBoard, GamePiece};

/// Defines possible end-game states.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EndgameType {
    /// A given player represented by a [`GamePiece`] has won.
    Win(GamePiece),
    /// The board is full; draw.
    Full,
    /// No end-game state, play continues.
    None,
}

#[derive(Clone, Copy, Debug)]
pub struct GameState {
    board: GameBoard,
    next_player: GamePiece,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            board: Default::default(),
            next_player: GamePiece::random(),
        }
    }
}

impl GameState {
    pub fn get_board(&self) -> &GameBoard {
        &self.board
    }

    pub(crate) fn get_board_mut(&mut self) -> &mut GameBoard {
        &mut self.board
    }

    pub fn get_next_player(&self) -> &GamePiece {
        &self.next_player
    }

    pub fn toggle_player(&mut self) {
        self.next_player = match self.next_player {
            GamePiece::Red => GamePiece::Yellow,
            GamePiece::Yellow => GamePiece::Red,
        }
    }

    pub fn check_endgame(&self) -> EndgameType {
        if let Some(p) = self.board.is_winner() {
            EndgameType::Win(p)
        } else if self.board.is_full() {
            EndgameType::Full
        } else {
            EndgameType::None
        }
    }

    #[cfg(test)]
    fn new(board: GameBoard) -> Self {
        Self {
            board,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{EndgameType, GameState};
    use crate::game::board::{GameBoard, GamePiece};

    #[test]
    fn toggle_player() {
        let mut state = GameState::default();
        let initial_player = *state.get_next_player();

        state.toggle_player();
        assert_ne!(state.get_next_player(), &initial_player);

        state.toggle_player();
        assert_eq!(state.get_next_player(), &initial_player);
    }

    #[test]
    fn check_endgame_winner() {
        let mut board = GameBoard::default();
        let piece = GamePiece::Red;
        for _ in 0..4 {
            board.insert_piece(piece, 1).unwrap();
        }

        let state = GameState::new(board);
        assert_eq!(EndgameType::Win(piece), state.check_endgame());
    }

    #[test]
    fn check_endgame_full() {
        let mut board = GameBoard::default();

        // Fill board in a tie (full) state
        for col in 1..=crate::game::board::GRID_COLS {
            let mut flip = match col {
                4 => true,
                _ => false,
            };
            for _ in 0..crate::game::board::GRID_ROWS {
                let piece = match flip {
                    true => GamePiece::Red,
                    false => GamePiece::Yellow,
                };
                board.insert_piece(piece, col).unwrap();
                flip = !flip;
            }
        }

        let state = GameState::new(board);
        assert_eq!(EndgameType::Full, state.check_endgame());
    }

    #[test]
    fn check_endgame_none() {
        let state = GameState::default();
        assert_eq!(state.check_endgame(), EndgameType::None);
    }
}
