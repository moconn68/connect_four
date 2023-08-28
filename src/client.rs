use crate::manager::GameManager;

/// Arbitrates the high-level workflow of a FourStack game.
/// This client is agnostic to the game type (local, AI, online) being played, and should have the same high-level workflow for all modes.
pub struct GameClient {
    game_manager: Box<dyn GameManager>,
}

impl GameClient {
    pub fn new(game_manager: Box<dyn GameManager>) -> Self {
        Self { game_manager }
    }
}

impl GameClient {
    /// Runs the main game loop until an end condition is met (winner or full board).
    pub fn game_loop(&mut self) {
        loop {
            crate::tui::display_board(self.game_manager.get_board());

            match self.game_manager.check_endgame() {
                crate::manager::EndgameState::Win(p) => {
                    println!("The {p:?} player wins!");
                    break;
                }
                crate::manager::EndgameState::Full => {
                    println!("The board is full! It's a draw!");
                    break;
                }
                crate::manager::EndgameState::None => (),
            }
            self.game_manager.take_turn();
        }
    }
}
