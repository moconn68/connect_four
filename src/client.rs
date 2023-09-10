use crate::ai::NaiveEvaluator;
use crate::game::{
    board::GamePiece,
    state::{EndgameType, GameState},
    GameMode,
};
use crate::view::ViewManager;

use minimax::{Negamax, Strategy};

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
            self.game_state.toggle_player();
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
        Ok(())
    }

    fn take_turn_adversary(&mut self) {
        // TODO implement for Online game mode
        match self.game_mode {
            // TODO this isn't great, should refactor to prevent this from being possible
            GameMode::Local => unimplemented!("Local game mode should never get here!"),
            GameMode::Online => todo!(),
            GameMode::Ai => {
                let mut strategy = Negamax::new(NaiveEvaluator, 7);
                let col_choice = strategy.choose_move(&self.game_state).unwrap();
                let player = *self.game_state.get_next_player();
                self.game_state
                    .get_board_mut()
                    .insert_piece(player, col_choice)
                    .unwrap();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{FourStackGame, GameClient};
    use crate::game::{
        board::{GameBoard, GamePiece},
        state::EndgameType,
        GameMode,
    };
    use crate::view::ViewManager;

    #[derive(Clone, Copy, Default)]
    struct StubViewManager {
        column: usize,
    }

    impl StubViewManager {
        fn set_column(&mut self, column: usize) {
            self.column = column;
        }
    }

    impl ViewManager for StubViewManager {
        fn main_menu(&mut self) -> crate::game::GameMode {
            unimplemented!()
        }

        fn get_column_selection(&mut self, _board: &GameBoard, _player: &GamePiece) -> usize {
            self.column
        }

        fn show_error(&mut self, _error: impl Into<String>) {
            unimplemented!()
        }

        fn show_endgame(&mut self, _board: &GameBoard, _state: &EndgameType) -> bool {
            unimplemented!()
        }
    }

    #[test]
    fn take_turn_local_updates_board() {
        let mut view_manager = StubViewManager::default();
        view_manager.set_column(1);
        let mut client = GameClient::new(GameMode::Local, view_manager);
        let board_before = *client.game_state.get_board();

        client.take_turn_local().unwrap();
        let board_after = *client.game_state.get_board();

        assert_ne!(board_before, board_after);
    }

    #[test]
    fn take_turn_adversary_updates_board() {
        let mut client = GameClient::new(GameMode::Ai, StubViewManager::default());
        let board_before = *client.game_state.get_board();

        client.take_turn_adversary();
        let board_after = *client.game_state.get_board();

        assert_ne!(board_before, board_after);
    }
}
