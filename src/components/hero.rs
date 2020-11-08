use amethyst::{
    ecs::{Component, DenseVecStorage},
};

pub const HERO_WIDTH: f32 = 64.0;
pub const HERO_HEIGHT: f32 = 64.0;

pub struct Hero {
    pub width: f32,
    pub height: f32,
}

impl Hero {
    fn new() -> Hero {
        Hero {
            width: HERO_WIDTH,
            height: HERO_HEIGHT,
        }
    }
}

impl Component for Hero {
    type Storage = DenseVecStorage<Self>;
}