/// Contains logic for the game "board" state and interactions.
pub mod board;
/// Arbitrates the lifecycle of the game.
pub mod client;
/// Executes individual game-specific workflows to advance the game.
pub mod manager;
/// UI-related functionality.
pub mod view;

use crate::client::GameClient;
use crate::manager::{
    ai::AiGameManager, local::LocalGameManager, online::OnlineGameManager, GameManager, GameMode,
};
use crate::view::{tui::TuiManager, ViewManager};

fn main() {
    loop {
        let mut view_manager = TuiManager::new();
        let game_manager: Box<dyn GameManager> = match view_manager.main_menu() {
            GameMode::Ai => Box::<AiGameManager>::default(),
            GameMode::Local => Box::<LocalGameManager>::default(),
            GameMode::Online => Box::<OnlineGameManager>::default(),
        };

        let mut game_client = GameClient::new(game_manager, view_manager);
        if !game_client.game_loop() {
            break;
        }
    }
}
