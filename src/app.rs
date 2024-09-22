use bevy::prelude::*;
use crate::program_state::ProgramState;

#[derive(Component)]
pub struct Player;

pub fn create_app() -> App {
    let mut app = App::new();

    // Only add these plugin in testing.
    // The main app will assume it to be absent.
    // Adding DefaultPlugins will cause tests to crash
    if cfg!(test) {
        app.add_plugins(TaskPoolPlugin::default());
        app.add_plugins(AssetPlugin::default());
        app.init_asset::<bevy::render::texture::Image>();
    }
    app.init_state::<ProgramState>();
    app.add_systems(Startup, add_player);

    // Cannot update here, as 'main' would crash,
    // as it would do 'add_player' without loading the AssetServer
    // app.update();
    app
}

fn add_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let assets = crate::game_assets::GameAssets::new();
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(assets.get_player_filename(0, 0)),
            ..default()
        },
        Player,
    ));
}


#[cfg(test)]
fn count_n_players(app: &mut App) -> usize {
    let mut query = app.world_mut().query::<&Player>();
    query.iter(app.world_mut()).len()
}

#[cfg(test)]
fn get_player_position(app: &mut App) -> Vec2 {
    let mut query = app.world_mut().query::<(&Transform, &Player)>();
    let (transform, _) = query.single(app.world());
    transform.translation.xy()
}

#[cfg(test)]
fn get_player_scale(app: &mut App) -> Vec2 {
    let mut query = app.world_mut().query::<(&Transform, &Player)>();
    let (transform, _) = query.single(app.world());
    assert_eq!(transform.scale.z, 1.0); // 2D
    transform.scale.xy()
}

#[cfg(test)]
fn get_player_has_texture(app: &mut App) -> bool {
    let mut query = app.world_mut().query::<(&Handle<Image>, &Player)>();
    let (handle, _) = query.single(app.world());
    handle.is_strong()
}

#[cfg(test)]
fn get_program_state(app: &mut App) -> ProgramState {
  app.stat
}

#[cfg(test)]
mod tests {
    use crate::program_state;
    use super::*;

    #[test]
    fn test_empty_app_has_no_players() {
        let mut app = App::new();
        app.update();
        assert_eq!(count_n_players(&mut app), 0);
    }

    #[test]
    fn test_our_app_has_a_player() {
        let mut app = create_app();
        app.update();
        assert_eq!(count_n_players(&mut app), 1);
    }

    #[test]
    fn test_player_is_at_origin() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_player_position(&mut app), Vec2::new(0.0, 0.0));
    }

    #[test]
    fn test_player_has_the_default_scale() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_player_scale(&mut app), Vec2::new(1.0, 1.0));
    }

    #[test]
    fn test_player_has_a_texture() {
        let mut app = create_app();
        app.update();
        assert!(get_player_has_texture(&mut app));
    }

    #[test]
    fn test_app_starts_in_game() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_program_state(&mut app), program_state::ProgramState::InGame);
    }

}
