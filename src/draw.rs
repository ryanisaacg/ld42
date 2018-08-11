use super::*;
use quicksilver::geom::Shape;
use specs::Join;

pub struct DrawSystem<'a> {
    pub window: &'a mut Window,
    pub assets: &'a Assets,
    pub map: &'a Tilemap<i32>
}

type DrawSystemData<'a> = (ReadStorage<'a, Bounds>,
     ReadStorage<'a, PlayerTag>,
     ReadStorage<'a, WallsTag>);

impl<'a, 'b> System<'a> for DrawSystem<'b> {
    type SystemData = DrawSystemData<'a>;

    fn run(&mut self, data: DrawSystemData<'a>) {
        draw(self.window, self.assets, self.map, data).unwrap();
    }
}

fn draw<'a>(window: &mut Window, assets: &Assets, map: &'a Tilemap<i32>, data: DrawSystemData<'a>) -> Result<()> {
    window.clear(Color::WHITE)?;
    let (bounds, player_tag, walls_tag) = data;
    let mut x = 0.0;
    let mut y = 0.0;
    /*while x < map.width() {
        while y < map.height() {
            if let Some(_) = map.get((x, y)) {
                window.draw(&Rectangle::new((x, y), map.tile_size()), Background::Col(Color::BLACK));
            }
            y += map.tile_height();
        }
        x += map.tile_width();
    }*/
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

