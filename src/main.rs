/// Handles moves, updating game state, and communication to server(s).
pub mod client;
/// Core components that comprise the game itself.
pub mod game;
/// Arbitrates the lifecycle of the game.
pub mod manager;
/// UI-related functionality.
pub mod view;

use crate::client::{ai::AiGameClient, local::LocalGameClient, GameClient};
use crate::game::GameMode;
use crate::manager::{FourStackGame, GameManager};
use crate::view::{tui::TuiManager, ViewManager};

fn main() {
    loop {
        let mut view_manager = TuiManager::default();
        let game_client: Box<dyn GameClient> = match view_manager.main_menu() {
            GameMode::Ai => Box::<AiGameClient>::default(),
            GameMode::Local => Box::<LocalGameClient>::default(),
            GameMode::Online => todo!(),
        };

        let mut game_manager = GameManager::new(game_client, view_manager);
        if !game_manager.game_loop() {
            break;
        }
    }
}
