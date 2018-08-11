use super::*;
use quicksilver::geom::Shape;

// This module handles all of the different components that go into
// each entity

// Everything in the game is composed of one or more of these

pub struct Bounds {
    pub position: Vector,
    pub shape: ShapeHandle<f32>
}

impl Component for Bounds {
    type Storage = DenseVecStorage<Self>;
}
impl Bounds {
    pub fn new(rect: Rectangle) -> Bounds {
        let position = rect.center();
        let half_extents = rect.size() / 2;
        let shape = ShapeHandle::new(Cuboid::new(na::Vector2::new(half_extents.x, half_extents.y)));
        Bounds {
            position,
            shape,
        }
    }
}

pub struct Speed(pub Vector);
impl Component for Speed {
    type Storage = DenseVecStorage<Self>;
}

pub struct Acceleration(pub Vector);
impl Component for Acceleration {
    type Storage = VecStorage<Self>;
}

pub struct PlayerTag;
impl Component for PlayerTag {
    type Storage = HashMapStorage<Self>;
}

pub struct WallsTag;
impl Component for WallsTag {
    type Storage = HashMapStorage<Self>;
}

