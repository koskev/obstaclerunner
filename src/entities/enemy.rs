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

fn setup_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let mut model_map = HashMap::new();

    let enemy_file = File::open("assets/enemies.yaml");
    match enemy_file {
        Ok(f) => {
            let enemies: CharacterProperties = serde_yaml::from_reader(f).unwrap();
            for enemy in enemies.characters {
                model_map.insert(
                    enemy.name.clone(),
                    enemy.get_model(&asset_server, &mut texture_atlas_layouts),
                );
            }
        }
        Err(e) => error!("Failed to load enemies! {}", e),
    }

    commands.insert_resource(Models { models: model_map });
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_enemies);
    }
}
