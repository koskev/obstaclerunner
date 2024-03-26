use std::{fs::File, time::Duration};

use bevy::prelude::*;
use bevy_rapier2d::dynamics::LockedAxes;
use rand::{seq::IteratorRandom, Rng};

use crate::{
    animation::{Model, Models},
    physics::ControllerBundle,
    AppState, GameState,
};

use std::collections::HashMap;

use super::character::CharacterProperties;

#[derive(Component, Default)]
pub struct Enemy;

#[derive(Bundle, Default)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub controller: ControllerBundle,
}

#[derive(Resource, Default)]
pub struct EnemyHandle(Handle<CharacterProperties>);

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let enemies: Handle<CharacterProperties> = asset_server.load("enemies.yaml");

    commands.insert_resource(EnemyHandle(enemies));
}

fn setup_enemies(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
    mut enemy_assets: ResMut<Assets<CharacterProperties>>,
    enemy_handle: Res<EnemyHandle>,
) {
    let mut model_map = HashMap::new();
    if let Some(enemies) = enemy_assets.remove(enemy_handle.0.id()) {
        info!("Setting up enemies!");
        for enemy in &enemies.characters {
            model_map.insert(
                enemy.name.clone(),
                enemy.get_model(&asset_server, &mut texture_atlas_layouts),
            );
        }
        commands.insert_resource(Models { models: model_map });
    }
}

fn spawn_enemy(
    mut commands: Commands,
    time: Res<Time<Virtual>>,
    mut last_update: Local<Duration>,
    models: Res<Models>,
) {
    let next_spawn_ms = rand::thread_rng().gen_range(800..1500);
    // Generate new enemys
    if time.elapsed() - *last_update > Duration::from_millis(next_spawn_ms) {
        *last_update = time.elapsed();
        // spawn new
        if let Some(model_key) = models.models.keys().choose(&mut rand::thread_rng()) {
            let mut model = models.models.get(model_key).unwrap().clone();
            model.spritesheet.transform.translation.x = 500.0;
            let enemy = EnemyBundle {
                ..Default::default()
            };
            let mut entity_commands = commands.spawn(enemy);
            entity_commands.insert(LockedAxes::ROTATION_LOCKED);
            model.spawn(entity_commands);
        }
    }
}

fn cleanup_enemy(mut commands: Commands, q_enemy: Query<Entity, With<Enemy>>) {
    for enemy in &q_enemy {
        commands.entity(enemy).despawn_recursive();
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_assets)
            .add_systems(Update, setup_enemies)
            .add_systems(Update, spawn_enemy.run_if(in_state(GameState::Running)))
            .add_systems(OnEnter(AppState::Game), cleanup_enemy)
            .init_resource::<EnemyHandle>();
    }
}
