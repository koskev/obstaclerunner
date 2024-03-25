use animation::{
    animate_sprites, AnimationBundle, AnimationIndices, AnimationTimer, Model, Models,
};
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::{
    dynamics::{LockedAxes, RigidBody, Velocity},
    geometry::{ActiveEvents, Collider, Sensor},
    pipeline::CollisionEvent,
    plugin::{NoUserData, RapierConfiguration, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use input::PlayerAction;
use leafwing_input_manager::{
    action_state::ActionState, input_map::InputMap, plugin::InputManagerPlugin, InputManagerBundle,
};
use physics::{ColliderChild, ControllerBundle, RigidBodyBundle};
use rand::Rng;

use std::{collections::HashMap, time::Duration};

mod animation;
mod input;
mod physics;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(InputManagerPlugin::<PlayerAction>::default())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Startup, setup_player)
        .add_systems(Startup, setup_world)
        .add_systems(Startup, setup_large_obstacle)
        .add_systems(Update, (player_duck, player_jump, on_death))
        .add_systems(Update, update_world)
        .add_systems(Update, animate_sprites)
        .add_systems(Update, collision)
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0.0, -800.0),
            ..Default::default()
        })
        .run();
}

#[derive(Component, Default)]
pub struct Player;

#[derive(Component, Default)]
pub struct Despawner;

#[derive(Component, Default)]
pub struct Character;

#[derive(Component, Default)]
pub struct Obstacle;

#[derive(Component, Default, Hash, PartialEq, Eq, Debug)]
pub enum CharacterAnimationState {
    #[default]
    Running,
    Jumping,
    Ducking,
}

#[derive(Bundle, Default)]
pub struct ObstacleBundle {
    pub obstacle: Obstacle,
    pub controller: ControllerBundle,
}

#[derive(Bundle, Default)]
pub struct CharacterBundle {
    pub animation_state: CharacterAnimationState,

    pub controller: ControllerBundle,

    pub character: Character,
}

#[derive(Component, Default)]
pub struct PlayerCamera;

#[derive(Bundle, Default)]
pub struct PlayerCameraBundle {
    player_camera: PlayerCamera,
    camera: Camera2dBundle,
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

fn spawn_obstacle(mut commands: Commands, model: Model) {
    info!("Spawning obstacle");
    let obstacle = ObstacleBundle {
        ..Default::default()
    };
    let mut entity_commands = commands.spawn(obstacle);
    entity_commands.insert(LockedAxes::ROTATION_LOCKED);
    model.spawn(entity_commands);
}

fn update_world(
    commands: Commands,
    time: Res<Time>,
    mut last_update: Local<Duration>,
    mut q_obstacale: Query<&mut Transform, With<Obstacle>>,
    models: Res<Models>,
) {
    // Scroll world
    for mut transform in &mut q_obstacale {
        transform.translation.x -= 100.0 * time.delta_seconds();
    }

    let next_spawn_ms = rand::thread_rng().gen_range(800..1500);
    // Generate new obstacles
    if time.elapsed() - *last_update > Duration::from_millis(next_spawn_ms) {
        *last_update = time.elapsed();
        // spawn new
        let mut model = models.models.get("enemy1").unwrap().clone();
        model.spritesheet.transform.translation.x = 200.0;
        spawn_obstacle(commands, model);
    }
}

fn setup(mut commands: Commands) {
    // spawn camera
    let mut camera_bundle = PlayerCameraBundle::default();
    camera_bundle.camera.projection.scale = 0.25;

    commands.spawn(camera_bundle);
}

fn setup_world(mut commands: Commands) {
    // spawn initial world
    // Spawn ground (fixed)
    let ground_bundle = RigidBodyBundle {
        body: RigidBody::Fixed,
        ..Default::default()
    };
    commands
        .spawn(ground_bundle)
        .insert(Collider::cuboid(1000.0, 0.01));

    // Spawn delection sensor
    commands
        .spawn(Collider::cuboid(100.0, 100.0))
        .insert(TransformBundle {
            local: Transform::from_translation(Vec3::new(-200.0, 0.0, 0.0)),
            ..Default::default()
        })
        .insert(Sensor)
        .insert(Despawner::default())
        .insert(ActiveEvents::COLLISION_EVENTS);
}

fn setup_large_obstacle(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let idle_set = asset_server.load("tiles/dungeon.png");

    let layout = TextureAtlasLayout::from_grid(
        Vec2::new(32.0, 32.0),
        8,
        1,
        None,
        Some(Vec2::new(16.0, 16.0 + 13.0 * 32.0)),
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { first: 0, last: 5 };
    let bundle = Model {
        animation: AnimationBundle {
            indices: animation_indices.clone(),
            timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        },
        spritesheet: SpriteSheetBundle {
            texture: idle_set,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
            sprite: Sprite {
                flip_x: true,
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 16.0, 0.0)),
            ..Default::default()
        },
        collider: ColliderChild {
            collider: Collider::capsule_y(10.0, 5.0),
            ..Default::default()
        },
    };

    let mut model_map = HashMap::new();
    model_map.insert("enemy1".to_string(), bundle);

    commands.insert_resource(Models { models: model_map });
}

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let idle_set = asset_server.load("tiles/dungeon.png");

    let layout = TextureAtlasLayout::from_grid(
        Vec2::new(16.0, 32.0),
        9,
        1,
        None,
        Some(Vec2::new(8.0 * 16.0, 0.0)),
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { first: 0, last: 6 };
    let bundle = Model {
        animation: AnimationBundle {
            indices: animation_indices.clone(),
            timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        },
        spritesheet: SpriteSheetBundle {
            texture: idle_set,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
            transform: Transform::from_translation(Vec3::new(0.0, 15.0, 0.0)),
            ..Default::default()
        },
        collider: ColliderChild {
            collider: Collider::capsule_y(3.5, 5.0),
            transform: TransformBundle {
                local: Transform::from_translation(Vec3::new(0.0, -8.0, 0.0)),
                ..Default::default()
            },
            ..Default::default()
        },
    };

    // spawn player
    let player_bundle = PlayerBundle::default();

    let entity_commands = commands.spawn(player_bundle);
    bundle.spawn(entity_commands);
}

fn player_jump(mut q_player: Query<(&mut Velocity, &ActionState<PlayerAction>), With<Player>>) {
    for (mut velocity, input_data) in &mut q_player {
        if input_data.just_pressed(&PlayerAction::Jump) {
            info!("jump");
            velocity.linvel = Vec2::new(0.0, 1.0) * 300.0;
        }
    }
}

fn player_duck() {}

fn on_death() {}

fn collision(
    mut commands: Commands,
    mut er_collision: EventReader<CollisionEvent>,
    q_despawner: Query<Entity, With<Despawner>>,
    q_obstacle: Query<Entity, With<Obstacle>>,
    q_collider_children: Query<&Parent, With<Collider>>,
) {
    for event in er_collision.read() {
        match event {
            CollisionEvent::Started(ent1, ent2, _flags) => {
                let (_despawner, obstacle_collider) = if q_despawner.contains(*ent1) {
                    (*ent1, *ent2)
                } else if q_despawner.contains(*ent2) {
                    (*ent2, *ent1)
                } else {
                    continue;
                };
                if let Ok(parent) = q_collider_children.get(obstacle_collider) {
                    if let Ok(obstacle) = q_obstacle.get(parent.get()) {
                        commands.entity(obstacle).despawn_recursive();
                    }
                }
            }
            _ => (),
        }
    }
}
