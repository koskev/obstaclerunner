use bevy::prelude::*;
use bevy_2d_animations::{Animated, AnimatedSprite};
use bevy_rapier2d::geometry::Collider;
use serde::{Deserialize, Serialize};

use crate::{
    model::Model,
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

impl From<ColliderType> for Collider {
    fn from(value: ColliderType) -> Self {
        match value {
            ColliderType::CapsuleY(data) => Collider::capsule_y(data.height, data.radius),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct ColliderProperties {
    pub collider: ColliderType,
    #[serde(default)]
    pub translation: Vec2,
}

#[derive(Deserialize, Serialize, Default)]
pub struct AnimationProperties {
    pub name: String,
    // TODO: allow for range in yaml
    pub indices: Vec<usize>,
    pub speed: f32,
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
    animations: Vec<AnimationProperties>,
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
        let mut animation = AnimatedSprite::default();
        for anim_data in &self.animations {
            animation.add_animation(&anim_data.name, anim_data.indices.clone(), anim_data.speed);
        }
        animation.queue_animation("idle", true, None);
        let model = Model {
            animation,
            spritesheet: SpriteSheetBundle {
                texture: idle_set,
                atlas: TextureAtlas {
                    layout: texture_atlas_layout,
                    index: 0,
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
                    ..Default::default()
                })
                .collect(),
        };
        model
    }
}
