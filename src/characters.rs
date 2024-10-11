// Characters data

use bevy::asset::Handle;
use bevy::prelude::{
    default, Bundle, Component, Image, Resource, SpriteBundle, TextureAtlas, TextureAtlasLayout,
    Transform, Vec3,
};
use bevy::utils::HashMap;

// We need an id, in this case we use the texture asset path
pub type CharacterId = Box<str>;

// Example component for a character
#[derive(Component)]
pub struct BaseStats {
    pub speed: f32,
    pub hp: f32,
    pub mp: f32,
}

impl From<CharacterRawData> for BaseStats {
    fn from(data: CharacterRawData) -> Self {
        Self {
            speed: data.speed,
            hp: data.hp,
            mp: data.mp,
        }
    }
}

#[derive(Bundle)]
pub struct CharacterBundle {
    pub sprite: SpriteBundle,
    pub atlas: TextureAtlas,
    pub stats: BaseStats,
}

#[derive(Clone)]
pub struct CharacterRawData {
    // Base character stats for mocking purposes
    pub speed: f32,
    pub hp: f32,
    pub mp: f32,
    // You can add other stuff here: items, spells, ai, etc..

    // I'm going to use layouts but this could be atlas indices
    pub atlas_layout: Handle<TextureAtlasLayout>,
}

#[derive(Resource)]
pub struct Characters {
    pub atlas_image: Handle<Image>,

    pub character_map: HashMap<CharacterId, CharacterRawData>,
}

impl Characters {
    pub fn character(&self, id: CharacterId) -> Option<CharacterBundle> {
        let data = self.character_map.get(&id)?;

        Some(CharacterBundle {
            sprite: SpriteBundle {
                texture: self.atlas_image.clone(),
                transform: Transform::from_scale(Vec3::new(6.0, 6.0, 1.0)),
                ..default()
            },
            atlas: TextureAtlas::from(data.atlas_layout.clone()),
            stats: BaseStats::from(data.clone()),
        })
    }
}
