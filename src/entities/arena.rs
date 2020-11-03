use amethyst::{
    core::transform::Transform,
    prelude::*,
    renderer::{Camera},
};

use crate::components::ArenaConfig;

pub fn initialize_camera(world: &mut World) {
    
    let mut transform = Transform::default();
    transform.set_translation_xyz(ArenaConfig::default().width * 0.9, ArenaConfig::default().height * 0.7, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ArenaConfig::default().width, ArenaConfig::default().height))
        .with(transform)
        .build();
}