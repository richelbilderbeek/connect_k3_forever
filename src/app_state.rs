use bevy::prelude::States;
#[derive(States, Copy, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    //Loading,
    #[default]
    MainMenu,
    //About,
    //Instructions,
    InGame,
}