use crate::game::{
    board::GamePiece,
    state::{EndgameType, GameState},
    GameMode,
};
use crate::view::ViewManager;

/// High-level functions for the game implementation.
pub(crate) trait FourStackGame {
    /// Main game loop that runs continuously until and endgame state is reached.
    ///
    /// Returns a [`bool`] when the loop exits; if `true` the user wants to start a new game.
    fn game_loop(&mut self) -> bool;
    /// Take a turn for a local player, including handling the UI.
    ///
    /// Used for both players in a local game, or the local machine player in AI or Online games.
    ///
    /// Returns a [`String`] error message if an invalid column is selected.
    fn take_turn_local(&mut self) -> Result<(), String>;
    /// Take a turn for an adversary, where an adversary is an AI or online opponent.
    ///
    /// Includes displaying any UI while the local player waits for the opponent to take their turn.
    fn take_turn_adversary(&mut self);
}

/// Arbitrates the high-level workflow of a FourStack game.
/// This client is agnostic to the game type (local, AI, online) being played, and should have the same high-level workflow for all modes.
#[derive(Clone, Copy, Debug, Default)]
pub struct GameClient<V> {
    game_mode: GameMode,
    player_one: GamePiece,
    game_state: GameState,
    view_manager: V,
}

impl<V> GameClient<V> {
    pub fn new(game_mode: GameMode, view_manager: V) -> Self {
        Self {
            game_mode,
            player_one: GamePiece::random(),
            game_state: Default::default(),
            view_manager,
        }
    }
}

impl<V: ViewManager> FourStackGame for GameClient<V> {
    fn game_loop(&mut self) -> bool {
        // let board = self.game_state.get_board();
        // let next_player = self.game_state.get_next_player();

        loop {
            match self.game_state.check_endgame() {
                EndgameType::None => (),
                s => {
                    return self
                        .view_manager
                        .show_endgame(self.game_state.get_board(), &s);
                }
            }

            if self.player_one.eq(self.game_state.get_next_player())
                || GameMode::Local.eq(&self.game_mode)
            {
                if let Err(e) = self.take_turn_local() {
                    self.view_manager.show_error(e);
                }
            } else {
                self.take_turn_adversary();
            }
        }
    }

    fn take_turn_local(&mut self) -> Result<(), String> {
        let column = self.view_manager.get_column_selection(
            self.game_state.get_board(),
            self.game_state.get_next_player(),
        );

        let piece = *self.game_state.get_next_player();
        self.game_state
            .get_board_mut()
            .insert_piece(piece, column)
            .map_err(|e| format!("Invalid move: {e}"))?;

        self.game_state.toggle_player();

        Ok(())
    }

    fn take_turn_adversary(&mut self) {
        // TODO implement for AI and Online game modes
        todo!()
    }
}
