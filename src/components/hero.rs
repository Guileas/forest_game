use amethyst::{
    ecs::{Component, DenseVecStorage},
};


/*
!!! : This enum is totally different from those in your 
"Prefab" this is only the animation for the player
*/
#[derive(PartialEq, Eq)]
pub enum HeroState {
    Idle,
}

impl Default for HeroState{
    fn default() -> Self{
        HeroState::Idle
    }
}

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Hero {
    pub state: HeroState,
    pub width: f32,
    pub height: f32,
}

impl Default for Hero {
    fn default() -> Self {
        Hero {
            state: HeroState::Idle,
            width: 64.0,
            height: 64.0,
        }
    }
}
