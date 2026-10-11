use bevy::prelude::*;
use my_library::RandomNumberGenerator;

#[derive(Resource)]
pub struct GameAssets {
    pub image: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

#[derive(Clone, Copy, Resource)]
pub struct Scores {
    pub player: usize,
    pub cpu: usize,
}

#[derive(Resource)]
pub struct Random(pub RandomNumberGenerator);

#[derive(Resource)]
pub struct HandTimer(pub Timer);
