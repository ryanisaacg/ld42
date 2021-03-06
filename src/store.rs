use super::*;
use quicksilver::geom::Shape;

pub struct Store {
    pub active: Vec<usize>,
    pub collision_id: Vec<GenerationalId>,
    pub bounds: Vec<Bounds>,
    pub speed: Vec<Option<Vector>>,
    pub accel: Vec<Option<Vector>>,
    pub attr: Vec<Option<PhysicsAttr>>,
    pub collisions: Vec<Option<CompositeShapeShapeManifoldGenerator<f32>>>,
    pub embedded: Vec<Option<bool>>,
    pub flip: Vec<bool>,
    pub supported: Vec<bool>,
    pub detonate_timer: Vec<Option<u32>>,
    pub player: usize,
    pub walls: usize,
    pub id_alloc: IdAllocator,
    pub deleted: Vec<usize>,
    next_id: usize,
}

impl Store {
    pub fn new() -> Store {
        Store {
            active: Vec::new(),
            collision_id: Vec::new(),
            bounds: Vec::new(),
            speed: Vec::new(),
            accel: Vec::new(),
            attr: Vec::new(),
            collisions: Vec::new(),
            embedded: Vec::new(),
            flip: Vec::new(),
            supported: Vec::new(),
            detonate_timer: Vec::new(),
            deleted: Vec::new(),
            player: 0,
            walls: 0,
            next_id: 0,
            id_alloc: IdAllocator::new(),
        }
    }

    pub fn spawn(&mut self, bounds: Bounds) -> usize {
        self.active.push(self.next_id);
        self.collision_id.push(self.id_alloc.alloc());
        let id = self.next_id;
        self.next_id += 1;
        self.bounds.push(bounds);
        self.speed.push(None);
        self.accel.push(None);
        self.attr.push(None);
        self.collisions.push(None);
        self.embedded.push(None);
        self.flip.push(false);
        self.supported.push(false);
        self.detonate_timer.push(None);
        id
    }

    pub fn despawn(&mut self, idx: usize) {
        let entity = self.active.remove(idx);
        self.speed[entity] = None;
        self.accel[entity] = None;
        self.attr[entity] = None;
        self.collisions[entity] = None;
        self.embedded[entity] = None;
        self.detonate_timer[entity] = None;
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
        let shape = ShapeHandle::new(rect_to_cuboid(rect));
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