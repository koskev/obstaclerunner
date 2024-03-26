use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_rapier2d::geometry::ActiveEvents;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use crate::physics::ColliderChild;

#[derive(Component, Default, Clone, Serialize, Deserialize)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut, Default, Clone)]
pub struct AnimationTimer(pub Timer);

#[derive(Bundle, Default, Clone)]
pub struct AnimationBundle {
    pub indices: AnimationIndices,
    pub timer: AnimationTimer,
}

#[derive(Default, Clone)]
pub struct Model {
    pub spritesheet: SpriteSheetBundle,
    pub animation: AnimationBundle,
    pub colliders: Vec<ColliderChild>,
}

impl Model {
    pub fn spawn(&self, mut commands: EntityCommands) {
        commands
            .insert(self.spritesheet.clone())
            .insert(self.animation.clone())
            .with_children(|parent| {
                for collider in &self.colliders {
                    parent
                        .spawn(collider.clone())
                        .insert(ActiveEvents::COLLISION_EVENTS);
                }
            });
    }
}

#[derive(Resource, Default)]
pub struct Models {
    pub models: HashMap<String, Model>,
}

pub fn animate_sprites(
    time: Res<Time<Virtual>>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}
