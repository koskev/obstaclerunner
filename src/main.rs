use animation::{
    animate_sprites, AnimationBundle, AnimationIndices, AnimationTimer, Model, Models,
};
use bevy::{
    asset::AssetMetaCheck,
    prelude::*,
    render::{camera::Viewport, view::RenderLayers},
    window::{PrimaryWindow, WindowMode, WindowResized, WindowResolution},
};
use bevy_common_assets::yaml::YamlAssetPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_parallax::{
    CreateParallaxEvent, LayerData, LayerRepeat, LayerSpeed, ParallaxCameraComponent,
    ParallaxMoveEvent, ParallaxPlugin, RepeatStrategy,
};
use bevy_rapier2d::{
    dynamics::{LockedAxes, RigidBody, Velocity},
    geometry::{ActiveCollisionTypes, ActiveEvents, Collider, CollisionGroups, Sensor},
    pipeline::CollisionEvent,
    plugin::{NoUserData, RapierConfiguration, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use entities::{
    character::{CharacterProperties, CharacterProperty},
    enemy::{self, Enemy, EnemyPlugin},
    player::{Player, PlayerBundle, PlayerCamera, PlayerCameraBundle, PlayerPlugin},
};
use input::PlayerAction;
use leafwing_input_manager::{
    action_state::ActionState, input_map::InputMap, plugin::InputManagerPlugin, InputManagerBundle,
};
use physics::{ColliderChild, CollisionGroup, ControllerBundle, RigidBodyBundle};
use rand::{seq::IteratorRandom, Rng};
use std::{collections::HashMap, fs::File, time::Duration, vec::Vec};
use ui::GameUiPlugin;

use crate::entities::enemy::EnemyBundle;

mod animation;
mod entities;
mod input;
mod physics;
mod ui;

#[derive(States, Default, Debug, PartialEq, Eq, Hash, Clone)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}

#[derive(States, Default, Debug, PartialEq, Eq, Hash, Clone)]
pub enum GameState {
    Running,
    #[default]
    Paused,
}

#[derive(Event, Clone)]
pub struct StateChangeEvent<S: States + Clone> {
    pub next_state: S,
}

fn handle_state_change<S: States + Clone>(
    mut er_state: EventReader<StateChangeEvent<S>>,
    mut next_state: ResMut<NextState<S>>,
) {
    for event in er_state.read() {
        next_state.set(event.next_state.clone());
    }
}

#[derive(Event, Default)]
pub struct ResetGameEvent;

fn main() {
    let mut app = App::new();

    app.init_state::<GameState>().init_state::<AppState>();

    // Fix Trunk
    app.insert_resource(AssetMetaCheck::Never);

    app.add_plugins(
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    ..Default::default()
                }),
                ..Default::default()
            }),
    )
    .add_plugins(WorldInspectorPlugin::new())
    .add_plugins(InputManagerPlugin::<PlayerAction>::default())
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
    .add_plugins(RapierDebugRenderPlugin::default())
    .add_plugins(PlayerPlugin)
    .add_plugins(EnemyPlugin)
    .add_plugins(GameUiPlugin)
    .add_plugins(YamlAssetPlugin::<CharacterProperties>::new(&[
        "characters.yaml",
    ]))
    .add_plugins(YamlAssetPlugin::<CharacterProperty>::new(&[
        "character.yaml",
    ]))
    .add_plugins(ParallaxPlugin);

    app.add_systems(Startup, (setup_camera, setup_background).chain())
        .add_systems(
            Update,
            (update_world, animate_sprites, collision).run_if(in_state(GameState::Running)),
        )
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0.0, -800.0),
            ..Default::default()
        });

    app.add_systems(
        Update,
        (
            handle_state_change::<GameState>,
            handle_state_change::<AppState>,
        ),
    )
    .add_systems(Update, on_resize)
    .add_event::<StateChangeEvent<GameState>>()
    .add_event::<StateChangeEvent<AppState>>();

    app.add_systems(OnEnter(GameState::Paused), pause_time);
    app.add_systems(OnEnter(GameState::Running), resume_time);
    app.add_systems(OnEnter(AppState::GameOver), handle_gameover);
    app.add_systems(OnEnter(AppState::Game), (handle_game_start, setup_world));

    app.add_event::<ResetGameEvent>();

    app.init_resource::<Models>();

    app.run();
}

fn handle_game_start(mut commands: Commands, mut ew_reset: EventWriter<ResetGameEvent>) {
    // Cleanup old stuff
    ew_reset.send_default();
    commands.insert_resource(NextState(Some(GameState::Running)));
}

fn handle_gameover(mut commands: Commands) {
    commands.insert_resource(NextState(Some(GameState::Paused)));
    // Display UI with score etc.
}

fn resume_time(mut time: ResMut<Time<Virtual>>) {
    time.unpause();
}

fn pause_time(mut time: ResMut<Time<Virtual>>) {
    time.pause();
}

