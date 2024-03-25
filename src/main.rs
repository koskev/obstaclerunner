use animation::{
    animate_sprites, AnimationBundle, AnimationIndices, AnimationTimer, Model, Models,
};
use bevy::{prelude::*, reflect::serde, window::PrimaryWindow};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::{
    dynamics::{LockedAxes, RigidBody, Velocity},
    geometry::{ActiveEvents, Collider, Sensor},
    pipeline::CollisionEvent,
    plugin::{NoUserData, RapierConfiguration, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use entities::{
    character::{CharacterProperties, CharacterProperty},
    enemy::{self, Enemy, EnemyPlugin},
    player::{Player, PlayerBundle, PlayerCameraBundle, PlayerPlugin},
};
use input::PlayerAction;
use leafwing_input_manager::{
    action_state::ActionState, input_map::InputMap, plugin::InputManagerPlugin, InputManagerBundle,
};
use physics::{ColliderChild, ControllerBundle, RigidBodyBundle};
use rand::{seq::IteratorRandom, Rng};

use std::{collections::HashMap, fs::File, time::Duration};

use crate::entities::enemy::EnemyBundle;

mod animation;
mod entities;
mod input;
mod physics;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(InputManagerPlugin::<PlayerAction>::default())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .add_systems(Startup, setup)
        .add_systems(Startup, setup_world)
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
pub struct Despawner;

fn spawn_enemy(mut commands: Commands, model: Model) {
    info!("Spawning enemy");
    let enemy = EnemyBundle {
        ..Default::default()
    };
    let mut entity_commands = commands.spawn(enemy);
    entity_commands.insert(LockedAxes::ROTATION_LOCKED);
    model.spawn(entity_commands);
}

fn update_world(
    commands: Commands,
    time: Res<Time>,
    mut last_update: Local<Duration>,
    mut q_obstacale: Query<&mut Transform, With<Enemy>>,
    models: Res<Models>,
) {
    // Scroll world
    for mut transform in &mut q_obstacale {
        transform.translation.x -= 100.0 * time.delta_seconds();
    }

    let next_spawn_ms = rand::thread_rng().gen_range(800..1500);
    // Generate new enemys
    if time.elapsed() - *last_update > Duration::from_millis(next_spawn_ms) {
        *last_update = time.elapsed();
        // spawn new
        let model_key = models
            .models
            .keys()
            .choose(&mut rand::thread_rng())
            .unwrap();
        let mut model = models.models.get(model_key).unwrap().clone();
        model.spritesheet.transform.translation.x = 200.0;
        spawn_enemy(commands, model);
    }
}

fn setup(mut commands: Commands) {
    // spawn camera
    let mut camera_bundle = PlayerCameraBundle::default();
    camera_bundle.camera.projection.scale = 0.25;

    commands.spawn(camera_bundle);
}

fn setup_world(mut commands: Commands) {
    // spawn initial world#[serde(default)]
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
