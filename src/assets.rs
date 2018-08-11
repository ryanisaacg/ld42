use super::*;

pub const IMAGES: &[&str] = &[
    "astronaut.png",
    "astronaut_idle1.png",
    "astronaut_idle2.png",
    "astronaut_idle3.png",
    "astronaut_idle4.png",
    "astronaut_jump.png",
    "astronaut_walk1.png",
    "astronaut_walk2.png",
    "astronaut_walk3.png",
    "astronaut_walk4.png",
];
pub const SOUNDS: &[&str] = &[];

pub struct Assets {
    pub player: Image,
    pub player_idle: Animation,
    pub player_jump: Animation,
    pub player_walk: Animation,
}

// TODO: include this in quicksilver
fn from_sheet(image: Image, columns: u32, frame_delay: u32) -> Animation {
    let mut subimage_size = image.area().size();
    subimage_size.x = subimage_size.x / (columns as f32);
    let mut frames = Vec::new();
    for i in 0..columns {
        let view = Rectangle::new(Vector::X * subimage_size.x * i, subimage_size);
        frames.push(image.subimage(view));
    }
    Animation::new(frames, frame_delay)
}

impl Assets {
    pub fn new() -> Asset<Assets> {
        let image_loader = join_all(IMAGES
            .iter()
            .cloned()
            .map(Image::load))
            .map(|loaded: Vec<Image>| IMAGES
                .iter()
                .cloned()
                .zip(loaded)
                .collect::<HashMap<&'static str, Image>>());
        let sound_loader = join_all(SOUNDS
            .iter()
            .cloned()
            .map(Sound::load))
            .map(|loaded: Vec<Sound>| SOUNDS
                .iter()
                .cloned()
                .zip(loaded)
                .collect::<HashMap<&'static str, Sound>>());
        let loaders = image_loader.join(sound_loader);
        Asset::new(loaders.map(|(images, sounds)| Assets {
            player: images["astronaut.png"].clone(),
            player_idle: Animation::new(vec![
                images["astronaut_idle1.png"].clone(),
                images["astronaut_idle2.png"].clone(),
                images["astronaut_idle3.png"].clone(),
                images["astronaut_idle4.png"].clone(),
            ], 11),
            player_jump: Animation::new(vec![
                images["astronaut_jump.png"].clone(),
            ], 11),
            player_walk: Animation::new(vec![
                images["astronaut_walk1.png"].clone(),
                images["astronaut_walk2.png"].clone(),
                images["astronaut_walk3.png"].clone(),
                images["astronaut_walk4.png"].clone(),
            ], 11),
        }))
    }
}