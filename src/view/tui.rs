use crate::game::{
    board::{self, GameBoard, GamePiece},
    state::EndgameType,
};
use crate::manager::GameMode;
use crate::view::ViewManager;

use cursive::{
    direction::Orientation,
    theme::Theme,
    views::{Button, Dialog, LinearLayout, SelectView, TextView},
    CursiveRunnable,
};

const TITLE: &str = "FourStack";
// Created with https://patorjk.com/software/taag
const SPLASH: &str = r#" ______               _____ _             _    
|  ____|             / ____| |           | |   
| |__ ___  _   _ _ _| (___ | |_ __ _  ___| | __
|  __/ _ \| | | | '__\___ \| __/ _` |/ __| |/ /
| | | (_) | |_| | |  ____) | || (_| | (__|   < 
|_|  \___/ \__,_|_| |_____/ \__\__,_|\___|_|\_\
"#;

pub struct TuiManager {
    runtime: CursiveRunnable,
}

impl TuiManager {
    pub fn new() -> Self {
        let mut runtime = cursive::default();
        // Application-wide TUI config
        // Set theme to correspond with user's terminal
        runtime.set_theme(Theme::terminal_default());
        // Default base UI layer to prevent background flickering
        runtime.add_layer(Dialog::default());
        Self { runtime }
    }
}

impl ViewManager for TuiManager {
    fn main_menu(&mut self) -> GameMode {
        let mut layout = LinearLayout::new(Orientation::Vertical);
        layout.add_child(TextView::new(SPLASH));
        layout.add_child(TextView::new("Select game mode:"));

        let mut main_menu_selector = SelectView::<GameMode>::new();
        main_menu_selector.add_item("1. AI", GameMode::Ai);
        main_menu_selector.add_item("2. Local", GameMode::Local);
        main_menu_selector.add_item("3. Online", GameMode::Online);

        main_menu_selector.set_on_submit(|c, e| {
            c.set_user_data(*e);
            cleanup(c);
        });
        layout.add_child(main_menu_selector);

        self.runtime.add_layer(Dialog::around(layout));
        self.runtime.run();
        // This is safe because `run` above will block the thread until `set_on_submit` is called
        self.runtime
            .take_user_data()
            .expect("Cursive user data should be set to a GameMode!")
    }

    fn get_column_selection(&mut self, board: &GameBoard, player: &GamePiece) -> usize {
        let mut col_select_view = LinearLayout::new(Orientation::Horizontal);
        for i in 1..=board::GRID_COLS {
            let btn = Button::new_raw(format!(" [{}]", i), move |c| {
                c.set_user_data(i);
                cleanup(c);
            });
            col_select_view.add_child(btn);
        }

        let mut layout = LinearLayout::new(Orientation::Vertical);
        layout.add_child(TextView::new(board.to_string()));
        layout.add_child(TextView::new(format!("It is {}'s turn", player)));
        layout.add_child(col_select_view);

        self.runtime.add_layer(Dialog::around(layout).title(TITLE));
        self.runtime.run();

        self.runtime
            .take_user_data()
            .expect("Cursive user data should contain a column selection!")
    }

    fn show_error(&mut self, error: impl Into<String>) {
        self.runtime
            .add_layer(Dialog::around(TextView::new(error)).button("OK", |c| {
                cleanup(c);
            }));
        self.runtime.run();
    }

    fn show_endgame(&mut self, board: &GameBoard, state: &EndgameType) -> bool {
        let mut layout = LinearLayout::new(Orientation::Vertical);
        layout.add_child(TextView::new(board.to_string()));

        let mut banner = match state {
            EndgameType::Win(p) => format!("{p} wins!"),
            _ => "It's a draw!".to_string(),
        };
        banner.push_str(" Play again?");
        layout.add_child(TextView::new(banner));

        let mut btn_row = LinearLayout::new(Orientation::Horizontal);
        btn_row.add_child(Button::new("Yes", |c| {
            c.set_user_data(true);
            cleanup(c);
        }));
        btn_row.add_child(Button::new("No", |c| {
            c.set_user_data(false);
            cleanup(c);
        }));
        layout.add_child(btn_row);

        self.runtime.add_layer(Dialog::around(layout).title(TITLE));
        self.runtime.run();
        self.runtime.take_user_data().unwrap_or_default()
    }
}

/* Helper functions */

/// Common tear-down steps when a cursive view is closed.
fn cleanup(cursive: &mut cursive::Cursive) {
    // Clear the foreground UI layer
    cursive.pop_layer();
    // Terminate the currently running UI event loop
    cursive.quit();
}
