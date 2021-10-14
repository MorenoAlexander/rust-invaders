extern crate amethyst;

use self::amethyst::{SimpleState, StateData, GameData, StateEvent, SimpleTrans};
use self::amethyst::core::ecs::Entity;
use self::amethyst::core::Transform;
use self::amethyst::prelude::*;
use amethyst::renderer::Camera;
use self::amethyst::assets::AssetLoaderSystemData;
use self::amethyst::renderer::rendy::mesh::{Position, TexCoord, Tangent};
use self::amethyst::renderer::{MaterialDefaults, Material, Mesh,};
use self::amethyst::renderer::rendy::util::types::vertex::Normal;
use self::amethyst::renderer::shape::Shape;
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
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
       intialize_camera(_data.world)
    }
}

fn intialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0,0.0,10.0);

    world.create_entity()
        .with(Camera::standard_3d(1024.0, 768.0))
        .with(transform)
        .build();
}

fn initialize_player(world: &mut World) {

}

fn intialize_simple_3d_object(world : &mut World) {
    let mesh = world.exec(|loader: AssetLoaderSystemData<'_, Mesh>| {
        loader.load_from_data(Shape::Sphere(100,100).generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(None)
                                  .into(),
                              ()
        )
    });
    let material_defaults = world.read_resource::<MaterialDefaults>().0.clone();
    let material = world.exec(|loader: AssetLoaderSystemData<'_, Material>| {
        loader.load_from_data(Material {
            ..material_defaults
        },
                              (),
        )
    },);

    world.create_entity().with(mesh).with(material).build();

}



