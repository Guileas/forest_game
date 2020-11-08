use amethyst::{
    animation::AnimationSetPrefab,
    assets::{PrefabData,ProgressCounter},
    derive::PrefabData,
    ecs::{Entity, Component, DenseVecStorage},
    error::Error,
    renderer::sprite::{prefab::SpriteScenePrefab,SpriteRender},
};

use serde::{Deserialize, Serialize};

/* AnimationId is the ID used in my prefab file in AnimationSet section to identify which animation to play 

!!! : This enum should contain all the Animation you've set in all your prefab files
*/
#[derive(Eq, PartialEq, Deserialize)]
pub enum AnimationId {
    Idle,
}

#[derive(Deserialize)]
pub struct AnimationPrefabData {
    // Information for rendering a scene with sprites
    sprite_scene: SpriteScenePrefab,
    // Use in Prefab file to represent all the animations that can be run on the 'Entity'
    animation_set: AnimationSetPrefab<AnimationId, SpriteRender>
}

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Animation {
    pub current: AnimationId,
    pub types: Vec<AnimationId>,
}