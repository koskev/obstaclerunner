use std::fs::File;

use bevy::prelude::*;

use crate::{animation::Models, physics::ControllerBundle};

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

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_assets)
            .add_systems(Update, setup_enemies)
            .init_resource::<EnemyHandle>();
    }
}
