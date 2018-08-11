use super::*;

pub fn system(store: &mut Store) {
    let mut offset = 0;
    for i in 0..store.deleted.len() {
        let idx = store.deleted[i] - offset;
        store.despawn(idx);
        offset += 1;
    }
    store.deleted.clear();
}