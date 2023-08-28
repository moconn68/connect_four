use crate::board::GameBoard;
use crate::manager::GameMode;

use std::io::stdin;

pub fn display_board(board: &GameBoard) {
    // TODO board should not be re-displayed multiple times; UI should be overwritten
    println!("{board}");
}

/// Prompts the player to select a game mode, and returns said [`GameMode`].
/// If the selection is invalid, loops until the user provides a valid input.
pub fn get_game_mode() -> GameMode {
    let menu_str =
        "Select a game mode:\n1. VS Computer\n2. VS Friend Offline\n3. Online matchmaking";
    println!("{menu_str}");
    loop {
        let mut selection = String::default();
        stdin()
            .read_line(&mut selection)
            .expect("Error reading user input");
        let selection_int = selection.trim().parse::<i8>().unwrap_or(-1);
        match selection_int.try_into() {
            Ok(mode) => break mode,
            Err(_) => println!("Invalid selection, please try again:"),
        }
    }
}

/// Method for getting the column selection from the user.
/// If the selection is invalid, loops until the user provides a valid input.
pub fn get_column_selection() -> usize {
    loop {
        let mut in_col = String::default();
        stdin()
            .read_line(&mut in_col)
            .expect("Unable to read user input");
        let col_num = in_col.trim().parse::<usize>();
        match col_num {
            Ok(col) => break col,
            Err(_) => println!("Selection must be a number between [1,7]; please try again:"),
        }
    }
}
