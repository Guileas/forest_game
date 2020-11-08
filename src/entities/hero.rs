use amethyst::{
    assets::{AssetStorage, Handle, Loader, PrefabLoader, Prefab, ProgressCounter, RonFormat},
    core::transform::Transform,
    prelude::*,
    renderer::{ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

use crate::{
    components::{ArenaConfig, Animation, AnimationPrefabData, ActionStatus, HeroState},
};

/**
 * Set the position of the hero in the arena
 */
pub fn initialize_hero(world: &mut World) {

    println!(
        "Initialize hero"
    );
    let mut hero_transform = Transform::default();
    let y = ArenaConfig::default().height / 2.0;
    hero_transform.set_translation_xyz(ArenaConfig::default().width, y, 0.0);

    world
        .create_entity()
        .with(hero_transform)
        .build();
}

/**
 * Load hero ron spritesheet
 */
pub fn load_hero_sprite_sheet(world: &mut World, mut progress_counter: ProgressCounter) -> Handle<Prefab<AnimationPrefabData>>{

    println!(
        "Hey"
    );
    world.exec(|loader: PrefabLoader<'_, AnimationPrefabData>| {

        loader.load(
            "AxeWarrior/Idle/hero_spritesheet.ron", 
            RonFormat, 
            &mut progress_counter,
        )
        
    })
    
    //world.insert(prefab_handle);

    //world.create_entity().with(prefab_handle).build();
    
    
    // Load hero spritesheet
    /*let texture_handle = {
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
        ,
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )*/
}