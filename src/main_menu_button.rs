use bevy::math::Vec3;
use bevy::prelude::{default, Commands, Component, Text, Text2dBundle, TextStyle, Transform};
use crate::k3_color::get_rainbow_dress_red;
use crate::language::Language;
use crate::main_menu::{get_main_menu_font_size, MainMenuComponent};
use crate::main_menu_start_button::MainMenuStartButtonComponent;

/// Marker component
#[derive(Component)]
pub struct MainMenuButtonComponent;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MainMenuButtonType {
    About,
    Instructions,
    Quit,
    Start
}

pub fn add_main_menu_button(mut commands: Commands, button_type: MainMenuButtonType, language: Language) {
    let text_str = get_main_menu_button_text(button_type, language);
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
    commands.spawn((text_bundle, MainMenuComponent, MainMenuButtonComponent, MainMenuStartButtonComponent));
}


/// From https://stackoverflow.com/a/32712140
impl std::fmt::Display for MainMenuButtonType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        //write!(f, "{:?}", self)
        // or, alternatively:
        std::fmt::Debug::fmt(self, f)
    }
}

fn get_all_main_menu_button_types() -> Vec<MainMenuButtonType> {
    vec![
        MainMenuButtonType::About,
        MainMenuButtonType::Instructions,
        MainMenuButtonType::Quit,
        MainMenuButtonType::Start
    ]
}

fn get_main_menu_button_text(button_type: MainMenuButtonType, language: Language) -> String {
    match language {
        Language::Dutch => get_dutch_main_menu_button_text(button_type),
        Language::English => get_english_main_menu_button_text(button_type)
    }
}

fn get_dutch_main_menu_button_text(button_type: MainMenuButtonType) -> String {
    match button_type {
        MainMenuButtonType::About => "Over".to_string(),
        MainMenuButtonType::Instructions => "Spelregels".to_string(),
        MainMenuButtonType::Start => "Begin".to_string(),
        MainMenuButtonType::Quit => "Stoppen".to_string(),
    }
}

fn get_english_main_menu_button_text(button_type: MainMenuButtonType) -> String {
    button_type.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_compare_main_menu_button_types() {
        assert_eq!(MainMenuButtonType::Start, MainMenuButtonType::Start);
        assert_ne!(MainMenuButtonType::Start, MainMenuButtonType::Quit);
    }
    #[test]
    fn test_can_convert_main_menu_button_types_to_string() {
        assert_eq!(MainMenuButtonType::Start.to_string(), "Start");
    }
    #[test]
    fn test_get_dutch_main_menu_button_text() {
        assert_eq!(get_dutch_main_menu_button_text(MainMenuButtonType::About), "Over");
        assert_eq!(get_dutch_main_menu_button_text(MainMenuButtonType::Instructions), "Spelregels");
        assert_eq!(get_dutch_main_menu_button_text(MainMenuButtonType::Quit), "Stoppen");
        assert_eq!(get_dutch_main_menu_button_text(MainMenuButtonType::Start), "Begin");

    }
    #[test]
    fn test_get_english_main_menu_button_text() {
        for t in get_all_main_menu_button_types() {
            assert_eq!(get_english_main_menu_button_text(t.clone()), t.to_string());
        }
    }
    #[test]
    fn test_get_main_menu_button_text() {
        for t in get_all_main_menu_button_types() {
            assert_eq!(get_dutch_main_menu_button_text(t.clone()), get_main_menu_button_text(t.clone(), Language::Dutch));
            assert_eq!(get_english_main_menu_button_text(t.clone()), get_main_menu_button_text(t.clone(), Language::English));
        }
    }
}
