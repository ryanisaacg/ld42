use super::*;
use specs::Join;

pub struct PhysicsSystem<'a> {
    pub assets: &'a Assets,
    pub map: &'a Tilemap<i32>,
}

type PhysicsSystemData<'a> = (WriteStorage<'a, Bounds>,
    ReadStorage<'a, Speed>,
    ReadStorage<'a, PlayerTag>,
    ReadStorage<'a, WallsTag>);

impl<'a, 'b> System<'a> for PhysicsSystem<'b> {
    type SystemData = PhysicsSystemData<'a>;

    fn run(&mut self, mut data: PhysicsSystemData<'a>) {
        let (mut bounds, speed, player_tag, walls_tag) = data;
        let (wall_pos, wall_shape) = {
            let walls = (&bounds, &walls_tag).join().next().unwrap().0;
            (vec_to_iso(walls.position), walls.shape.clone())
        };
        for (bounds, speed, player_tag) in (&mut bounds, &speed, &player_tag).join() {
            bounds.position += speed.0;
            let pos = vec_to_iso(bounds.position);
            let collision = contact(&pos, bounds.shape.as_ref(), &wall_pos, wall_shape.as_ref(), 1.0);
            if let Some(collision) = collision {
                let normal: Vector = collision.normal.unwrap().into();
                let penetration: Vector = (normal * collision.depth).into();
                bounds.position -= penetration;
            }
        }
    }
}
