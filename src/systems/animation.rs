use amethyst::{
  animation::{get_animation_set, AnimationCommand, AnimationControlSet, AnimationSet, EndControl},
  ecs::{Entities, Join, ReadStorage, System, WriteStorage},
  renderer::SpriteRender,
};

use crate::components::{
  animation::{Animation, AnimationId},
  dude::{Dude, DudeState},
};

//#region AnimationControlSystem

#[derive(Default)]
pub struct AnimationControlSystem;

impl<'s> System<'s> for AnimationControlSystem {
  type SystemData = (
    Entities<'s>,
    ReadStorage<'s, Animation>,
    ReadStorage<'s, AnimationSet<AnimationId, SpriteRender>>,
    WriteStorage<'s, AnimationControlSet<AnimationId, SpriteRender>>,
  );

  fn run(
    &mut self,
    (mut entities, animations, animation_sets, mut animation_control_sets): Self::SystemData,
  ) {
    for (entity, animation, mut set) in (&entities, &animations, &animation_sets).join() {
      let control_set = get_animation_set(&mut animation_control_sets, entity).unwrap();

      if animation.show {
        animation.types.iter().for_each(|&animation_id| {
          // Add each animation to the AnimationControlSet if it isn't already there.
          // This ensures persistence after `abort()` calls
          if !control_set.has_animation(animation_id) {
            let end = match animation_id {
              AnimationId::DudeIdle => EndControl::Loop(None),
            };

            control_set.add_animation(
              animation_id,
              &set.get(&animation_id).unwrap(),
              end,
              1.0,
              AnimationCommand::Init,
            );
          }
        });
      }

      // Start the current animation
      control_set.start(animation.current);
    }
  }
}

//#endregion

//#region DudeAnimationSystem

#[derive(Default)]
pub struct DudeAnimationSystem;

impl<'s> System<'s> for DudeAnimationSystem {
  type SystemData = (
    Entities<'s>,
    ReadStorage<'s, Dude>,
    WriteStorage<'s, Animation>,
    WriteStorage<'s, AnimationControlSet<AnimationId, SpriteRender>>,
  );

  fn run(&mut self, (entities, dudes, mut animations, mut control_sets): Self::SystemData) {
    for (entity, dude, mut animation, control_set) in
      (&entities, &dudes, &mut animations, &mut control_sets).join()
    {
      let new_animation_id = match dude.state {
        DudeState::Idle => AnimationId::DudeIdle,
        _ => AnimationId::DudeIdle,
      };

      // If new animation differs from current, abort current
      // and start new
      if new_animation_id != animation.current {
        control_set.abort(animation.current);
        control_set.start(new_animation_id);

        animation.current = new_animation_id
      }
    }
  }
}

//#endregion
