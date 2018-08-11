use super::*;

pub fn system(store: &mut Store) {
    let wall_bounds = store.bounds[store.walls].clone();
    let wall_pos = vec_to_iso(wall_bounds.position);
    let wall_shape = wall_bounds.shape;
    for idx in 0..store.active.len() {
        let entity = store.active[idx];
        if let (Some(speed), Some(accel)) = (&mut store.speed[entity], &mut store.accel[entity]) {
            *speed += *accel;
        }
        if let (Some(speed), Some(phys)) = (&mut store.speed[entity], &mut store.attr[entity]) {
            let bounds = &mut store.bounds[entity];
            *speed = speed.clamp(-phys.speed_cap, phys.speed_cap);
            bounds.position += *speed;
            let pos = vec_to_iso(bounds.position);
            let collision = contact(&pos, bounds.shape.as_ref(), &wall_pos, wall_shape.as_ref(), 1.0);
            if let Some(collision) = collision {
                *speed *= phys.friction;
                let normal: Vector = collision.normal.unwrap().into();
                let penetration: Vector = (normal * collision.depth).into();
                bounds.position -= penetration;
            }
        }
    }
}
