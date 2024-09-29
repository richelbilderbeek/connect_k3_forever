use crate::app::*;

use bevy::prelude::*;
mod app;
mod game_assets;
mod band;
mod game;
mod program_state;
mod player;

fn main() {
    let mut app = create_app();
    let add_camera_fn = |mut commands: Commands| {
        commands.spawn(Camera2dBundle::default());
    };
    app.add_systems(Startup, add_camera_fn);

    assert!(!app.is_plugin_added::<AssetPlugin>());
    app.add_plugins(DefaultPlugins);
    assert!(app.is_plugin_added::<AssetPlugin>());

    app.run();
}
