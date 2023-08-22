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

/// Used when checking if a [`BoardSpace`] contains some [`GamePiece`].
impl PartialEq<BoardSpace> for GamePiece {
    fn eq(&self, other: &BoardSpace) -> bool {
        match other {
            BoardSpace::Empty => false,
            BoardSpace::Piece(p) => self.eq(p),
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

    /// Checks if there is a winner in the current game state.
    /// Checks for four like pieces in a row horizontally, vertically, and diagonally.
    ///
    /// Returns an [`Option`] containing the [`GamePiece`] of the winning player, or [`None`] if there is no winner.
    // TODO remove cfg(test) when game loop is implemented
    #[cfg(test)]
    fn is_winner(&self) -> Option<GamePiece> {
        // Horizontal
        for row in 0..GRID_ROWS {
            for col in 0..GRID_COLS - 3 {
                match self.grid[row][col] {
                    BoardSpace::Empty => (),
                    BoardSpace::Piece(piece) => {
                        if piece == self.grid[row][col + 1]
                            && piece == self.grid[row][col + 2]
                            && piece == self.grid[row][col + 3]
                        {
                            return Some(piece);
                        }
                    }
                }
            }
        }

        // Vertical
        for row in 0..GRID_ROWS - 3 {
            for col in 0..GRID_COLS {
                match self.grid[row][col] {
                    BoardSpace::Empty => (),
                    BoardSpace::Piece(piece) => {
                        if piece == self.grid[row + 1][col]
                            && piece == self.grid[row + 2][col]
                            && piece == self.grid[row + 3][col]
                        {
                            return Some(piece);
                        }
                    }
                }
            }
        }

        // Diagonal up
        for row in 0..GRID_ROWS - 3 {
            for col in 3..GRID_COLS {
                match self.grid[row][col] {
                    BoardSpace::Empty => (),
                    BoardSpace::Piece(piece) => {
                        if piece == self.grid[row + 1][col - 1]
                            && piece == self.grid[row + 2][col - 2]
                            && piece == self.grid[row + 3][col - 3]
                        {
                            return Some(piece);
                        }
                    }
                }
            }
        }

        // Diagonal down
        for row in 3..GRID_ROWS {
            for col in 3..GRID_COLS {
                match self.grid[row][col] {
                    BoardSpace::Empty => (),
                    BoardSpace::Piece(piece) => {
                        if piece == self.grid[row - 1][col - 1]
                            && piece == self.grid[row - 2][col - 2]
                            && piece == self.grid[row - 3][col - 3]
                        {
                            return Some(piece);
                        }
                    }
                }
            }
        }

        // All win condition checks failed - no one has won yet!
        None
    }

    /// Checks if the board is full by seeing if the topmost row is full.
    // TODO remove cfg(test) when game loop is implemented
    #[cfg(test)]
    fn is_full(&self) -> bool {
        for space in self.grid[0] {
            if BoardSpace::Empty == space {
                return false;
            }
        }
        true
    }

    #[cfg(test)]
    fn new(grid: GameGrid) -> Self {
        Self { grid }
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

    #[test]
    fn is_full_empty() {
        assert!(!GameBoard::default().is_full());
    }

    #[test]
    fn is_full_full() {
        let grid: GameGrid = [[BoardSpace::Piece(GamePiece::Red); 7]; 6];
        let board = GameBoard::new(grid);

        assert!(board.is_full());
    }

    #[test]
    fn is_full_incomplete() {
        let mut board = GameBoard::default();

        for col in 1..GRID_COLS {
            for _ in 0..GRID_ROWS {
                board.insert_piece(GamePiece::Red, col).unwrap();
            }
        }
        for _ in 0..GRID_ROWS - 1 {
            board.insert_piece(GamePiece::Red, GRID_COLS).unwrap();
        }

        assert!(!board.is_full());
    }

    #[test]
    fn is_winner_empty() {
        assert_eq!(None, GameBoard::default().is_winner())
    }

    #[test]
    fn is_winner_horizonal() {
        let mut board = GameBoard::default();
        let piece = GamePiece::Red;

        board.insert_piece(piece, 1).unwrap();
        board.insert_piece(piece, 2).unwrap();
        board.insert_piece(piece, 3).unwrap();
        board.insert_piece(piece, 4).unwrap();

        assert_eq!(Some(piece), board.is_winner());
    }

    #[test]
    fn is_winner_vertical() {
        let mut board = GameBoard::default();
        let piece = GamePiece::Red;

        board.insert_piece(piece, 1).unwrap();
        board.insert_piece(piece, 1).unwrap();
        board.insert_piece(piece, 1).unwrap();
        board.insert_piece(piece, 1).unwrap();

        assert_eq!(Some(piece), board.is_winner());
    }

    #[test]
    fn is_winner_diagonal_up() {
        let mut board = GameBoard::default();
        let win_piece = GamePiece::Red;
        let lose_piece = GamePiece::Yellow;

        board.insert_piece(win_piece, 1).unwrap();

        board.insert_piece(lose_piece, 2).unwrap();
        board.insert_piece(win_piece, 2).unwrap();

        board.insert_piece(lose_piece, 3).unwrap();
        board.insert_piece(lose_piece, 3).unwrap();
        board.insert_piece(win_piece, 3).unwrap();

        board.insert_piece(lose_piece, 4).unwrap();
        board.insert_piece(lose_piece, 4).unwrap();
        board.insert_piece(lose_piece, 4).unwrap();
        board.insert_piece(win_piece, 4).unwrap();

        assert_eq!(Some(win_piece), board.is_winner());
    }

    #[test]
    fn is_winner_diagonal_down() {
        let mut board = GameBoard::default();
        let win_piece = GamePiece::Red;
        let lose_piece = GamePiece::Yellow;

        board.insert_piece(lose_piece, 1).unwrap();
        board.insert_piece(lose_piece, 1).unwrap();
        board.insert_piece(lose_piece, 1).unwrap();
        board.insert_piece(win_piece, 1).unwrap();

        board.insert_piece(lose_piece, 2).unwrap();
        board.insert_piece(lose_piece, 2).unwrap();
        board.insert_piece(win_piece, 2).unwrap();

        board.insert_piece(lose_piece, 3).unwrap();
        board.insert_piece(win_piece, 3).unwrap();

        board.insert_piece(win_piece, 4).unwrap();

        assert_eq!(Some(win_piece), board.is_winner());
    }
}
