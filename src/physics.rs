use super::*;
use specs::Join;

pub struct PhysicsSystem;

type PhysicsSystemData<'a> = (WriteStorage<'a, Bounds>,
    WriteStorage<'a, Speed>,
    ReadStorage<'a, Acceleration>,
    ReadStorage<'a, PhysicsAttr>,
    ReadStorage<'a, WallsTag>);

impl<'a> System<'a> for PhysicsSystem {
    type SystemData = PhysicsSystemData<'a>;

    fn run(&mut self, data: PhysicsSystemData<'a>) {
        let (mut bounds, mut speed, accel, phys, walls_tag) = data;
        let (wall_pos, wall_shape) = {
            let walls = (&bounds, &walls_tag).join().next().unwrap().0;
            (vec_to_iso(walls.position), walls.shape.clone())
        };
        for (speed, accel) in (&mut speed, &accel).join() {
            speed.0 += accel.0;
        }
        for (bounds, speed, phys) in (&mut bounds, &mut speed, &phys).join() {
            speed.0 = speed.0.clamp(-phys.speed_cap, phys.speed_cap);
            bounds.position += speed.0;
            let pos = vec_to_iso(bounds.position);
            let collision = contact(&pos, bounds.shape.as_ref(), &wall_pos, wall_shape.as_ref(), 1.0);
            if let Some(collision) = collision {
                speed.0 *= phys.friction;
                let normal: Vector = collision.normal.unwrap().into();
                let penetration: Vector = (normal * collision.depth).into();
                bounds.position -= penetration;
            }
        }
    }
}
