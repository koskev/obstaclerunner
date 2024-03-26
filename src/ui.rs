use bevy::prelude::*;

use crate::{handle_gameover, AppState};

#[derive(Component)]
struct MainMenuMarker;

#[derive(Component)]
struct StartGameButton;

fn handle_main_menu_button(
    q_interaction: Query<&Interaction, (Changed<Interaction>, With<MainMenuButton>)>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    for interaction in &q_interaction {
        match *interaction {
            Interaction::Pressed => {
                next_app_state.set(AppState::MainMenu);
            }
            _ => (),
        }
    }
}

fn handle_new_game_button(
    q_interaction: Query<&Interaction, (Changed<Interaction>, With<StartGameButton>)>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    for interaction in &q_interaction {
        match *interaction {
            Interaction::Pressed => {
                next_app_state.set(AppState::Game);
            }
            _ => (),
        }
    }
}

fn main_menu_setup(mut commands: Commands) {
    info!("Seting up mai menu");
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(MainMenuMarker)
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    ..default()
                })
                .insert(StartGameButton)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Start Game",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..Default::default()
                        },
                    ));
                });
        });
}

fn main_menu_cleanup(mut commands: Commands, q_menu_items: Query<Entity, With<MainMenuMarker>>) {
    for entity in &q_menu_items {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(Component)]
struct GameoverMenuMarker;

#[derive(Component)]
struct MainMenuButton;

fn gameover_menu_setup(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(GameoverMenuMarker)
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    ..default()
                })
                .insert(MainMenuButton)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Main Menu",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..Default::default()
                        },
                    ));
                });
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    ..default()
                })
                .insert(StartGameButton)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Start Game",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..Default::default()
                        },
                    ));
                });
        });
}

fn gameover_menu_cleanup(
    mut commands: Commands,
    q_menu_items: Query<Entity, With<GameoverMenuMarker>>,
) {
    for entity in &q_menu_items {
        commands.entity(entity).despawn_recursive();
    }
}

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
