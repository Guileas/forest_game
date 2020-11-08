use amethyst::{
    core::timing::Time,
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
    renderer::{SpriteRender},
};

use crate::{
    components::{Animation, ActionStatus, Action},
};

pub struct AnimationSystem;

impl<'s> System<'s> for AnimationSystem{
    type SystemData = (
        ReadStorage<'s, Animation>,
        ReadStorage<'s, ActionStatus>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, Time>,
    );
    fn run(&mut self, (animations, action_statuses, mut sprite_renders, time): Self::SystemData) {
        // Get every animation with its associated SpriteRender component.
        for (animation, action_status, sprite) in (&animations, &action_statuses, &mut sprite_renders).join() {
          let elapsed_time = time.frame_number();
          let frame = (elapsed_time / animation.frame_duration) as i32 % animation.frames;
    
          match action_status.action_type {
              Action::Idle => {
                sprite.sprite_number = animation.first_sprite_index + frame as usize;
              }
          }
        }
      }
}