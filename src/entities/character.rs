use bevy::prelude::*;

use crate::{physics::ControllerBundle, CharacterAnimationState};

#[derive(Component, Default)]
pub struct Character;

#[derive(Bundle, Default)]
pub struct CharacterBundle {
    pub animation_state: CharacterAnimationState,

    pub controller: ControllerBundle,

    pub character: Character,
}