#[derive(Component, Default)]
pub struct Despawner;

fn update_world(
    time: Res<Time<Virtual>>,
    mut q_obstacale: Query<&mut Transform, With<Enemy>>,
    mut ew_parallax: EventWriter<ParallaxMoveEvent>,
    q_camera: Query<Entity, With<ParallaxCameraComponent>>,
) {
    let camera = q_camera.get_single().unwrap();
    ew_parallax.send(ParallaxMoveEvent {
        camera,
        camera_move_speed: Vec2::new(1.0, 0.0) * time.delta_seconds() * 100.0,
    });
    // Scroll world
    for mut transform in &mut q_obstacale {
        transform.translation.x -= 100.0 * time.delta_seconds();
    }
}

fn setup_background(
    mut ew_create_parallax: EventWriter<CreateParallaxEvent>,
    q_camera: Query<Entity, With<ParallaxCameraComponent>>,
) {
    let layer_speeds: Vec<f32> = vec![0.1, 0.6, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9];
    let layers: Vec<LayerData> = layer_speeds
        .iter()
        .enumerate()
        .map(|(idx, speed)| LayerData {
            speed: LayerSpeed::Horizontal(*speed),
            path: format!("tiles/river/layer{}.png", idx),
            tile_size: Vec2::new(640.0, 360.0),
            repeat: LayerRepeat::horizontally(RepeatStrategy::Same),
            rows: 1,
            cols: 1,
            z: layer_speeds.len() as f32 - idx as f32,
            scale: Vec2::splat(1.0),
            ..Default::default()
        })
        .collect();

    let parallax_entity = q_camera.single();
    ew_create_parallax.send(CreateParallaxEvent {
        camera: parallax_entity,
        layers_data: layers,
    });
}

fn on_resize(
    mut er_resize: EventReader<WindowResized>,
    mut q_window: Query<&mut Window, With<PrimaryWindow>>,
    mut q_camera: Query<&mut Transform, With<PlayerCamera>>,
) {
    for event in er_resize.read() {
        for mut window in &mut q_window {
            window.resolution = WindowResolution::new(event.width, event.height);
            for mut transform in &mut q_camera {
                // TODO: remove fixed values
                transform.translation.y = window.height() * 0.1;
                transform.translation.x = window.width() * 0.1;
            }
        }
    }
}

fn setup_camera(mut commands: Commands) {
    // spawn camera
    let mut camera_bundle = PlayerCameraBundle::default();
    camera_bundle.camera.projection.scale = 0.25;

    let mut cam_cmd = commands.spawn(camera_bundle);
    cam_cmd.insert(Name::new("Player Camera"));

    let mut parallax_camera = Camera2dBundle {
        camera: Camera {
            order: -1,
            ..Default::default()
        },
        ..Default::default()
    };
    parallax_camera.projection.scale = 0.25;
    // TODO: remove fixed values
    parallax_camera.transform.translation.y = -130.0;

    cam_cmd.with_children(|p| {
        p.spawn(parallax_camera)
            .insert(ParallaxCameraComponent::new(1))
            .insert(RenderLayers::layer(1))
            .insert(Name::new("Parallax Camera"));
    });
}

fn setup_world(mut commands: Commands) {
    // spawn initial world#[serde(default)]
    // Spawn ground (fixed)
    let ground_bundle = RigidBodyBundle {
        body: RigidBody::Fixed,
        collision_group: CollisionGroups::new(
            CollisionGroup::Wall.group() | CollisionGroup::Common.group(),
            CollisionGroup::All.group(),
        ),
        ..Default::default()
    };
    commands
        .spawn(ground_bundle)
        .insert(Collider::cuboid(1000.0, 0.01));

    // Spawn delection sensor
    commands
        .spawn(Collider::cuboid(100.0, 100.0))
        .insert(TransformBundle {
            local: Transform::from_translation(Vec3::new(-500.0, 0.0, 0.0)),
            ..Default::default()
        })
        .insert(Sensor)
        .insert(Despawner::default())
        .insert(ActiveEvents::COLLISION_EVENTS);
}

fn collision(
    mut commands: Commands,
    mut er_collision: EventReader<CollisionEvent>,
    q_despawner: Query<Entity, With<Despawner>>,
    q_enemy: Query<Entity, With<Enemy>>,
    q_collider_children: Query<&Parent, With<Collider>>,
) {
    for event in er_collision.read() {
        match event {
            CollisionEvent::Started(ent1, ent2, _flags) => {
                let (_despawner, enemy_collider) = if q_despawner.contains(*ent1) {
                    (*ent1, *ent2)
                } else if q_despawner.contains(*ent2) {
                    (*ent2, *ent1)
                } else {
                    continue;
                };
                if let Ok(parent) = q_collider_children.get(enemy_collider) {
                    if let Ok(enemy) = q_enemy.get(parent.get()) {
                        commands.entity(enemy).despawn_recursive();
                    }
                }
            }
            _ => (),
        }
    }
}
