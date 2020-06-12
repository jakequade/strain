mod asset_loader;
mod components;
mod entities;
mod resources;
mod states;
mod strain;
mod systems;

use amethyst::{
    animation::AnimationBundle,
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        sprite::SpriteRender,
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

use crate::{
    components::animation::{AnimationId, AnimationPrefabData},
    strain::Strain,
    systems::{
        AnimationControlSystem, CameraTransformationSystem, DudeAnimationSystem, PhysicsSystem, TransformationSystem,
        WalkingSystem,
    },
    states::LoadState
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
        .with_bundle(TransformBundle::new())?
        // .with(WalkingSystem, "walking_system", &[])
        // .with(PhysicsSystem, "physics_system", &["walking_system"])
        .with(TransformationSystem, "transformation_system", &[])
        .with(CameraTransformationSystem, "camera_transformation_system", &["transformation_system"])
        .with(
            DudeAnimationSystem,
            "dude_animation_system",
            &["transformation_system"],
        )
        .with(
            AnimationControlSystem,
            "animation_control_system",
            &["dude_animation_system"],
        );

    let mut game = Application::new(assets_dir, LoadState::default(), game_data)?;
    game.run();

    Ok(())
}
