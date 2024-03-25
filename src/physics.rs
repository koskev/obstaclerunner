use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Bundle, Clone)]
pub struct RigidBodyBundle {
    pub body: RigidBody,
    /// Group to process collision responses with
    pub solver_group: SolverGroups,
    /// Group to process collision events with
    pub collision_group: CollisionGroups,
    pub mass: ColliderMassProperties,
    pub friction: Friction,
    pub velocity: Velocity,
    /// Slowing down of a body. Like Air resistance
    pub damping: Damping,
    /// If two bodies with a differend dominance value collide, the one with the higher dominance
    /// will act as if it has an infinite mass. Good for characters using a dynamic body
    pub dominance: Dominance,

    pub collider: Collider,
}

#[derive(Bundle, Clone, Default)]
pub struct ColliderChild {
    pub collider: Collider,
    pub transform: TransformBundle,
}

impl Default for RigidBodyBundle {
    fn default() -> Self {
        Self {
            body: RigidBody::default(),
            velocity: Velocity::default(),
            collider: Collider::default(),
            mass: ColliderMassProperties::default(),
            friction: Friction::default(),
            solver_group: SolverGroups::default(),
            collision_group: CollisionGroups::new(
                CollisionGroup::Common.group(),
                CollisionGroup::Common.group(),
            ),
            damping: Damping::default(),
            dominance: Dominance::default(),
        }
    }
}

#[derive(Bundle)]
pub struct ControllerBundle {
    pub body: RigidBody,
    pub velocity: Velocity,
    pub friction: Friction,
    pub group: CollisionGroups,
}
impl Default for ControllerBundle {
    fn default() -> Self {
        Self {
            body: RigidBody::Dynamic,
            velocity: Velocity::zero(),
            friction: Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            group: CollisionGroups::new(
                CollisionGroup::Common.group(),
                CollisionGroup::Common.group(),
            ),
        }
    }
}

pub enum CollisionGroup {
    Common,
    Player,
    Enemy,
    Wall,
    Spell,
    Collectible,
    All,
    None,
}

impl CollisionGroup {
    pub fn group(&self) -> Group {
        match self {
            Self::Common => Group::GROUP_1,
            Self::Player => Group::GROUP_2,
            Self::Enemy => Group::GROUP_3,
            Self::Wall => Group::GROUP_4,
            Self::Spell => Group::GROUP_5,
            Self::Collectible => Group::GROUP_6,
            Self::All => Group::ALL,
            Self::None => Group::NONE,
        }
    }
}
