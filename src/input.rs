use super::*;
use specs::Join;

pub struct InputSystem<'a> {
    pub window: &'a mut Window,
}

type InputSystemData<'a> = (ReadStorage<'a, Bounds>,
    WriteStorage<'a, Speed>,
    WriteStorage<'a, Acceleration>,
    ReadStorage<'a, PlayerTag>,
    ReadStorage<'a, WallsTag>);

impl<'a, 'b> System<'a> for InputSystem<'b> {
    type SystemData = InputSystemData<'a>;

    fn run(&mut self, data: InputSystemData<'a>) {
        let (bounds, mut speed, mut accel, player_tag, walls_tag) = data;
        let (wall_pos, wall_shape) = {
            let walls = (&bounds, &walls_tag).join().next().unwrap().0;
            (vec_to_iso(walls.position), walls.shape.clone())
        };
        for (accel, _player_tag) in (&mut accel, &player_tag).join() {
            accel.0.x = 0.0;
            if self.window.keyboard()[Key::D].is_down() {
                accel.0.x += 0.2;
            }
            if self.window.keyboard()[Key::A].is_down() {
                accel.0.x -= 0.2;
            }
        }
    }
}
