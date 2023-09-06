use crate::game::state::EndgameType;
use crate::manager::GameManager;
use crate::view::ViewManager;

/// Arbitrates the high-level workflow of a FourStack game.
/// This client is agnostic to the game type (local, AI, online) being played, and should have the same high-level workflow for all modes.
pub struct GameClient<V> {
    game_manager: Box<dyn GameManager>,
    view_manager: V,
}

impl<V> GameClient<V> {
    pub fn new(game_manager: Box<dyn GameManager>, view_manager: V) -> Self {
        Self {
            game_manager,
            view_manager,
        }
    }
}

impl<V: ViewManager> GameClient<V> {
    /// Runs the main game loop until an end condition is met (winner or full board).
    ///
    /// Returns bool of whether user wants to play again or not.
    pub fn game_loop(&mut self) -> bool {
        loop {
            let game_state = self.game_manager.get_state();

            match game_state.check_endgame() {
                EndgameType::None => (),
                s => {
                    return self.view_manager.show_endgame(game_state.get_board(), &s);
                }
            }

            let column = self
                .view_manager
                .get_column_selection(game_state.get_board(), game_state.get_next_player());

            if let Err(e) = self.game_manager.take_turn(column) {
                self.view_manager.show_error(e.to_string());
            }
        }
    }
}
