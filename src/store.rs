use super::*;
use quicksilver::geom::Shape;

pub struct Store {
    pub active: Vec<usize>,
    pub bounds: Vec<Bounds>,
    pub speed: Vec<Option<Vector>>,
    pub accel: Vec<Option<Vector>>,
    pub attr: Vec<Option<PhysicsAttr>>,
    pub player: usize,
    pub walls: usize,
    next_id: usize,
}

impl Store {
    pub fn new() -> Store {
        Store {
            active: Vec::new(),
            bounds: Vec::new(),
            speed: Vec::new(),
            accel: Vec::new(),
            attr: Vec::new(),
            player: 0,
            walls: 0,
            next_id: 0,
        }
    }

    pub fn spawn(&mut self, bounds: Bounds) -> usize {
        self.active.push(self.next_id);
        let id = self.next_id;
        self.next_id += 1;
        self.bounds.push(bounds);
        self.speed.push(None);
        self.accel.push(None);
        self.attr.push(None);
        id
    }
}

#[derive(Clone)]
pub struct Bounds {
    pub position: Vector,
    pub shape: ShapeHandle<f32>
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

pub struct PhysicsAttr {
    pub speed_cap: Vector,
    pub friction: f32,
}