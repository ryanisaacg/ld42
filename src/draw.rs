use super::*;
use specs::Join;

pub struct DrawSystem<'a> {
    pub window: &'a mut Window,
    pub assets: &'a Assets,
}

type DrawSystemData<'a> = (ReadStorage<'a, Bounds>,
     ReadStorage<'a, PlayerTag>);

impl<'a, 'b> System<'a> for DrawSystem<'b> {
    type SystemData = DrawSystemData<'a>;

    fn run(&mut self, data: DrawSystemData<'a>) {
        draw(self.window, self.assets, data).unwrap();
    }
}

fn draw<'a>(window: &mut Window, assets: &Assets, data: DrawSystemData<'a>) -> Result<()> {
    window.clear(Color::WHITE)?;
    let (bounds, player_tag) = data;
    for (bounds, player_tag) in (&bounds, &player_tag).join() {
        window.draw(&bounds.0, Background::Img(&assets.player));
    }
    Ok(())
}