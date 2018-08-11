use super::*;

const WALK_ACCEL: f32 = 0.3;

pub fn system(window: &Window, store: &mut Store) {
    let wall_bounds = store.bounds[store.walls].clone();
    let wall_pos = vec_to_iso(wall_bounds.position);
    let wall_shape = wall_bounds.shape;
    let mut accel = Vector::ZERO;
    if window.keyboard()[Key::D].is_down() {
        accel.x += WALK_ACCEL;
    }
    if window.keyboard()[Key::A].is_down() {
        accel.x -= WALK_ACCEL;
    }
    store.accel[store.player] = Some(accel);
}
