use std::fs::File;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::{action_state::ActionState, input_map::InputMap, InputManagerBundle};

use crate::{
    input::PlayerAction,
    physics::{CollisionGroup, ControllerBundle},
};

use super::character::{CharacterBundle, CharacterProperty};

#[derive(Component, Default)]
pub struct Player {
    pub is_grounded: bool,
}

#[derive(Component, Default)]
pub struct PlayerCamera;

#[derive(Bundle, Default)]
pub struct PlayerCameraBundle {
    pub player_camera: PlayerCamera,
    pub camera: Camera2dBundle,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub character: CharacterBundle,

    pub player: Player,

    pub input: InputManagerBundle<PlayerAction>,
    pub locked_axes: LockedAxes,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        let mut input_map = InputMap::default();
        input_map.insert(PlayerAction::Jump, KeyCode::Space);
        input_map.insert(PlayerAction::Duck, KeyCode::ArrowDown);

        Self {
            character: CharacterBundle {
                controller: ControllerBundle {
                    ..Default::default()
                },
                ..Default::default()
            },
            player: Player::default(),
            input: InputManagerBundle::with_map(input_map),
            locked_axes: LockedAxes::ROTATION_LOCKED | LockedAxes::TRANSLATION_LOCKED_X,
        }
    }
}
fn player_jump(
    mut q_player: Query<(&mut Velocity, &ActionState<PlayerAction>, &Player), With<Player>>,
) {
    for (mut velocity, input_data, player) in &mut q_player {
        if input_data.just_pressed(&PlayerAction::Jump) {
            if player.is_grounded {
                info!("jump");
                velocity.linvel = Vec2::new(0.0, 1.0) * 300.0;
            }
        }
    }
}

fn ground_check(
    mut q_player: Query<(&mut Player, &Transform)>,
    rapier_context: Res<RapierContext>,
) {
    for (mut player, transform) in &mut q_player {
        let source = transform.translation.xy();
        let direction = Vec2::new(0.0, -1.0);
        // TODO: read from player
        let toi = 17.0;
        if let Some((_hit_entity, _intersection_toi)) = rapier_context.cast_ray(
            source,
            direction,
            toi,
            false,
            QueryFilter::new().groups(CollisionGroups::new(
                CollisionGroup::Common.group(),
                CollisionGroup::Wall.group(),
            )),
        ) {
            player.is_grounded = true;
        } else {
            player.is_grounded = false;
        }
    }
}

fn player_duck() {}

fn player_death() {}

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let player_file = File::open("assets/player.yaml");
    match player_file {
        Ok(f) => {
            let player: CharacterProperty = serde_yaml::from_reader(f).unwrap();
            let model = player.get_model(&asset_server, &mut texture_atlas_layouts);
            // spawn player
            let player_bundle = PlayerBundle::default();

            let entity_commands = commands.spawn(player_bundle);
            model.spawn(entity_commands);
        }
        Err(e) => error!("Failed to load player! {}", e),
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .add_systems(Update, player_jump)
            .add_systems(Update, player_duck)
            .add_systems(Update, player_death)
            .add_systems(Update, ground_check);
    }
}
