extern crate amethyst;

use self::amethyst::{SimpleState, StateData, GameData, StateEvent, SimpleTrans};
use self::amethyst::core::ecs::Entity;
use self::amethyst::core::Transform;
use self::amethyst::prelude::World;

// constants

// ARENA CONSTS
pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

// enemies

// player





#[derive(Default)]
pub struct RustInvaders {
    enemies: Option<f32>
}


#[derive(Default)]
pub struct ScoreBoard {
    pub score: i32,
    pub time_left: i32,
    pub enemies_destroyed: i32
}

pub struct ScoreText {
    pub score_text: Entity
}


impl SimpleState for RustInvaders {
}

fn intialize_camera(world: &mut World) {
    let mut transform = Transform::default();

}

fn initialize_player(world: &mut World) {
}



