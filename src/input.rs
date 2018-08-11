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
    if window.keyboard()[Key::Space] == ButtonState::Pressed {
        //TODO: only allow jumps on the ground
        let iso = vec_to_iso(store.bounds[store.walls].position);
        let ray = Ray::new(
            store.bounds[store.player].position.into(),
            na::Vector2::new(0.0, 0.02),
        );
        let touch = contact(
            &vec_to_iso(store.bounds[store.player].position),
            store.bounds[store.player].shape.as_ref(),
            &vec_to_iso(store.bounds[store.walls].position),
            store.bounds[store.walls].shape.as_ref(),
            0.001
        );
        match touch {
            Some(touch) => {
                if touch.normal.y > 0.0 {
                    let speed = store.speed[store.player].unwrap();
                    store.speed[store.player] = Some(Vector::new(speed.x, -0.12));
             
                }
            }
            None => ()
        }
    }
}
