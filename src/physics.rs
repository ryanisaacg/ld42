use super::*;
use specs::Join;

pub struct PhysicsSystem<'a> {
    pub assets: &'a Assets,
    pub map: &'a Tilemap<i32>,
}

type PhysicsSystemData<'a> = (WriteStorage<'a, Bounds>,
    WriteStorage<'a, Speed>,
    ReadStorage<'a, Acceleration>,
    ReadStorage<'a, PlayerTag>,
    ReadStorage<'a, WallsTag>);

impl<'a, 'b> System<'a> for PhysicsSystem<'b> {
    type SystemData = PhysicsSystemData<'a>;

    fn run(&mut self, data: PhysicsSystemData<'a>) {
        let (mut bounds, mut speed, accel, player_tag, walls_tag) = data;
        let (wall_pos, wall_shape) = {
            let walls = (&bounds, &walls_tag).join().next().unwrap().0;
            (vec_to_iso(walls.position), walls.shape.clone())
        };
        for (speed, accel) in (&mut speed, &accel).join() {
            speed.0 += accel.0;
        }
        for (bounds, speed, player_tag) in (&mut bounds, &mut speed, &player_tag).join() {
            bounds.position += speed.0;
            let pos = vec_to_iso(bounds.position);
            let collision = contact(&pos, bounds.shape.as_ref(), &wall_pos, wall_shape.as_ref(), 1.0);
            if let Some(collision) = collision {
                // TODO: sliding doesn't work then
                speed.0 = Vector::ZERO;
                let normal: Vector = collision.normal.unwrap().into();
                let penetration: Vector = (normal * collision.depth).into();
                bounds.position -= penetration;
            }
        }
    }
}
