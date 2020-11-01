extern crate amethyst;

use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

// ACTION

#[derive(PartialEq, Eq)]
pub enum Action{
    Idle
}

pub struct ActionStatus{
    pub action_type: Action,
}

impl ActionStatus{
    pub fn set_action_type(&mut self, action: Action){
        self.action_type = action;
    }
}

impl Component for ActionStatus {
    type Storage = DenseVecStorage<Self>;
}

// Animations

pub struct Animation {
    pub frames: i32,
    pub frame_duration: u64,
    pub first_sprite_index: usize,
}

impl Component for Animation{
    type Storage = DenseVecStorage<Self>;
}

// HERO SECTION

pub const HERO_WIDTH: f32 = 64.0;
pub const HERO_HEIGHT: f32 = 64.0;


pub struct Hero {
    pub width: f32,
    pub height: f32
}

impl Hero{
    fn new() -> Hero{
        Hero{
            width: HERO_WIDTH,
            height: HERO_HEIGHT
        }
    }
}

impl Component for Hero{
    type Storage = DenseVecStorage<Self>;
}

// INIT HERO 

fn initialise_hero(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>){
    
    let mut hero_transform = Transform::default();
    let y = ARENA_HEIGHT / 2.0;
    hero_transform.set_translation_xyz(ARENA_WIDTH, y, 0.0);

    let animation = Animation{
        frames: 9,
        frame_duration: 10,
        first_sprite_index: 0,
    };

    let action_status = ActionStatus{
        action_type: Action::Idle
    };
    let sprite_render  = SpriteRender{
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
fn load_hero_sprite_sheet(world: &mut World) -> Handle<SpriteSheet>{
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

// CAMERA

fn initialise_camera(world: &mut World){
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.9 , ARENA_HEIGHT * 0.7, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

// WORLD SECTION

pub const ARENA_HEIGHT: f32 = 500.0;
pub const ARENA_WIDTH: f32 = 500.0;

#[derive(Default)]
pub struct Forest{
    sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

impl SimpleState for Forest {

    fn on_start(&mut self, data:StateData<'_, GameData<'_,'_>>){
        let world = data.world;
        //load the spritesheet
        let sprite_sheet_handle = load_hero_sprite_sheet(world);

        world.register::<Hero>();

        initialise_hero(world, sprite_sheet_handle);
        initialise_camera(world);
    }
}

