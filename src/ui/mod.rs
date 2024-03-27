use bevy::prelude::*;
use main_menu::*;

use crate::AppState;

pub mod main_menu;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), main_menu_setup)
            .add_systems(OnExit(AppState::MainMenu), main_menu_cleanup)
            .add_systems(OnEnter(AppState::GameOver), gameover_menu_setup)
            .add_systems(OnExit(AppState::GameOver), gameover_menu_cleanup)
            .add_systems(
                Update,
                handle_new_game_button.run_if(in_state(AppState::MainMenu)),
            )
            .add_systems(
                Update,
                (handle_main_menu_button, handle_new_game_button)
                    .run_if(in_state(AppState::GameOver)),
            );
    }
}
