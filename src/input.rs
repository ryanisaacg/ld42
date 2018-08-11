use super::*;

const WALK_ACCEL: f32 = 0.3;

pub fn system(window: &Window, store: &mut Store) {
    let mut accel = Vector::new(0, store.accel[store.player].unwrap().y);
    if window.keyboard()[Key::D].is_down() {
        accel.x += WALK_ACCEL;
    }
    if window.keyboard()[Key::A].is_down() {
        accel.x -= WALK_ACCEL;
    }
    store.accel[store.player] = Some(accel);
}
