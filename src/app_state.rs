use bevy::prelude::States;
#[derive(States, Copy, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    //About,
    InGame,
    //Instructions,
    //Loading,
    #[default]
    MainMenu,
    Quit
}