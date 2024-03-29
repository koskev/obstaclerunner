use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_rapier2d::geometry::ActiveEvents;

use std::collections::{HashMap, VecDeque};

use crate::physics::ColliderChild;

#[derive(Clone, Debug, Default)]
pub struct AnimationData {
    /// Name of the Animation for identification
    pub name: String,
    /// Indices in the atlas for the animation
    pub indices: Vec<usize>,
    /// Saves the current atlas index
    pub current_index: usize, // TODO: separate common data and state?

    /// Timer the animation uses
    pub timer: Timer,
    /// Indicates weather the animation should loop or stay at the last frame
    pub looped: bool,
    // TODO: add changed collision based on animation?
    // Adding the sprite here would make adding it to bevy more complicated
}

#[derive(Component, Clone, Debug, Default)]
pub struct AnimatedSprite {
    // This could be a resource?
    pub animation_data: HashMap<String, AnimationData>,
    pub current_animation: AnimationData,
    pub animation_queue: VecDeque<String>,
}

pub trait Animated {
    fn add_animation(&mut self, name: &str, indices: Vec<usize>, speed: f32);
    fn queue_animation(&mut self, name: &str, looped: bool, follow_up: Option<Vec<String>>);
    fn update(&mut self, time: &Time<Virtual>, atlas: &mut TextureAtlas);
}

impl Animated for AnimatedSprite {
    fn add_animation(&mut self, name: &str, indices: Vec<usize>, speed: f32) {
        self.animation_data.insert(
            name.to_string(),
            AnimationData {
                name: name.to_string(),
                timer: Timer::from_seconds(speed, TimerMode::Repeating),
                indices,
                ..Default::default()
            },
        );
    }

    fn queue_animation(&mut self, name: &str, looped: bool, _follow_up: Option<Vec<String>>) {
        if let Some(animation) = self.animation_data.get(name) {
            self.current_animation = animation.clone();
            self.current_animation.looped = looped;
        }
    }

    fn update(&mut self, time: &Time<Virtual>, atlas: &mut TextureAtlas) {
        self.current_animation.timer.tick(time.delta());
        if self.current_animation.timer.just_finished() {
            // Check if we can increase the index without overflow
            if self.current_animation.current_index + 1 >= self.current_animation.indices.len() {
                // Set to 0 if we have looping enabled. Otherwise do nothing
                if self.current_animation.looped {
                    self.current_animation.current_index = 0;
                }
                // Increase if we are not at the target count
            } else {
                self.current_animation.current_index += 1;
            }
            if let Some(new_index) = self
                .current_animation
                .indices
                .get(self.current_animation.current_index)
            {
                atlas.index = *new_index;
            }
        }
    }
}

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

fn update_animations(
    mut q_animation: Query<(&mut AnimatedSprite, &mut TextureAtlas)>,
    time: Res<Time<Virtual>>,
) {
    for (mut sprite, mut atlas) in &mut q_animation {
        sprite.update(&time, &mut atlas);
    }
}

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_animations);
    }
}
