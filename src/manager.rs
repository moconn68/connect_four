use crate::client::GameClient;
use crate::game::state::EndgameType;
use crate::view::ViewManager;

/// High-level functions for the game implementation.
pub(crate) trait FourStackGame {
    /// Main game loop that runs continuously until and endgame state is reached.
    ///
    /// Returns a [`bool`] when the loop exits; if `true` the user wants to start a new game.
    fn game_loop(&mut self) -> bool;
}

/// Arbitrates the high-level workflow of a FourStack game.
/// This client is agnostic to the game type (local, AI, online) being played, and should have the same high-level workflow for all modes.
pub struct GameManager<V> {
    client: Box<dyn GameClient>,
    view_manager: V,
}

impl<V> GameManager<V> {
    pub fn new(client: Box<dyn GameClient>, view_manager: V) -> Self {
        Self {
            client,
            view_manager,
        }
    }
}

impl<V: ViewManager> FourStackGame for GameManager<V> {
    fn game_loop(&mut self) -> bool {
        loop {
            let game_state = self.client.get_current_state();
            match game_state.check_endgame() {
                EndgameType::None => (),
                s => {
                    return self.view_manager.show_endgame(game_state.get_board(), &s);
                }
            }

            let col_choice = self
                .view_manager
                .get_column_selection(game_state.get_board(), game_state.get_next_player());

            if let Err(e) = self.client.handle_input_move(col_choice) {
                self.view_manager.show_error(e);
            }
        }
    }
}
