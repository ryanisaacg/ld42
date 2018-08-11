//TODO: nalgebra isometry conversion with quicksilver

extern crate nalgebra as na;
extern crate ncollide2d;
extern crate quicksilver;

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
    std::collections::HashMap
};

mod assets;
use assets::Assets;
mod draw;
mod input;
mod physics;
mod store;
use store::*;

// TODO: include in quicksilver
fn vec_to_iso(vec: Vector) -> na::Isometry2<f32> {
    let vec = na::Vector2::new(vec.x, vec.y);
    na::Isometry2::new(vec, na::zero())
}

struct Game {
    assets: Asset<Assets>,
    store: Store,
}


impl State for Game {
    fn new() -> Result<Game> {
        let mut store = Store::new();
        store.player = store.spawn(Bounds::new(Rectangle::new((0, 0), (32, 32))));
        store.speed[store.player] = Some(Vector::new(0, 0));
        store.accel[store.player] = Some(Vector::new(0, 0.1));
        store.attr[store.player] = Some(PhysicsAttr {
            speed_cap: Vector::new(6, 12),
            friction: 0.9,
        });
        store.walls = store.spawn(Bounds::new(Rectangle::new((0, 500), (600, 100))));
        Ok(Game {
            assets: Assets::new(),
            store,
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        let store = &mut self.store;
        self.assets.execute(|_| {
            input::system(window, store);
            physics::system(store);
            Ok(())
        })
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;
        let store = &mut self.store;
        self.assets.execute(|assets| {
            draw::system(window, assets, store)
        })
    }
}

fn main() {
    run::<Game>(WindowBuilder::new("LD42 Entry!", (800, 600)));
}
