use std::fmt::{Display, Formatter, Result as FmtResult};

const SPACE_EMPTY: char = ' ';
const SPACE_RED: char = 'R';
const SPACE_YELLOW: char = 'Y';
const GRID_ROWS: usize = 6;
const GRID_COLS: usize = 7;

// Errors

/// Occurs when a piece is attempted to be inserted.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InsertError {
    InvalidColumn,
    FullColumn,
}

impl Display for InsertError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            InsertError::InvalidColumn => write!(f, "Please choose a valid column number [1,7]."),
            InsertError::FullColumn => {
                write!(f, "Selected column is full, please choose another column.")
            }
        }
    }
}

/// Colored game peices, one per player.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GamePiece {
    Red,
    Yellow,
}

impl Display for GamePiece {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Red => write!(f, "\x1b[91m{}\x1b[0m", SPACE_RED),
            Self::Yellow => write!(f, "\x1b[93m{}\x1b[0m", SPACE_YELLOW),
        }
    }
}

/// Possible states for a game "board" space.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
enum BoardSpace {
    #[default]
    Empty,
    Piece(GamePiece),
}

impl Display for BoardSpace {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            BoardSpace::Empty => write!(f, "{}", SPACE_EMPTY),
            BoardSpace::Piece(p) => write!(f, "{}", p),
        }
    }
}

type BoardColumns = [BoardSpace; GRID_COLS];
type GameGrid = [BoardColumns; GRID_ROWS];

/// Manages the state of the game board.
#[derive(Clone, Copy, Debug, Default)]
pub struct GameBoard {
    grid: GameGrid,
}

impl GameBoard {
    /// Insert a new piece into the game board in a given column number.
    /// Returns an [`InsertError`] if the column number is invalid, or if the column is already full.
    pub fn insert_piece(&mut self, piece: GamePiece, col_num: usize) -> Result<(), InsertError> {
        // Validate input column number
        if !(1..=GRID_COLS).contains(&col_num) {
            return Err(InsertError::InvalidColumn);
        }

        for row in self.grid.iter_mut().rev() {
            if let BoardSpace::Empty = row[col_num - 1] {
                row[col_num - 1] = BoardSpace::Piece(piece);
                return Ok(());
            }
        }
        // No spaces in given column left to put piece into
        Err(InsertError::FullColumn)
    }
}

impl Display for GameBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        // Top of board should display the column number
        let board_header = (1..=GRID_COLS)
            .map(|num| format!("+-{}-", num))
            .collect::<Vec<String>>()
            .join("");
        writeln!(f, "{}+", board_header)?;

        let row_separator = "+---".repeat(GRID_COLS) + "+\n";
        for row in &self.grid {
            for space in row {
                write!(f, "| {} ", space)?;
            }
            write!(f, "|\n{}", row_separator)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn verify_grid_display() {
        let empty_board = r#"+-1-+-2-+-3-+-4-+-5-+-6-+-7-+
|   |   |   |   |   |   |   |
+---+---+---+---+---+---+---+
|   |   |   |   |   |   |   |
+---+---+---+---+---+---+---+
|   |   |   |   |   |   |   |
+---+---+---+---+---+---+---+
|   |   |   |   |   |   |   |
+---+---+---+---+---+---+---+
|   |   |   |   |   |   |   |
+---+---+---+---+---+---+---+
|   |   |   |   |   |   |   |
+---+---+---+---+---+---+---+
"#;
        assert_eq!(empty_board, GameBoard::default().to_string());
    }

    #[test]
    fn insert_piece_valid() {
        let mut board = GameBoard::default();
        board.insert_piece(GamePiece::Red, 1).unwrap();
        board.insert_piece(GamePiece::Yellow, 1).unwrap();
        board.insert_piece(GamePiece::Red, 1).unwrap();
        board.insert_piece(GamePiece::Yellow, 7).unwrap();

        let expected_col_1 = [
            BoardSpace::Empty,
            BoardSpace::Empty,
            BoardSpace::Empty,
            BoardSpace::Piece(GamePiece::Red),
            BoardSpace::Piece(GamePiece::Yellow),
            BoardSpace::Piece(GamePiece::Red),
        ];
        let actual_col_1: Vec<BoardSpace> = board.grid.iter().map(|row| row[0]).collect();
        assert_eq!(expected_col_1, actual_col_1.as_slice());

        let expected_col_7 = [
            BoardSpace::Empty,
            BoardSpace::Empty,
            BoardSpace::Empty,
            BoardSpace::Empty,
            BoardSpace::Empty,
            BoardSpace::Piece(GamePiece::Yellow),
        ];
        let actual_col_7: Vec<BoardSpace> = board.grid.iter().map(|row| row[6]).collect();
        assert_eq!(expected_col_7, actual_col_7.as_slice());
    }

    #[test]
    fn insert_piece_full_column() {
        let col = 1;
        let mut board = GameBoard::default();
        for _ in 0..GRID_ROWS {
            board.insert_piece(GamePiece::Red, col).unwrap();
        }
        let bad_insert = board.insert_piece(GamePiece::Red, col);
        assert!(bad_insert.is_err());
    }

    #[test]
    fn insert_piece_invalid_column() {
        let mut board = GameBoard::default();
        let bad_insert = board.insert_piece(GamePiece::Red, 8);
        assert!(bad_insert.is_err());
    }
}
