use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_2d_animations::AnimatedSprite;
use bevy_rapier2d::geometry::ActiveEvents;

use std::collections::HashMap;

use crate::physics::ColliderChild;

#[derive(Default, Clone)]
pub struct Model {
    pub spritesheet: SpriteSheetBundle,
    pub animation: AnimatedSprite,
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
