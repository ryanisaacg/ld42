use super::*;
use quicksilver::geom::Shape;

pub fn system(window: &mut Window, assets: &Assets, store: &mut Store) -> Result<()> {
    window.clear(Color::WHITE)?;
    {
        let bounds = store.bounds[store.player].clone();
        let aabb = bounds.shape.aabb(&na::one());
        let rect: Rectangle = aabb.into();
        window.draw(&rect.translate(bounds.position), Background::Img(&assets.player));
    }
    {
        let bounds = store.bounds[store.walls].clone();
        let aabb = bounds.shape.aabb(&na::one());
        let rect: Rectangle = aabb.into();
        window.draw(&rect.translate(bounds.position), Background::Img(&assets.player));
    }
    Ok(())
}

