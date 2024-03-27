use bevy::{input::touch::TouchPhase, prelude::*};
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::{action_state::ActionState, input_map::InputMap, InputManagerBundle};

use crate::{
    input::PlayerAction,
    physics::{CollisionGroup, ControllerBundle},
    AppState, GameState,
};

use super::{
    character::{CharacterBundle, CharacterProperty},
    enemy::Enemy,
};

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

#[derive(Event)]
struct PlayerHitEvent {
    pub player: Entity,
    pub enemy: Entity,
}

#[derive(Event)]
struct PlayerDeathEvent {
    pub player: Entity,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        let mut input_map = InputMap::default();
        input_map.insert(PlayerAction::Jump, KeyCode::Space);
        input_map.insert(PlayerAction::Duck, KeyCode::ArrowDown);
        input_map.insert(PlayerAction::DebugRenderer, KeyCode::F1);
        input_map.insert(PlayerAction::Pause, KeyCode::Escape);

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

fn inject_touch(
    mut er_touch: EventReader<TouchInput>,
    mut q_player: Query<&mut ActionState<PlayerAction>, With<Player>>,
) {
    for event in er_touch.read() {
        match event.phase {
            TouchPhase::Started => {
                for mut action in &mut q_player {
                    action.press(&PlayerAction::Jump);
                }
            }
            TouchPhase::Ended => {
                for mut action in &mut q_player {
                    action.release(&PlayerAction::Jump);
                }
            }
            _ => (),
        }
    }
}

fn pause_game(
    mut commands: Commands,
    q_player: Query<&ActionState<PlayerAction>, With<Player>>,
    game_state: Res<State<GameState>>,
) {
    for input_action in &q_player {
        if input_action.just_pressed(&PlayerAction::Pause) {
            let next_state = match game_state.get() {
                GameState::Running => GameState::Paused,
                GameState::Paused => GameState::Running,
            };
            commands.insert_resource(NextState(Some(next_state)));
        }
    }
}

fn toggle_debug(
    q_player: Query<&ActionState<PlayerAction>, With<Player>>,
    mut debug: ResMut<DebugRenderContext>,
) {
    for input_action in &q_player {
        if input_action.just_pressed(&PlayerAction::DebugRenderer) {
            debug.enabled = !debug.enabled;
            // Prevent toggling multiple times
            break;
        }
    }
}

fn player_jump(
    mut q_player: Query<(&mut Velocity, &ActionState<PlayerAction>, &Player), With<Player>>,
) {
    for (mut velocity, input_data, player) in &mut q_player {
        if input_data.just_pressed(&PlayerAction::Jump) && player.is_grounded {
            info!("jump");
            velocity.linvel = Vec2::new(0.0, 1.0) * 300.0;
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

#[derive(Resource, Default)]
pub struct PlayerHandle(Handle<CharacterProperty>);

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player: Handle<CharacterProperty> = asset_server.load("player.yaml");

    commands.insert_resource(PlayerHandle(player));
}

fn setup_player(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
    mut player_assets: ResMut<Assets<CharacterProperty>>,
    player_handle: Res<PlayerHandle>,
) {
    if let Some(player) = player_assets.remove(player_handle.0.id()) {
        let model = player.get_model(&asset_server, &mut texture_atlas_layouts);
        // spawn player
        let player_bundle = PlayerBundle::default();

        let entity_commands = commands.spawn(player_bundle);
        model.spawn(entity_commands);
    }
}

fn handle_hit(
    mut er_hit: EventReader<PlayerHitEvent>,
    mut ew_death: EventWriter<PlayerDeathEvent>,
) {
    for event in er_hit.read() {
        ew_death.send(PlayerDeathEvent {
            player: event.player,
        });
    }
}

fn handle_death(mut er_death: EventReader<PlayerDeathEvent>, mut commands: Commands) {
    for _event in er_death.read() {
        commands.insert_resource(NextState(Some(AppState::GameOver)));
    }
}

fn collision(
    mut er_collision: EventReader<CollisionEvent>,
    q_enemy: Query<Entity, With<Enemy>>,
    q_player: Query<Entity, With<Player>>,
    q_parents: Query<&Parent>,
    mut ew_hit: EventWriter<PlayerHitEvent>,
) {
    for event in er_collision.read() {
        if let CollisionEvent::Started(ent1, ent2, _flags) = event {
            if let (Ok(ent1_parent), Ok(ent2_parent)) = (q_parents.get(*ent1), q_parents.get(*ent2))
            {
                let (player, other) = if q_player.contains(ent1_parent.get()) {
                    (ent1_parent.get(), ent2_parent.get())
                } else if q_player.contains(*ent2) {
                    (ent2_parent.get(), ent1_parent.get())
                } else {
                    continue;
                };
                // we got a player collision with some other

                // Hit with enemy -> send event
                if let Ok(enemy) = q_enemy.get(other) {
                    ew_hit.send(PlayerHitEvent { player, enemy });
                    info!("player with enemy collision! {:?} {:?}", player, enemy);
                }
            }
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_assets)
            .add_systems(Update, setup_player)
            .add_systems(Update, pause_game.run_if(in_state(AppState::Game)))
            .add_systems(
                Update,
                (
                    player_jump,
                    player_duck,
                    player_death,
                    ground_check,
                    toggle_debug,
                    collision,
                    handle_hit,
                    handle_death,
                    inject_touch,
                )
                    .run_if(in_state(GameState::Running))
                    .run_if(in_state(AppState::Game)),
            )
            .init_resource::<PlayerHandle>()
            .add_event::<PlayerHitEvent>()
            .add_event::<PlayerDeathEvent>();
    }
}
