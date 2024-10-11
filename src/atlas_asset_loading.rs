use crate::characters::{CharacterRawData, Characters};
use bevy::app::App;
use bevy::asset::{AssetServer, LoadState};
use bevy::math::UVec2;
use bevy::prelude::{
    in_state, AppExtStates, Assets, Commands, Handle, Image, IntoSystemConfigs, NextState, OnEnter,
    Plugin, Res, ResMut, Resource, States, TextureAtlasBuilder, Update,
};
use bevy::sprite::TextureAtlasLayout;
use bevy::utils::hashbrown::HashMap;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub enum AssetLoadingState {
    #[default]
    Setup,
    Loaded,
}

// Handles are stored here during the loading process and emptied after the atlas is generated
#[derive(Resource)]
struct LoadingAssets(pub Vec<Handle<Image>>);

// Start loading the textures
fn load_assets(mut command: Commands, asset_server: Res<AssetServer>) {
    let mut loading_assets = LoadingAssets(Vec::new());
    loading_assets.0.push(asset_server.load("gabe.png"));
    loading_assets.0.push(asset_server.load("mani.png"));
    command.insert_resource(loading_assets);
}

// Check until they are all loaded, then generate the atlas
fn generate_atlas(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AssetLoadingState>>,
    mut loading_images: ResMut<LoadingAssets>,
    asset_server: Res<AssetServer>,
    images: ResMut<Assets<Image>>,
) {
    // You should read events instead of checking asset state this way, but I'm kinda lazy
    let all_loaded = loading_images
        .0
        .iter()
        .all(|image| asset_server.load_state(image) == LoadState::Loaded);
    if !all_loaded {
        return;
    }
    // Images are all loaded, we can generate the atlas
    let mut builder = TextureAtlasBuilder::default();
    builder.padding(UVec2::splat(2));
    for image_handle in loading_images.0.iter() {
        builder.add_texture(
            Some(image_handle.id()),
            images.get(image_handle).expect("checked they are loaded"),
        );
    }
    let (layout, image) = builder.build().expect("should be correctly built");

    // Create the character database
    let mut characters_database = Characters {
        atlas_image: asset_server.add(image),
        character_map: HashMap::new(),
    };

    //Add the sprites to the database and remove them from loading_images so they are not permanently loaded
    for image_handle in loading_images.0.drain(..) {
        let atlas_indice = layout
            .get_texture_index(image_handle.id())
            .expect("should exist");
        // Each asset we loaded is a 7 sprite animation so we subdivide it to get our real layout for this sprite
        let rect = layout.textures.get(atlas_indice).expect("should exist");
        let layout = TextureAtlasLayout::from_grid(
            UVec2::splat(24),
            7,
            1,
            None,
            Some(UVec2::new(rect.min.x, rect.min.y)),
        );
        characters_database.character_map.insert(
            image_handle
                .path()
                .expect("path still exists")
                .to_string()
                .into_boxed_str(),
            CharacterRawData {
                atlas_layout: asset_server.add(layout),
                // I'm faking stats here, but you would generally do it all in a proper asset loader or hardcode it.
                hp: 100.0,
                mp: 150.0,
                speed: 1.0,
            },
        );
    }

    commands.insert_resource(characters_database);
    next_state.set(AssetLoadingState::Loaded);
}

pub struct AtlasAssetLoadingPlugin;

impl Plugin for AtlasAssetLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AssetLoadingState>()
            .add_systems(OnEnter(AssetLoadingState::Setup), load_assets)
            .add_systems(
                Update,
                generate_atlas.run_if(in_state(AssetLoadingState::Setup)),
            );
    }
}
