use crate::app_state::AppState;
use crate::hair_color::HairColor;
use crate::main_menu::*;
use crate::player::Player;
use bevy::input::InputPlugin;
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle,  Mesh2dHandle},
};

fn add_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Build a default quad mesh
    let mut mesh = Mesh::from(Rectangle::default());
    // Build vertex colors for the quad. One entry per vertex (the corners of the quad)
    let vertex_colors: Vec<[f32; 4]> = vec![
        [1.0, 0.2, 0.3, 1.0], // Top-right
        [1.0, 0.7, 0.8, 1.0], // Top left
        [1.0, 0.2, 0.4, 1.0], // Bottom-left
        [1.0, 0.6, 0.7, 1.0], //Bottom-right
    ];
    // Insert the vertex colors as an attribute
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, vertex_colors);

    let mesh_handle: Mesh2dHandle = meshes.add(mesh).into();

    // Spawn the quad with vertex colors
    commands.spawn(MaterialMesh2dBundle {
        mesh: mesh_handle.clone(),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, -0.1))
            .with_scale(Vec3::splat(2048.0)),
        material: materials.add(ColorMaterial::default()),
        ..default()
    });
}


fn add_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn create_app_with_game_state(game_state: AppState) -> App {
    let mut app = create_default_app();
    app.insert_state(game_state);
    app
}

pub fn create_default_app() -> App {
    let mut app = App::new();

    // Only add these plugin in testing.
    // The main app will assume it to be absent.
    // Adding DefaultPlugins will cause tests to crash
    if cfg!(test) {
        app.add_plugins(MinimalPlugins);
        app.add_plugins(AssetPlugin::default()); // For assets
        app.init_asset::<bevy::render::texture::Image>(); // For using images
        app.init_asset::<bevy::render::mesh::Mesh>(); // For background
        app.init_asset::<bevy::prelude::ColorMaterial>(); // For background
        app.add_plugins(InputPlugin); // For input
        app.add_plugins(bevy::state::app::StatesPlugin); // For States
        app.add_plugins(bevy::window::WindowPlugin::default()); // For window title
    } else {
        app.add_plugins(DefaultPlugins);
    }
    app.init_state::<AppState>();
    app.add_systems(Startup, add_camera);
    app.add_systems(Startup, add_background);
    app.add_systems(OnEnter(AppState::MainMenu), add_main_menu_components);
    app.add_systems(OnEnter(AppState::InGame), setup_game);
    app.add_systems(OnEnter(AppState::Quit), setup_quit_state);
    app.add_systems(Update, main_menu_respond_to_keyboard.run_if(in_state(AppState::MainMenu)));
    app.add_systems(Update, in_game_respond_to_keyboard.run_if(in_state(AppState::InGame)));
    app.add_systems(OnExit(AppState::MainMenu), cleanup_main_menu);
    app.add_systems(OnExit(AppState::InGame), cleanup_game);

    if app.is_plugin_added::<bevy::window::WindowPlugin>() {
        set_window_title(&mut app, String::from("Connect K3 Forever"));
    }


    // Background color
    //app.insert_resource(ClearColor(Color::srgba(1.0, 0.6, 0.6, 1.0)));

    // Cannot update here, as 'main' would crash,
    // as it would do 'add_player' without loading the AssetServer
    // app.update();
    app
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
    query: Query<Entity, With<MainMenuComponent>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

#[cfg(test)]
pub fn count_n_main_menu_components(app: &mut App) -> usize {
    let mut query = app.world_mut().query::<&MainMenuComponent>();
    query.iter(app.world_mut()).len()
}

#[cfg(test)]
fn count_n_players(app: &mut App) -> usize {
    let mut query = app.world_mut().query::<&Player>();
    query.iter(app.world_mut()).len()
}


fn count_n_windows(app: &mut App) -> usize {
    let mut query = app.world_mut().query::<&Window>();
    query.iter(app.world()).len()
}


#[cfg(test)]
fn get_window_title(app: &mut App) -> String {
    assert!(app.is_plugin_added::<bevy::window::WindowPlugin>());
    assert_eq!(count_n_windows(app), 1);
    let mut query = app.world_mut().query::<&Window>();
    let window = query.single(app.world());
    window.title.clone()
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
    if input.just_pressed(KeyCode::KeyS) {
        next_state.set(AppState::InGame);
    }
    if input.just_pressed(KeyCode::KeyQ) {
        next_state.set(AppState::Quit);
    }
}

fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    let assets = crate::game_assets::GameAssets::new();
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(assets.get_player_filename(HairColor::Red, 0)),
            ..default()
        },
        Player,
    ));
}

fn set_window_title(app: &mut App, title: String) {
    assert!(app.is_plugin_added::<bevy::window::WindowPlugin>());
    assert_eq!(count_n_windows(app), 1);
    let mut query = app.world_mut().query::<&mut Window>();
    let mut window = query.single_mut(app.world_mut());
    window.title = title;

}

fn setup_quit_state(mut exit: EventWriter<AppExit>) {
    exit.send(AppExit::Success);
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
pub fn get_program_state(app: &mut App) -> AppState {
    return *app.world_mut().resource_mut::<State<AppState>>().get();
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
    fn test_game_has_a_player() {
        let mut app = create_app_with_game_state(AppState::InGame);
        app.update();
        assert_eq!(count_n_players(&mut app), 1);
    }

    #[test]
    fn test_player_is_at_origin() {
        let mut app = create_app_with_game_state(AppState::InGame);
        app.update();
        assert_eq!(get_player_position(&mut app), Vec2::new(0.0, 0.0));
    }

    #[test]
    fn test_player_has_the_default_scale() {
        let mut app = create_app_with_game_state(AppState::InGame);
        app.update();
        assert_eq!(get_player_scale(&mut app), Vec2::new(1.0, 1.0));
    }

    #[test]
    fn test_player_has_a_texture() {
        let mut app = create_app_with_game_state(AppState::InGame);
        app.update();
        assert!(get_player_has_texture(&mut app));
    }

    #[test]
    fn test_app_has_the_correct_window_title() {
        let mut app = create_default_app();
        app.update();
        assert_eq!(get_window_title(&mut app), String::from("Connect K3 Forever"));
    }

    #[test]
    fn test_escape_leaves_game() {
        let mut app = create_default_app();
        app.update();
        assert_eq!(get_program_state(&mut app), AppState::MainMenu);
        app.world_mut()
            .send_event(bevy::input::keyboard::KeyboardInput {
                key_code: KeyCode::KeyS,
                logical_key: bevy::input::keyboard::Key::Character("s".parse().unwrap()),
                state: bevy::input::ButtonState::Pressed,
                window: Entity::PLACEHOLDER,
            });
        app.update();
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
