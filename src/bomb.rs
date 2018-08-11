use super::*;

pub fn system(store: &mut Store) {
    for idx in 0..store.active.len() {
        let entity = store.active[idx];
        if let Some(timer) = &mut store.detonate_timer[entity] {
            *timer -= 1;
            if *timer == 0 {
                store.deleted.push(idx);
            }
        }
    }
}