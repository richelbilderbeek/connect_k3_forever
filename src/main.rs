use crate::app::*;

mod app;
mod game_assets;
mod band;
mod game;
mod app_state;
mod player;
mod hair_color;

fn main() {
    let mut app = create_app();
    app.run();
}
