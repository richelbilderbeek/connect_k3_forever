use crate::app::*;

mod app;
mod game_assets;
mod band;
mod game;
mod app_state;
mod player;
mod hair_color;
mod colors;
mod main_menu;

fn main() {
    let mut app = create_default_app();
    app.run();
}
