extern crate amethyst;

use amethyst::{
    assets::{Handle},
    prelude::*,
    renderer::{SpriteSheet},
};

use crate::{
    components::{ArenaConfig, Hero},
    entities::{initialize_camera, initialize_hero, load_hero_sprite_sheet}
};

#[derive(Default)]
pub struct Forest {
    sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

impl SimpleState for Forest {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {

        let world = data.world;
        //load the spritesheet
        let sprite_sheet_handle = load_hero_sprite_sheet(world);
        let arena = ArenaConfig::default();

        world.register::<Hero>();
        world.insert(arena);
        initialize_hero(world, sprite_sheet_handle);
        initialize_camera(world);
    }
}
