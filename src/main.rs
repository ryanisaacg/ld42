extern crate quicksilver;
#[macro_use]
extern crate specs_derive;
extern crate specs;

use {
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

struct Game {
    assets: Asset<Assets>,
    world: World,
}

impl State for Game {
    fn new() -> Result<Game> {
        let mut world = World::new();
        world.register::<Bounds>();
        world.register::<PlayerTag>();
        world.create_entity()
            .with(Bounds(Rectangle::new((0, 0), (32, 32))))
            .with(PlayerTag)
            .build();
        Ok(Game {
            assets: Assets::new(),
            world,
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        let world = &self.world;
        self.assets.execute(|assets| {
            PhysicsSystem { assets }.run_now(&world.res);
            Ok(())
        })
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;
        let world = &self.world;
        self.assets.execute(|assets| {
            let mut draw = DrawSystem {
                window,
                assets,
            };
            draw.run_now(&world.res);
            Ok(())
        })
    }
}

fn main() {
    run::<Game>(WindowBuilder::new("LD42 Entry!", (800, 600)));
}
