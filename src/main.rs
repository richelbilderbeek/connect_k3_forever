use crate::app::*;

use bevy::prelude::*;
mod app;
mod game_assets;
mod band;
mod game;
mod app_state;
mod player;
mod hair_color;

fn main() {
    let mut app = create_app();
    let add_camera_fn = |mut commands: Commands| {
        commands.spawn(Camera2dBundle::default());
    };
    app.add_systems(Startup, add_camera_fn);
    app.run();
}
