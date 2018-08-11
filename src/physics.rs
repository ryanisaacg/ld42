use super::*;
use specs::Join;

pub struct PhysicsSystem<'a> {
    pub assets: &'a Assets,
    pub map: &'a Tilemap<i32>,
}

type PhysicsSystemData<'a> = (ReadStorage<'a, Bounds>,
     ReadStorage<'a, PlayerTag>,
     ReadStorage<'a, WallsTag>);

impl<'a, 'b> System<'a> for PhysicsSystem<'b> {
    type SystemData = PhysicsSystemData<'a>;

    fn run(&mut self, mut data: PhysicsSystemData<'a>) {
        let (mut bounds, player_tag) = data;
        let (mut bounds, walls_tag) = data;
        let mut walls = (&bounds, &walls_tag).join().next().unwrap().0;
        let isometry = na::Isometry2::new(walls.into(), 0.0);
        for (bounds, player_tag) in (&mut bounds, &player_tag).join() {
            bounds.position += Vector::new(1, 1);
        }
    }
}
