use amethyst::{
    assets::{Handle, Loader, PrefabLoaderSystemDesc, ProgressCounter},
    prelude::{GameData, SimpleState, SimpleTrans, StateData, Trans, WorldExt},
};

use crate::resources::{load_assets, PrefabList};

#[derive(Default)]
pub struct LoadState {
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
