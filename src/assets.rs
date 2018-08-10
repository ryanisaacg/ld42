use super::*;

pub const IMAGES: &[&str] = &["astronaut.png"];
pub const SOUNDS: &[&str] = &[];

pub struct Assets {
    pub player: Image,
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
        }))
    }
}