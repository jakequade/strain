mod asset_loader;
mod components;
mod entities;
mod resources;
mod states;
mod systems;

use amethyst::{
    animation::AnimationBundle,
    assets::PrefabLoaderSystemDesc,
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle, SpriteRender,
    },
    utils::application_root_dir,
};

use crate::{
    components::animation::{AnimationId, AnimationPrefabData},
    states::LoadState,
    systems::{
        animation::{AnimationControlSystem, DudeAnimationSystem},
        input::DudeInputSystem,
        motion::MotionSystem,
        physics::PhysicsSystem,
        transformation::{CameraTransformationSystem, TransformationSystem},
    },
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");
    let input_config_path = config_dir.join("bindings.ron");

    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(input_config_path)?;

    let game_data = GameDataBuilder::default()
        .with_system_desc(
            PrefabLoaderSystemDesc::<AnimationPrefabData>::default(),
            "scene_loader",
            &[],
        )
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(AnimationBundle::<AnimationId, SpriteRender>::new(
            "sprite_animation_control",
            "sprite_sampler_interpolation",
        ))?
        .with_bundle(input_bundle)?
        .with_bundle(
            TransformBundle::new()
                .with_dep(&["sprite_animation_control", "sprite_sampler_interpolation"]),
        )?
        .with(TransformationSystem, "transformation_system", &[])
        .with(
            CameraTransformationSystem,
            "camera_transformation_system",
            &["transformation_system"],
        )
        .with(
            DudeAnimationSystem,
            "dude_animation_system",
            &["transformation_system"],
        )
        .with(DudeInputSystem, "dude_input_system", &[])
        .with(MotionSystem, "dude_motion_system", &["dude_input_system"])
        .with(
            AnimationControlSystem,
            "animation_control_system",
            &["dude_animation_system"],
        );

    let mut game = Application::new(assets_dir, LoadState::default(), game_data)?;
    game.run();

    Ok(())
}
