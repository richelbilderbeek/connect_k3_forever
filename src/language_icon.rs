use bevy::prelude::Image;
use bevy::prelude::Handle;
use bevy::prelude::App;
use crate::main_menu::MainMenuComponent;
use bevy::prelude::Sprite;
use bevy::prelude::AssetServer;
use bevy::prelude::Res;
/// The icon at the bottom right of the menu screen
use bevy::prelude::{default, Commands, Component, Transform, Vec2, Vec3Swizzles};

/// Marker component
#[derive(Component)]
pub struct LanguageIconComponent;

pub fn add_language_icon(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            transform: Transform::from_xyz(200.0, 150.0, 0.0),
            texture: asset_server.load("uk_flag.png"),
            ..default()
        },
        MainMenuComponent, LanguageIconComponent,
    ));
}

fn count_n_language_icons(app: &mut App) -> usize {
    let mut query = app.world_mut().query::<&LanguageIconComponent>();
    return query.iter(app.world()).len();
}

fn get_language_icon_has_texture(app: &mut App) -> bool {
    let mut query = app.world_mut().query::<(&Handle<Image>, &LanguageIconComponent)>();
    let (handle, _) = query.single(app.world());
    handle.is_strong()
}

fn get_language_icon_scale(app: &mut App) -> Vec2 {
    let mut query = app.world_mut().query::<(&Transform, &LanguageIconComponent)>();
    let (transform, _) = query.single(app.world());
    transform.scale.xy()
}

#[cfg(test)]
mod tests {
    use crate::app::create_app_with_game_state;
    use crate::app_state::AppState;
    use super::*;

    #[test]
    fn test_empty_app_has_no_language_icon() {
        let mut app = App::new();
        app.update();
        assert_eq!(count_n_language_icons(&mut app), 0);
    }

    #[test]
    fn test_our_app_has_a_language_icon() {
        let mut app = create_app_with_game_state(AppState::MainMenu);
        app.update();
        assert_eq!(count_n_language_icons(&mut app), 1);
    }
    #[test]
    fn test_language_icon_has_the_default_scale() {
        let mut app = create_app_with_game_state(AppState::MainMenu);
        app.update();
        assert_eq!(get_language_icon_scale(&mut app), Vec2::new(1.0, 1.0));
    }

    #[test]
    fn test_language_icon_has_a_texture() {
        let mut app = create_app_with_game_state(AppState::MainMenu);
        app.update();
        assert!(get_language_icon_has_texture(&mut app));
    }
}
