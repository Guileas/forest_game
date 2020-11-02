use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ArenaConfig {
    pub height: f32,
    pub width: f32,
}

impl Default for ArenaConfig {
    fn default() -> Self {
        ArenaConfig {
            height: 500.0,
            width: 500.0,
        }
    }
}