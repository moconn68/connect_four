/// Arbitrates the lifecycle of the game.
pub mod client;
/// Core components that comprise the game itself.
pub mod game;
/// UI-related functionality.
pub mod view;

use crate::client::{FourStackGame, GameClient};
use crate::view::{tui::TuiManager, ViewManager};

fn main() {
    loop {
        let mut view_manager = TuiManager::new();
        let game_mode = view_manager.main_menu();

        let mut game_client = GameClient::new(game_mode, view_manager);
        if !game_client.game_loop() {
            break;
        }
    }
}
