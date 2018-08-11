use super::*;
use quicksilver::geom::Shape;
use specs::Join;

pub struct DrawSystem<'a> {
    pub window: &'a mut Window,
    pub assets: &'a Assets,
}

type DrawSystemData<'a> = (ReadStorage<'a, Bounds>,
     ReadStorage<'a, PlayerTag>,
     ReadStorage<'a, WallsTag>);

impl<'a, 'b> System<'a> for DrawSystem<'b> {
    type SystemData = DrawSystemData<'a>;

    fn run(&mut self, data: DrawSystemData<'a>) {
        draw(self.window, self.assets, data).unwrap();
    }
}

fn draw<'a>(window: &mut Window, assets: &Assets, data: DrawSystemData<'a>) -> Result<()> {
    window.clear(Color::WHITE)?;
    let (bounds, player_tag, walls_tag) = data;
    for (bounds, _player_tag) in (&bounds, &player_tag).join() {
        let aabb = bounds.shape.aabb(&na::one());
        let rect: Rectangle = aabb.into();
        window.draw(&rect.translate(bounds.position), Background::Img(&assets.player));
    }
    for (bounds, _walls_tag) in (&bounds, &walls_tag).join() {
        let aabb = bounds.shape.aabb(&na::one());
        let rect: Rectangle = aabb.into();
        window.draw(&rect.translate(bounds.position), Background::Img(&assets.player));
    }
    Ok(())
}

