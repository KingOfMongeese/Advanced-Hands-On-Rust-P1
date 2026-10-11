use bevy::prelude::*;
use my_library::RandomNumberGenerator;

use crate::resources::{GameAssets, HandTimer, Random, Scores};

pub fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut text_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d::default());

    let texture = asset_server.load("dice.ping");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(52), 6, 1, None, None);
    let texture_atlas_layout = text_atlas_layouts.add(layout);

    commands.insert_resource(GameAssets {
        image: texture,
        layout: texture_atlas_layout,
    });

    commands.insert_resource(Scores { player: 0, cpu: 0 });

    commands.insert_resource(Random(RandomNumberGenerator::new()));

    commands.insert_resource(HandTimer(Timer::from_seconds(0.5, TimerMode::Repeating)));
}
