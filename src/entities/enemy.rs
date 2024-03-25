use bevy::prelude::*;

use crate::physics::ControllerBundle;

#[derive(Component, Default)]
pub struct Enemy;

#[derive(Bundle, Default)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub controller: ControllerBundle,
}
