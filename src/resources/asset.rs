use std::collections::HashMap;

use amethyst::{
    assets::{AssetStorage, Handle, Loader, Prefab, PrefabLoader, ProgressCounter, RonFormat},
    ecs::prelude::World,
    renderer:: {
        sprite::{SpriteSheetFormat, SpriteSheetHandle},
        SpriteSheet, Texture,
    },
};

use crate::components::animation::AnimationPrefabData;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum AssetType {
    Dude,
}

//#region PrefabList

#[derive(Default)]
pub struct PrefabList {
    prefabs: HashMap<AssetType, Handle<Prefab<AnimationPrefabData>>>
}

impl PrefabList {
    pub fn insert(&mut self, asset_type: AssetType, prefab_handle: Handle<Prefab<AnimationPrefabData>>) {
        self.prefabs.insert(asset_type, prefab_handle);
    }

    pub fn get(&self, asset_type: AssetType) -> Option<&Handle<Prefab<AnimationPrefabData>>> {
        self.prefabs.get(&asset_type)
    }
}

//#endregion

//#region SpriteSheetList

//#endregion

pub fn load_assets(world: &mut World, asset_type_list: Vec<AssetType>) -> ProgressCounter {
    let mut prefab_list = PrefabList::default();
    let mut progress_counter = ProgressCounter::new();

    for &asset_type in asset_type_list.iter() {
        let (_, ron_path) = match asset_type {
            AssetType::Dude => {
                ("", "prefabs/dude.ron")
            }
        };

        match asset_type {
            AssetType::Dude => {
                let prefab_handle = get_animation_prefab_handle(world, ron_path, &mut progress_counter);
                prefab_list.insert(asset_type, prefab_handle);
            }
        };
    }

    world.insert(prefab_list);
    progress_counter
}

fn get_animation_prefab_handle(
    world: &mut World,
    ron_path: &str,
    progress_counter: &mut ProgressCounter
) -> Handle<Prefab<AnimationPrefabData>> {
    world.exec(|loader: PrefabLoader<'_, AnimationPrefabData>| {
        loader.load(ron_path, RonFormat, progress_counter)
    })
}