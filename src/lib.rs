use std::fmt::{Display, Formatter, Result as FmtResult};

const SPACE_EMPTY: char = ' ';
const SPACE_RED: char = 'R';
const SPACE_YELLOW: char = 'Y';
const GRID_ROWS: usize = 6;
const GRID_COLS: usize = 7;

/// Possible states for a game board space, where [`BoardSpace::Red`] and [`BoardSpace::Yellow`]
/// represent the game pieces for each player.
#[derive(Clone, Copy, Debug, Default)]
pub enum BoardSpace {
    #[default]
    Empty,
    Red,
    Yellow,
}

impl Display for BoardSpace {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            BoardSpace::Empty => write!(f, "{}", SPACE_EMPTY),
            BoardSpace::Red => write!(f, "\x1b[91m{}\x1b[0m", SPACE_RED),
            BoardSpace::Yellow => write!(f, "\x1b[93m{}\x1b[0m", SPACE_YELLOW),
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

impl Display for GameBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        // Top of board should display the column number
        let board_header = (1..=GRID_COLS)
            .map(|num| format!("+-{}-", num))
            .collect::<Vec<String>>()
            .join("");
        write!(f, "{}+\n", board_header)?;

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
}
