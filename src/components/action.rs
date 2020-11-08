use amethyst::{
    ecs::{Component, DenseVecStorage},
};

use crate::{
    components::{HeroState},
};

pub struct ActionStatus {
    pub action_type: HeroState,
}

impl ActionStatus {
    pub fn set_action_type(&mut self, action: HeroState) {
        self.action_type = action;
    }
}

impl Component for ActionStatus{
    type Storage = DenseVecStorage<Self>;
}