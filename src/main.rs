//TODO: nalgebra isometry conversion with quicksilver

extern crate nalgebra as na;
extern crate ncollide2d;
extern crate quicksilver;

use {
    ncollide2d::{
        bounding_volume::*,
        narrow_phase::*,
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
use quicksilver::geom::Shape as brbr;

mod assets;
use assets::Assets;
mod bomb;
mod caching;
mod cleanup;
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

fn rect_to_cuboid(rect: Rectangle) -> Cuboid<f32> {
    let half_extents = rect.size() / 2;
    Cuboid::new(na::Vector2::new(half_extents.x, half_extents.y))
}

struct Game {
    assets: Asset<Assets>,
    store: Store,
}


impl State for Game {
    fn new() -> Result<Game> {
        let mut store = Store::new();
        store.player = store.spawn(Bounds::new(Rectangle::new((0, 0), (0.32, 0.32))));
        store.speed[store.player] = Some(Vector::new(0, 0));
        store.accel[store.player] = Some(Vector::new(0, 0.003));
        store.attr[store.player] = Some(PhysicsAttr {
            speed_cap: Vector::new(0.06, 0.12),
            friction: 0.9,
        });
        store.collisions[store.player] = Some(CompositeShapeShapeManifoldGenerator::new(true));
        store.embedded[store.player] = Some(false);
        let wall_bounds = vec![
            Rectangle::new((0, 5), (6, 1)),
            Rectangle::new((2, 3), (2.3, 5.5))
        ].into_iter()
            .map(|rect| (vec_to_iso(rect.center()), ShapeHandle::new(rect_to_cuboid(rect))))
            .collect();
        store.walls = store.spawn(Bounds {
            position: Vector::ZERO,
            shape: ShapeHandle::new(Compound::new(wall_bounds))
        });
        Ok(Game {
            assets: Assets::new(),
            store,
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        let store = &mut self.store;
        self.assets.execute(|_| {
            bomb::system(store);
            caching::system(store);
            input::system(window, store);
            physics::system(store);
            cleanup::system(store);
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
    run::<Game>(WindowBuilder::new("LD42 Entry!", (960, 540)));
}
