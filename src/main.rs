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
    map: Tilemap<i32>,
}


impl State for Game {
    fn new() -> Result<Game> {
        let mut world = World::new();
        world.register::<Bounds>();
        world.register::<Speed>();
        world.register::<PlayerTag>();
        world.register::<WallsTag>();
        world.create_entity()
            .with(Bounds::new(Rectangle::new((0, 0), (32, 32))))
            .with(Speed(Vector::new(0, 3)))
            .with(PlayerTag)
            .build();
        world.create_entity()
            .with(Bounds::new(Rectangle::new((0, 500), (600, 100))))
            .with(WallsTag)
            .build();
        let mut map = Tilemap::new((800, 600), (32, 32));
        map.set((0, 100), Tile::solid(Some(1)));
        Ok(Game {
            assets: Assets::new(),
            world,
            map,
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        let world = &self.world;
        let map = &self.map;
        self.assets.execute(|assets| {
            PhysicsSystem { assets, map }.run_now(&world.res);
            Ok(())
        })
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;
        let world = &self.world;
        let map = &self.map;
        self.assets.execute(|assets| {
            let mut draw = DrawSystem {
                window,
                assets,
                map,
            };
            draw.run_now(&world.res);
            Ok(())
        })
    }
}

fn main() {
    run::<Game>(WindowBuilder::new("LD42 Entry!", (800, 600)));
}
