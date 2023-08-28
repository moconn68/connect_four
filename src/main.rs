/// Contains logic for the game "board" state and interactions.
pub mod board;
/// Arbitrates the lifecycle of the game.
pub mod client;
/// Executes individual game-specific workflows to advance the game.
pub mod manager;
/// UI-related methods.
pub mod tui;

use crate::client::GameClient;
use crate::manager::{
    ai::AiGameManager, local::LocalGameManager, online::OnlineGameManager, GameManager, GameMode,
};

use std::io::stdin;

const SPLASH: &str = r#" ______               _____ _             _    
|  ____|             / ____| |           | |   
| |__ ___  _   _ _ _| (___ | |_ __ _  ___| | __
|  __/ _ \| | | | '__\___ \| __/ _` |/ __| |/ /
| | | (_) | |_| | |  ____) | || (_| | (__|   < 
|_|  \___/ \__,_|_| |_____/ \__\__,_|\___|_|\_\
"#;

fn main() {
    println!("{SPLASH}");

    loop {
        let game_manager: Box<dyn GameManager> = match tui::get_game_mode() {
            GameMode::Ai => Box::<AiGameManager>::default(),
            GameMode::Local => Box::<LocalGameManager>::default(),
            GameMode::Online => Box::<OnlineGameManager>::default(),
        };

        let mut game_client = GameClient::new(game_manager);
        game_client.game_loop();

        println!("Play again? Y/N");
        let mut buf = String::default();
        stdin()
            .read_line(&mut buf)
            .expect("Error reading user input");

        let input = buf.trim().to_ascii_lowercase();
        match input.as_str() {
            "y" | "yes" => continue,
            _ => break,
        }
    }
}
