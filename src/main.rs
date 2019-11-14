#[allow(clippy::all)]
use roguelike::{
    map::{Map, Point},
    Entity, Game, GameState,
};
use tcod::colors;

fn main() {
    let mut player = Entity::new(Point::default(), '@', colors::WHITE);
    let map = Map::new(&mut player);
    let entities = vec![player];

    let state = GameState { map };

    Game::new(state).start(entities);
}
