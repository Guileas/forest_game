use amethyst::{
    ecs::{Component, DenseVecStorage},
};

#[derive(PartialEq, Eq)]
pub enum Action {
    Idle,
}

pub struct ActionStatus {
    pub action_type: Action,
}

impl ActionStatus {
    pub fn set_action_type(&mut self, action: Action) {
        self.action_type = action;
    }
}

impl Component for ActionStatus {
    type Storage = DenseVecStorage<Self>;
}