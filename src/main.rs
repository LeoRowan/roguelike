#[allow(clippy::all)]
use roguelike::{
    components::Fighter,
    map::{Map, Point},
    Entity, Game, GameState,
};
use tcod::colors;

fn main() {
    let mut player = Entity::new(Point::default(), '@', colors::WHITE, "player", true)
        .with_fighter_component(Fighter::new(30, 30, 2, 5));
    player.is_alive = true;

    let mut entities = vec![player];
    let map = Map::new(&mut entities);

    let state = GameState { map, entities };

    Game::new(state).start();
}
