use super::*;
use specs::Join;

pub struct PhysicsSystem<'a> {
    pub assets: &'a Assets,
}

type PhysicsSystemData<'a> = (WriteStorage<'a, Bounds>,
     ReadStorage<'a, PlayerTag>);

impl<'a, 'b> System<'a> for PhysicsSystem<'b> {
    type SystemData = PhysicsSystemData<'a>;

    fn run(&mut self, mut data: PhysicsSystemData<'a>) {
        let (mut bounds, player_tag) = data;
        for (bounds, player_tag) in (&mut bounds, &player_tag).join() {
            bounds.0 = bounds.0.translate((0.01, 0.01));
        }
    }
}
