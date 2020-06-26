use amethyst::{
    assets::{Handle, JsonFormat, Loader, PrefabLoaderSystemDesc, ProgressCounter},
    prelude::{GameData, SimpleState, SimpleTrans, StateData, Trans, WorldExt},
};

use crate::resources::{load_assets, PrefabList};

#[derive(Default)]
pub struct LoadState {
    map_handle: Option<Handle<Map>>,
    progress_counter: Option<ProgressCounter>,
}

use crate::{
    entities::{load_camera, load_camera_subject, load_dude},
    resources::{AssetType, Context},
};

impl SimpleState for LoadState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        world.insert(Context::new());

        // Load map
        let loader = world.read_resource::<Loader>();

        self.map_handle = {
            Some(loader.load(
                "textures/level-one-demo.json",
                JsonFormat,
                self.progress_counter.as_mut().expect("couldn't load progress counter for map json"),
                &world.read_resource::<AssetStorage<Map>>()
            ))
        };

        self.progress_counter = Some(load_assets(world, vec![AssetType::Dude]));

        let camera_subject = load_camera_subject(world);
        load_camera(world, camera_subject);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if let Some(ref progress_counter) = self.progress_counter {
            if progress_counter.is_complete() {
                let dude_prefab_handle = {
                    let prefab_list = data.world.read_resource::<PrefabList>();
                    prefab_list.get(AssetType::Dude).unwrap().clone()
                };
                load_dude(data.world, dude_prefab_handle);

                self.progress_counter = None;
            }
        }
        Trans::None
    }
}
