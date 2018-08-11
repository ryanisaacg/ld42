use super::*;
use quicksilver::geom::Shape;

// TODO: Background with an animation
// TODO: are subimages broken??

pub fn system(window: &mut Window, assets: &mut Assets, store: &mut Store) -> Result<()> {
    window.clear(Color::WHITE)?;
    let scale = Transform::scale((100, 100));
    {
        let position = scale * store.bounds[store.player].position;
        let aabb = store.bounds[store.player].shape.aabb(&na::one());
        let rect: Rectangle = aabb.into();
        let animation = if !store.supported[store.player] {
            &mut assets.player_jump
        } else if store.accel[store.player].unwrap().x == 0.0 {
            &mut assets.player_idle
        } else {
            &mut assets.player_walk
        };
        animation.tick();
        let frame = animation.current_frame();
        let flip = if store.flip[store.player] {
            Transform::scale((-1, 1))
        } else {
            Transform::IDENTITY
        };
        window.draw_ex(&rect.translate(position), Background::Img(&frame), flip * scale, 0);
    }
    /*{
        let bounds = store.bounds[store.walls].clone();
        let aabb = bounds.shape.aabb(&na::one());
        let rect: Rectangle = aabb.into();
        window.draw(&rect.translate(bounds.position), Background::Img(&assets.player));
    }*/
    Ok(())
}

