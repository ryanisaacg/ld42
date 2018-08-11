use super::*;

// TODO: ncollide cache

pub fn system(store: &mut Store) {
    let wall_pos = vec_to_iso(store.bounds[store.walls].position);
    let dispatch = DefaultContactDispatcher::new();
    let mut id_gen = IdAllocator::new();
    // TODO: make this more cache-friendly
    for idx in 0..store.active.len() {
        let entity = store.active[idx];
        if let (Some(speed), Some(accel)) = (&mut store.speed[entity], &mut store.accel[entity]) {
            *speed += *accel;
        }
        if let Some(embed) = &mut store.embedded[entity] {
            let bounds = &store.bounds[entity];
            let pos = vec_to_iso(bounds.position);
            *embed = (Proximity::Intersecting == proximity(
                &pos,
                bounds.shape.as_ref(),
                &wall_pos,
                store.bounds[store.walls].shape.as_ref(),
                1.0
            ));
        }
        if let (Some(speed), Some(phys)) = (&mut store.speed[entity], &mut store.attr[entity]) {
            if let Some(true) = store.embedded[entity] {
                continue;   
            }
            let bounds = &mut store.bounds[entity];
            *speed = speed.clamp(-phys.speed_cap, phys.speed_cap);
            *speed *= phys.friction;
            bounds.position += *speed;
        }
        if let Some(collision) = &mut store.collisions[entity] {
            if let Some(true) = store.embedded[entity] {
                continue;   
            }
            let pos = vec_to_iso(store.bounds[entity].position);
            let mut contact_cache = Vec::new();
            collision.update(
                &dispatch,
                entity,
                &pos,
                store.bounds[entity].shape.as_ref(),
                store.walls,
                &wall_pos,
                store.bounds[store.walls].shape.as_ref(),
                &ContactPrediction::new(1.0, 0.0, 0.0),
                &mut id_gen
            );
            let touch = contact(
                &pos,
                store.bounds[entity].shape.as_ref(),
                &wall_pos,
                store.bounds[store.walls].shape.as_ref(),
                0.0
            );
            collision.contacts(&mut contact_cache);
            for contact in contact_cache.drain(..) {
                if let Some(deep) = contact.deepest_contact() {
                    let normal: Vector = deep.contact.normal.unwrap().into();
                    let penetration: Vector = (normal * deep.contact.depth).into();
                    store.bounds[entity].position -= penetration;
                }
            }
            contact_cache.clear();
        }
    }
}
