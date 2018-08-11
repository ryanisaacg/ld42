//TODO: nalgebra isometry conversion with quicksilver

extern crate nalgebra as na;
extern crate ncollide2d;
extern crate quicksilver;
#[macro_use]
extern crate specs_derive;
extern crate specs;

use {
    ncollide2d::{
        bounding_volume::*,
        partitioning::*,
        procedural::*,
        query::*,
        shape::{*, Shape},
        transformation::*,
        utils::*,
    },
    quicksilver::{
        *,
        combinators::*,
        geom::*,
        graphics::*,
        lifecycle::*,
        input::*,
        sound::*,
    },
    specs::*,
    std::collections::HashMap
};

mod assets;
use assets::*;
mod draw;
use draw::*;
mod components;
use components::*;
mod input;
use input::*;
mod physics;
use physics::*;

// TODO: include in quicksilver
fn vec_to_iso(vec: Vector) -> na::Isometry2<f32> {
    let vec = na::Vector2::new(vec.x, vec.y);
    na::Isometry2::new(vec, na::zero())
}

struct Game {
    assets: Asset<Assets>,
    world: World,
}


impl State for Game {
    fn new() -> Result<Game> {
        let mut world = World::new();
        world.register::<Bounds>();
        world.register::<Speed>();
        world.register::<Acceleration>();
        world.register::<PlayerTag>();
        world.register::<WallsTag>();
        world.create_entity()
            .with(Bounds::new(Rectangle::new((0, 0), (32, 32))))
            .with(Speed(Vector::new(0, 0)))
            .with(Acceleration(Vector::new(0, 0.1)))
            .with(PlayerTag)
            .build();
        world.create_entity()
            .with(Bounds::new(Rectangle::new((0, 500), (600, 100))))
            .with(WallsTag)
            .build();
        Ok(Game {
            assets: Assets::new(),
            world,
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        let world = &self.world;
        self.assets.execute(|assets| {
            InputSystem { window }.run_now(&world.res);
            PhysicsSystem.run_now(&world.res);
            Ok(())
        })
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;
        let world = &self.world;
        self.assets.execute(|assets| {
            let mut draw = DrawSystem { window, assets };
            draw.run_now(&world.res);
            Ok(())
        })
    }
}

fn main() {
    run::<Game>(WindowBuilder::new("LD42 Entry!", (800, 600)));
}
