#[allow(clippy::all)]
use roguelike::{
    map::{Map, Point},
    Entity, Game, GameState,
};
use tcod::colors;

fn main() {
    let player = Entity::new(Point::default(), '@', colors::WHITE);
    let mut entities = vec![player];
    let map = Map::new(&mut entities);

    let state = GameState { map };

    Game::new(state).start(entities);
}
