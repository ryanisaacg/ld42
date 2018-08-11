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

    let position = store.bounds[store.player].position;
    if window.keyboard()[Key::Down] == ButtonState::Released {
        toss_bomb(store, position, Vector::Y * 0.3);
    }
    if window.keyboard()[Key::Up] == ButtonState::Released {
        toss_bomb(store, position, Vector::Y * -0.3);
    }
    if window.keyboard()[Key::Left] == ButtonState::Released {
        toss_bomb(store, position, Vector::new(-0.5, -0.04));
    }
    if window.keyboard()[Key::Right] == ButtonState::Released {
        toss_bomb(store, position, Vector::new(0.5, -0.04));
    }
}

fn toss_bomb(store: &mut Store, center: Vector, initial_speed: Vector) {
    let rectangle = Rectangle::new((0, 0), (0.16, 0.16)).with_center(center);
    let bomb = store.spawn(Bounds::new(rectangle));
    store.speed[bomb] = Some(initial_speed);
    store.accel[bomb] = Some(Vector::new(0, 0.003));
    store.attr[bomb] = Some(PhysicsAttr {
        speed_cap: Vector::new(0.06, 0.12),
        friction: 1.0,
    });
    store.collisions[bomb] = Some(CompositeShapeShapeManifoldGenerator::new(true));
    store.detonate_timer[bomb] = Some(59);
}