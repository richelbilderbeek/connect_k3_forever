use bevy::prelude::*;
use crate::app_state::AppState;
use crate::player::Player;
use bevy::input::InputPlugin;

pub fn create_app() -> App {
    let mut app = App::new();

    // Only add these plugin in testing.
    // The main app will assume it to be absent.
    // Adding DefaultPlugins will cause tests to crash
    if cfg!(test) {
        app.add_plugins(MinimalPlugins);
        //app.add_plugins(TaskPoolPlugin::default());
        app.add_plugins(AssetPlugin::default());
        app.init_asset::<bevy::render::texture::Image>();
        app.add_plugins(InputPlugin);
        app.add_plugins(bevy::state::app::StatesPlugin);
    } else {
        app.add_plugins(DefaultPlugins);
    }
    app.init_state::<AppState>();
    app.add_systems(Startup, add_camera);
    app.add_systems(OnEnter(AppState::MainMenu), add_menu_text);
    app.add_systems(OnEnter(AppState::InGame), setup_game);
    app.add_systems(Update, main_menu_respond_to_keyboard.run_if(in_state(AppState::MainMenu)));
    app.add_systems(Update, in_game_respond_to_keyboard.run_if(in_state(AppState::InGame)));
    app.add_systems(OnExit(AppState::MainMenu), cleanup_main_menu);
    app.add_systems(OnExit(AppState::InGame), cleanup_game);

    // Cannot update here, as 'main' would crash,
    // as it would do 'add_player' without loading the AssetServer
    // app.update();
    app
}

fn add_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn add_menu_text(mut commands: Commands) {
    commands.spawn(Text2dBundle {
        text: Text::from_section(String::from("Menu. Press space to start"), TextStyle { ..default() }),
        ..default()
    });
}

fn cleanup_game(
    mut commands: Commands,
    query: Query<Entity, With<Player>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn cleanup_main_menu(
    mut commands: Commands,
    query: Query<Entity, With<Text>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn in_game_respond_to_keyboard(
    input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,

) {
    if input.just_pressed(KeyCode::Escape) {
        next_state.set(AppState::MainMenu);
    }
}

fn main_menu_respond_to_keyboard(
    input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,

) {
    if input.just_pressed(KeyCode::Space) {
        next_state.set(AppState::InGame);
    }
}

fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
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
fn get_program_state(app: &mut App) -> AppState {
    return *app.world_mut().resource_mut::<State<AppState>>().get()
}

#[cfg(test)]
mod tests {
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
    fn test_app_starts_at_game() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_program_state(&mut app), AppState::InGame);
    }

    /*
    #[test]
    fn test_space_starts_game() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_program_state(&mut app), AppState::Menu);
        app.world_mut()
            .send_event(bevy::input::keyboard::KeyboardInput {
                key_code: KeyCode::Space,
                logical_key: bevy::input::keyboard::Key::Space,
                state: bevy::input::ButtonState::Pressed,
                window: Entity::PLACEHOLDER,
            });
        app.update();
        app.update();
        assert_eq!(get_program_state(&mut app), AppState::InGame);
    }
    */
    #[test]
    fn test_escape_leaves_game() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_program_state(&mut app), AppState::InGame);
        app.world_mut()
            .send_event(bevy::input::keyboard::KeyboardInput {
                key_code: KeyCode::Escape,
                logical_key: bevy::input::keyboard::Key::Escape,
                state: bevy::input::ButtonState::Pressed,
                window: Entity::PLACEHOLDER,
            });
        app.update();
        app.update();
        assert_eq!(get_program_state(&mut app), AppState::MainMenu);
    }
}
