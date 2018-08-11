use super::*;

const WALK_ACCEL: f32 = 0.003;

pub fn system(window: &Window, store: &mut Store) {
    let mut accel = Vector::new(0, store.accel[store.player].unwrap().y);
    if window.keyboard()[Key::D].is_down() {
        accel.x += WALK_ACCEL;
        store.flip[store.player] = false;
    }
    if window.keyboard()[Key::A].is_down() {
        accel.x -= WALK_ACCEL;
        store.flip[store.player] = true;
    }
    store.accel[store.player] = Some(accel);
    if window.keyboard()[Key::Space] == ButtonState::Pressed && store.supported[store.player] {
        let speed = store.speed[store.player].unwrap();
        store.speed[store.player] = Some(Vector::new(speed.x, -0.12));
    }
}
