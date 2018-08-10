use super::*;

// This module handles all of the different components that go into
// each entity

// Everything in the game is composed of one or more of these

pub struct Bounds(pub Rectangle);
impl Component for Bounds {
    type Storage = DenseVecStorage<Self>;
}

pub struct Speed(pub Vector);
impl Component for Speed {
    type Storage = DenseVecStorage<Self>;
}

pub struct Acceleration(pub Vector);
impl Component for Acceleration {
    type Storage = VecStorage<Self>;
}

pub struct PlayerTag;
impl Component for PlayerTag {
    type Storage = HashMapStorage<Self>;
}