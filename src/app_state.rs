use bevy::{
    prelude::*
};

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

#[cfg(test)]
fn get_all_app_states() -> Vec<AppState> {
    vec![AppState::InGame, AppState::MainMenu, AppState::Quit]
}

#[cfg(test)]
pub fn get_program_state(app: &mut App) -> AppState {
    return *app.world_mut().resource_mut::<State<AppState>>().get();
}
#[cfg(test)]
mod tests {
    use crate::app::create_app_with_game_state;
    use super::*;

    #[test]
    fn test_there_are_three_app_states() {
        assert_eq!(3, get_all_app_states().len())
    }


    #[test]
    fn test_game_has_an_initial_state() {
        for state in get_all_app_states() {
            let mut app = create_app_with_game_state(state);
            app.update();
            assert_eq!(get_program_state(&mut app), state);

        }
    }
}