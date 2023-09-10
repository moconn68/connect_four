pub(crate) mod tui;

use crate::game::GameMode;
use crate::game::{
    board::{GameBoard, GamePiece},
    state::EndgameType,
};

/// Defines the behavior of UI components, mainly displaying individual views/screens.
pub trait ViewManager {
    /// Displays the main menu, including the title splash and game mode selection menu.
    ///
    /// Returns the user-selected [`GameMode`].
    fn main_menu(&mut self) -> GameMode;
    /// Displays the game board and prompts the user for input to get a column number selection.
    ///
    /// Returns the column selection.
    fn get_column_selection(&mut self, board: &GameBoard, player: &GamePiece) -> usize;
    /// Displays an error message to the user.
    fn show_error(&mut self, error: impl Into<String>);
    /// Shows the endgame board state when a game is over and asks the user if they want to play again.
    ///
    /// Returns user choice to play again where yes = true.
    fn show_endgame(&mut self, board: &GameBoard, state: &EndgameType) -> bool;
}
