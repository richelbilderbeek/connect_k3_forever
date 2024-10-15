use crate::app::*;

mod app;
mod game_assets;
mod band;
mod game;
mod app_state;
mod player;
mod hair_color;
mod k3_color;
mod main_menu;
mod language;
mod main_menu_start_button;

fn main() {
    let mut app = create_default_app();
    app.run();
}
