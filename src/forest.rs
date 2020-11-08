extern crate amethyst;

use amethyst::{
    assets::{ProgressCounter, RonFormat, PrefabLoader, Handle, Prefab},
    prelude::*,
    renderer::{SpriteSheet},
};

use crate::{
    components::{ArenaConfig, Hero, AnimationPrefabData},
    entities::{initialize_camera, initialize_hero, load_hero_sprite_sheet}
};


#[derive(Default)]
pub struct Forest {
    progress_counter: ProgressCounter,
    prefab_handle: Option<Handle<Prefab<AnimationPrefabData>>>,
}

impl SimpleState for Forest {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {

        let world = data.world;
        //load the spritesheet
        let mut progress_counter = ProgressCounter::new();
        
        let arena = ArenaConfig::default();
        world.insert(arena);
        initialize_camera(world);

        //let prefab_handle = Some(load_hero_sprite_sheet(world, progress_counter));

        let prefab_handle = world.exec(|loader: PrefabLoader<'_, AnimationPrefabData>| {

            loader.load(
                "AxeWarrior/Idle/hero_spritesheet.ron", 
                RonFormat, 
                (),
            )
            
        });
        world.register::<Hero>();
        world.create_entity().with(prefab_handle).build();
        
        
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        data.data.update(&data.world);
        /*if let Some(ref counter) = self.progress_counter.as_ref() {
            println!(
                "Loading: {}, Failed: {}, Finished: {}",
                counter.num_loading(),
                counter.num_failed(),
                counter.num_finished()
            );*/
            if self.progress_counter.is_complete() {
                println!(
                    "Complete"
                );
                /*let hero_prefab_handle = data.world.read_resource::<Handle<Prefab<AnimationPrefabData>>>();
                
                
                hero_prefab_handle.clone();*/
                //initialize_hero(data.world);
                
            }
        //}

        Trans::None
    }
}
