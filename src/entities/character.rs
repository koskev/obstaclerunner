use bevy::prelude::*;
use bevy_rapier2d::geometry::Collider;
use serde::{Deserialize, Serialize};

use crate::{
    animation::{AnimationBundle, AnimationIndices, AnimationTimer, Model},
    physics::{ColliderChild, ControllerBundle},
};

#[derive(Component, Default)]
pub struct Character;

#[derive(Bundle, Default)]
pub struct CharacterBundle {
    pub animation_state: CharacterAnimationState,

    pub controller: ControllerBundle,

    pub character: Character,
}

#[derive(Component, Default, Hash, PartialEq, Eq, Debug)]
pub enum CharacterAnimationState {
    #[default]
    Running,
    Jumping,
    Ducking,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct CapsuleColliderY {
    pub radius: f32,
    pub height: f32,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(tag = "type")]
pub enum ColliderType {
    #[serde(rename = "capsule_y")]
    CapsuleY(CapsuleColliderY),
}

impl Into<Collider> for ColliderType {
    fn into(self) -> Collider {
        match self {
            Self::CapsuleY(data) => Collider::capsule_y(data.height, data.radius),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct ColliderProperties {
    pub collider: ColliderType,
    #[serde(default)]
    pub translation: Vec2,
}

#[derive(Deserialize, Serialize, Asset, TypePath, Default)]
#[serde(default)]
pub struct CharacterProperty {
    pub name: String,
    file_path: String,
    tile_size: Vec2,
    rows: usize,
    columns: usize,
    padding: Vec2,
    offset_tiles: Vec2,
    offset_fixed: Vec2,
    flip_x: bool,
    animation_indices: AnimationIndices,
    animation_speed: f32,
    colliders: Vec<ColliderProperties>,
}

#[derive(Deserialize, Serialize, Asset, TypePath)]
pub struct CharacterProperties {
    pub characters: Vec<CharacterProperty>,
}

impl CharacterProperty {
    pub fn get_model(
        &self,
        asset_server: &Res<AssetServer>,
        texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    ) -> Model {
        let idle_set = asset_server.load(self.file_path.clone());
        let layout = TextureAtlasLayout::from_grid(
            self.tile_size,
            self.columns,
            self.rows,
            Some(self.padding),
            Some(self.offset_fixed + self.offset_tiles * self.tile_size),
        );
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let model = Model {
            animation: AnimationBundle {
                indices: self.animation_indices.clone(),
                timer: AnimationTimer(Timer::from_seconds(
                    self.animation_speed,
                    TimerMode::Repeating,
                )),
            },
            spritesheet: SpriteSheetBundle {
                texture: idle_set,
                atlas: TextureAtlas {
                    layout: texture_atlas_layout,
                    index: self.animation_indices.first,
                },
                sprite: Sprite {
                    flip_x: self.flip_x,
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(0.0, 16.0, 0.0)),
                ..Default::default()
            },
            colliders: self
                .colliders
                .iter()
                .map(|collider_props| ColliderChild {
                    collider: collider_props.collider.clone().into(),
                    transform: TransformBundle {
                        local: Transform::from_translation(collider_props.translation.extend(0.0)),
                        ..Default::default()
                    },
                })
                .collect(),
        };
        model
    }
}
