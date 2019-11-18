use super::{entity::Entity, map::Map};

pub struct GameState {
    pub map: Map,
    pub entities: Vec<Entity>,
}
