use amethyst::{
    animation::{AnimationControlSet, AnimationSet, EndControl},
    core::timing::Time,
    ecs::prelude::{Entities, Join, Read, ReadStorage, System, WriteStorage},
    renderer::SpriteRender,
};

use crate::components::{Animation, AnimationId, Hero, HeroState};

pub struct AnimationSystem;

impl<'s> System<'s> for AnimationSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Hero>,
        WriteStorage<'s, Animation>,
        WriteStorage<'s, AnimationControlSet<AnimationId, SpriteRender>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, heros, mut animations, mut animation_control_sets) = data;

        for (entity, hero, mut animation, animation_control_set) in (
            &entities, 
            &heros, 
            &mut animations, 
            &mut animation_control_sets
        )
        .join() 
        {

            let new_animation_id = match hero.state {
                HeroState::Idle => AnimationId::Idle,
                _ => AnimationId::Idle,
            };

            animation_control_set.start(new_animation_id);


           /* let elapsed_time = time.frame_number();
            let frame = (elapsed_time / animation.frame_duration) as i32 % animation.frames;

            // List of all movement with their result
            match new_animation_id {
                HeroState::Idle => {
                    sprite.sprite_number = animation.first_sprite_index + frame as usize;
                }
            }*/
        }
    }
}
