use super::*;
use quicksilver::geom::Shape;

pub fn system(window: &mut Window, assets: &Assets, store: &mut Store) -> Result<()> {
    window.clear(Color::WHITE)?;
    {
        let position = store.bounds[store.player].position;
        let aabb = store.bounds[store.player].shape.aabb(&na::one());
        let rect: Rectangle = aabb.into();
        window.draw(&rect.translate(position), Background::Img(&assets.player));
    }
    /*{
        let bounds = store.bounds[store.walls].clone();
        let aabb = bounds.shape.aabb(&na::one());
        let rect: Rectangle = aabb.into();
        window.draw(&rect.translate(bounds.position), Background::Img(&assets.player));
    }*/
    Ok(())
}

