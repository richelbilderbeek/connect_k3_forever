use bevy::prelude::States;
#[derive(States, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum ProgramState {
    Loading,
    MainMenu,
    About,
    Instructions,
    #[default]
    InGame,
}