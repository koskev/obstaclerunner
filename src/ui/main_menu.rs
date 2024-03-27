use bevy::prelude::*;

use crate::AppState;

#[derive(Component)]
pub struct MainMenuMarker;

#[derive(Component)]
pub struct StartGameButton;

#[derive(Component)]
pub struct ButtonAction {
    pub on_hover: fn() -> (),
    pub on_pressed: fn() -> (),
}

pub fn handle_buttons(q_interaction: Query<(&Interaction, &ButtonAction), Changed<Interaction>>) {
    for (interaction, action) in &q_interaction {
        match *interaction {
            Interaction::Pressed => {
                (action.on_pressed)();
            }
            Interaction::Hovered => {
                (action.on_hover)();
            }
            _ => (),
        }
    }
}

pub fn handle_main_menu_button(
    q_interaction: Query<&Interaction, (Changed<Interaction>, With<MainMenuButton>)>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    for interaction in &q_interaction {
        if *interaction == Interaction::Pressed {
            next_app_state.set(AppState::MainMenu);
        }
    }
}

pub fn handle_new_game_button(
    q_interaction: Query<&Interaction, (Changed<Interaction>, With<StartGameButton>)>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    for interaction in &q_interaction {
        if *interaction == Interaction::Pressed {
            next_app_state.set(AppState::Game);
        }
    }
}

pub fn main_menu_setup(mut commands: Commands) {
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

pub fn main_menu_cleanup(
    mut commands: Commands,
    q_menu_items: Query<Entity, With<MainMenuMarker>>,
) {
    for entity in &q_menu_items {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(Component)]
pub struct GameoverMenuMarker;

#[derive(Component)]
pub struct MainMenuButton;

pub fn gameover_menu_setup(mut commands: Commands) {
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

pub fn gameover_menu_cleanup(
    mut commands: Commands,
    q_menu_items: Query<Entity, With<GameoverMenuMarker>>,
) {
    for entity in &q_menu_items {
        commands.entity(entity).despawn_recursive();
    }
}
