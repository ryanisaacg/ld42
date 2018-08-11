use super::*;
use quicksilver::geom::Shape;

pub fn system(window: &mut Window, assets: &Assets, store: &mut Store) -> Result<()> {
    window.clear(Color::WHITE)?;
    let scale = Transform::scale((100, 100));
    {
        let position = scale * store.bounds[store.player].position;
        let aabb = store.bounds[store.player].shape.aabb(&na::one());
        let rect: Rectangle = aabb.into();
        window.draw_ex(&rect.translate(position), Background::Img(&assets.player), scale, 0);
    }
    /*{
        let bounds = store.bounds[store.walls].clone();
        let aabb = bounds.shape.aabb(&na::one());
        let rect: Rectangle = aabb.into();
        window.draw(&rect.translate(bounds.position), Background::Img(&assets.player));
    }*/
    Ok(())
}

