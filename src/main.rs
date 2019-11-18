#[allow(clippy::all)]
use roguelike::{
    map::{Map, Point},
    Entity, Game, GameState,
};
use tcod::colors;

fn main() {
    let mut player = Entity::new(Point::default(), '@', colors::WHITE, "player", true);
    player.set_alive(true);

    let mut entities = vec![player];
    let map = Map::new(&mut entities);

    let state = GameState { map, entities };

    Game::new(state).start();
}
