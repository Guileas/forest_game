use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    prelude::*,
    renderer::{ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

use crate::{
    components::{ArenaConfig, Animation, ActionStatus, Action},
};

pub fn initialize_hero(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {

    

    let mut hero_transform = Transform::default();
    let y = ArenaConfig::default().height / 2.0;
    hero_transform.set_translation_xyz(ArenaConfig::default().width, y, 0.0);

    let animation = Animation {
        frames: 9,
        frame_duration: 10,
        first_sprite_index: 0,
    };

    let action_status = ActionStatus {
        action_type: Action::Idle,
    };
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: animation.first_sprite_index,
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(animation)
        .with(action_status)
        .with(hero_transform)
        .build();
}

// HERO SPRITE
pub fn load_hero_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    // Load hero spritesheet
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "AxeWarrior/Idle/idle.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };
    //Load hero ron spritesheet definition
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "AxeWarrior/Idle/hero_spritesheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}