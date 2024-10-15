use bevy::color::Color;
use bevy::math::Vec3;
use bevy::prelude::{default, Commands, Component, Entity, KeyCode, Text, Text2dBundle, TextStyle, Transform};
use crate::app::{create_app_with_game_state, create_default_app};
use crate::app_state::AppState;

/// A marker component.
/// All components used in the main menu are marked with this
#[derive(Component)]
pub struct MainMenuComponent;

/// When the menu menu starts
pub fn add_main_menu_components(mut commands: Commands) {
    let texts = ["Connect K3 Forever",
        "Main Menu",
        "Start",
        "Instructions",
        "About",
        "Quit"];
    let font_size = 60.0;
    let row_height = font_size * 1.3;
    let vertical_offset = (texts.len() as f32 * row_height) / 2.0;
    let color = Color::srgba(1.0, 0.8, 0.8, 1.0);
    for (i, &str) in texts.iter().enumerate() {
        let text_style = TextStyle { font_size, color, ..default() };
        let text = Text::from_section(str, text_style);
        let y = vertical_offset - (row_height * i as f32);
        let transform = Transform {
            translation: Vec3::new(0.0, y, 0.0),
            ..default()
        };
        let text_bundle = Text2dBundle {
            text,
            transform,
            ..default()
        };
        commands.spawn((text_bundle, MainMenuComponent));

        // Same, but with shadow, for shadow
        let black_color = Color::srgba(0.0, 0.0, 0.0, 1.0);
        let black_text_style = TextStyle { font_size, color: black_color, ..default() };
        let black_text = Text::from_section(str, black_text_style);
        let black_delta = 4.0;
        let black_transform = Transform {
            translation: Vec3::new(0.0 + black_delta, y - black_delta, -0.05),
            ..default()
        };
        let black_text_bundle = Text2dBundle {
            text: black_text,
            transform: black_transform,
            ..default()
        };
        commands.spawn((black_text_bundle, MainMenuComponent));
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_starts_at_main_menu() {
        let mut app = create_default_app();
        app.update();
        assert_eq!(crate::app::get_program_state(&mut app), AppState::MainMenu);
    }

    #[test]
    fn test_main_menu_has_multiple_menu_components() {
        let mut app = create_app_with_game_state(AppState::MainMenu);
        app.update();
        assert!(crate::app::count_n_main_menu_components(&mut app) > 1);
    }

    #[test]
    fn test_key_q_in_main_menu_exits_program() {
        let mut app = create_default_app();
        app.update();
        assert_eq!(crate::app::get_program_state(&mut app), AppState::MainMenu);
        app.world_mut()
            .send_event(bevy::input::keyboard::KeyboardInput {
                key_code: KeyCode::KeyQ,
                logical_key: bevy::input::keyboard::Key::Character("q".parse().unwrap()),
                state: bevy::input::ButtonState::Pressed,
                window: Entity::PLACEHOLDER,
            });
        app.update();
        app.update();
        assert_eq!(crate::app::get_program_state(&mut app), AppState::Quit);
    }

    #[test]
    fn test_key_s_starts_game() {
        let mut app = create_default_app();
        app.update();
        assert_eq!(crate::app::get_program_state(&mut app), AppState::MainMenu);
        app.world_mut()
            .send_event(bevy::input::keyboard::KeyboardInput {
                key_code: KeyCode::KeyS,
                logical_key: bevy::input::keyboard::Key::Character("s".parse().unwrap()),
                state: bevy::input::ButtonState::Pressed,
                window: Entity::PLACEHOLDER,
            });
        app.update();
        app.update();
        assert_eq!(crate::app::get_program_state(&mut app), AppState::InGame);
    }
}
