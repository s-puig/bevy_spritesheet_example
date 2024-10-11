mod atlas_asset_loading;
mod characters;

use crate::atlas_asset_loading::{AssetLoadingState, AtlasAssetLoadingPlugin};
use crate::characters::Characters;
use bevy::prelude::{
    App, Camera2dBundle, Commands, Component, Deref, DerefMut, ImagePlugin, OnEnter, PluginGroup,
    Query, Res, TextureAtlas, Time, Timer, Update,
};
use bevy::time::TimerMode;
use bevy::DefaultPlugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(AtlasAssetLoadingPlugin)
        .add_systems(OnEnter(AssetLoadingState::Loaded), spawn_characters)
        .add_systems(Update, animate_sprite)
        .run();
}

fn spawn_characters(mut commands: Commands, characters: Res<Characters>) {
    commands.spawn(Camera2dBundle::default());

    let animation_indices = AnimationIndices { first: 1, last: 6 };

    commands
        .spawn(
            characters
                .character("gabe.png".into())
                .expect("should exist"),
        )
        .insert(animation_indices)
        .insert(AnimationTimer(Timer::from_seconds(
            0.5,
            TimerMode::Repeating,
        )));

    // This is a bundle so we can freely edit before spawning
    let mut mani_bundle = characters
        .character("mani.png".into())
        .expect("should exist");
    mani_bundle.sprite.sprite.flip_x = true;
    mani_bundle.sprite.transform.translation.x += 300.0;

    commands
        .spawn(mani_bundle)
        .insert(animation_indices)
        .insert(AnimationTimer(Timer::from_seconds(
            0.5,
            TimerMode::Repeating,
        )));
}

// Everything below is taken from bevy spritesheet example
#[derive(Clone, Copy, Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}
