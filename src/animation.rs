use bevy::{ecs::system::EntityCommands, prelude::*};

use std::collections::HashMap;

use crate::physics::ColliderChild;

#[derive(Component, Default, Clone)]
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
    pub collider: ColliderChild,
}

impl Model {
    pub fn spawn(&self, mut commands: EntityCommands) {
        commands
            .insert(self.spritesheet.clone())
            .insert(self.animation.clone())
            .with_children(|parent| {
                parent.spawn(self.collider.clone());
            });
    }
}

#[derive(Resource, Default)]
pub struct Models {
    pub models: HashMap<String, Model>,
}

pub fn animate_sprites(
    time: Res<Time>,
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
