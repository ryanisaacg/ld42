use super::*;

pub fn system(store: &mut Store) {
    for idx in 0..store.active.len() {
        let entity = store.active[idx];
        if let Some(timer) = &mut store.detonate_timer[entity] {
            *timer -= 1;
            if *timer == 0 {
                let center = store.bounds[entity].position;
                let rect = Rectangle::new_sized((0.64, 0.64)).with_center(center);
                let mut shapes = vec![
                    (vec_to_iso(center), ShapeHandle::new(rect_to_cuboid(rect)))
                ];
                {
                    let compound = store.bounds[store.walls].shape
                        .as_shape::<Compound<f32>>().unwrap();
                    shapes.extend(compound.shapes().iter().cloned());
                }
                store.bounds[store.walls] = Bounds {
                    position: Vector::ZERO,
                    shape: ShapeHandle::new(Compound::new(shapes))
                };
                store.deleted.push(idx);
            }
        }
    }
}