use super::*;

pub fn system(store: &mut Store) {
    for idx in 0..store.active.len() {
        if idx == store.walls { continue; }
        let entity = store.active[idx];
        let iso = vec_to_iso(store.bounds[store.walls].position);
        let ray = Ray::new(
            store.bounds[entity].position.into(),
            na::Vector2::new(0.0, 0.02),
        );
        let touch = contact(
            &vec_to_iso(store.bounds[entity].position),
            store.bounds[entity].shape.as_ref(),
            &vec_to_iso(store.bounds[store.walls].position),
            store.bounds[store.walls].shape.as_ref(),
            0.001
        );
        store.supported[entity] = false;
        match touch {
            Some(touch) => {
                store.supported[entity] = touch.normal.y > 0.0;
            }
            None => ()
        }
    }
}