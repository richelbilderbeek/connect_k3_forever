use bevy::math::Vec3;
use bevy::prelude::{default, Commands, Component, Text, Text2dBundle, TextStyle, Transform};
use crate::k3_color::*;
use crate::language::Language;
use crate::main_menu::*;

/// Marker component
#[derive(Component)]
pub struct MainMenuStartButtonComponent;

pub fn add_main_menu_start_button(mut commands: Commands) {
    let language = Language::English;
    let text_str = get_main_menu_start_button_text(language);
    let font_size = get_main_menu_font_size();
    let color = get_rainbow_dress_red();
    let text_style = TextStyle { font_size, color, ..default() };
    let text = Text::from_section(text_str, text_style);
    let transform = Transform {
        translation: Vec3::new(0.0, 75.0, 0.1),
        ..default()
    };
    let text_bundle = Text2dBundle {
        text,
        transform,
        ..default()
    };
    commands.spawn((text_bundle, MainMenuComponent, MainMenuStartButtonComponent));
}


fn get_main_menu_start_button_text(language: Language) -> String {
    if language == Language::Dutch {
        get_dutch_main_menu_start_button_text()
    } else {
        assert_eq!(language, Language::English);
        get_english_main_menu_start_button_text()
    }
}

fn get_dutch_main_menu_start_button_text() -> String {
    "Begin".to_string()
}

fn get_english_main_menu_start_button_text() -> String {
    "Start".to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_dutch_main_menu_start_button_text() {
        assert_eq!(get_dutch_main_menu_start_button_text(), "Begin".to_string());
    }
    #[test]
    fn test_get_english_main_menu_start_button_text() {
        assert_eq!(get_english_main_menu_start_button_text(), "Start".to_string());
    }
    #[test]
    fn test_get_main_menu_start_button_text() {
        assert_eq!(get_main_menu_start_button_text(Language::Dutch), get_dutch_main_menu_start_button_text());
        assert_eq!(get_main_menu_start_button_text(Language::English), get_english_main_menu_start_button_text());
    }
}
