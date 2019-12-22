use super::{
    constants::*,
    entity::{actions::PlayerAction, Entity},
    map::Point,
    systems::mut_two,
    Game,
};
use tcod::input::{Key, KeyCode::*};

pub fn handle_keys(game: &mut Game, entities: &mut Vec<Entity>) -> PlayerAction {
    let key = game.tcod.root.wait_for_keypress(true);
    match (key, key.text(), entities[PLAYER].is_alive) {
        // Handle Movement
        (Key { code: Up, .. }, _, true) => {
            player_move_or_attack(Point::up(), game, entities);
            PlayerAction::TookTurn
        }
        (Key { code: Down, .. }, _, true) => {
            player_move_or_attack(Point::down(), game, entities);
            PlayerAction::TookTurn
        }
        (Key { code: Left, .. }, _, true) => {
            player_move_or_attack(Point::left(), game, entities);
            PlayerAction::TookTurn
        }
        (Key { code: Right, .. }, _, true) => {
            player_move_or_attack(Point::right(), game, entities);
            PlayerAction::TookTurn
        }

        // Handle Fullscreen
        (
            Key {
                code: Enter,
                alt: true,
                ..
            },
            _,
            _,
        ) => {
            let fullscreen = game.tcod.root.is_fullscreen();
            game.tcod.root.set_fullscreen(!fullscreen);
            PlayerAction::DidntTakeTurn
        }

        // Handle Escape
        (Key { code: Escape, .. }, _, _) => return PlayerAction::Exit,
        _ => PlayerAction::DidntTakeTurn,
    }
}

fn player_move_or_attack(direction: Point, game: &mut Game, entities: &mut Vec<Entity>) {
    let new_transform = direction + entities[PLAYER].position;

    let target_id = entities
        .iter()
        .position(|entity| entity.fighter.is_some() && entity.position == new_transform);

    match target_id {
        Some(target_id) => {
            let (player, target) = mut_two(PLAYER, target_id, entities);
            player.attack(target, game);
        }
        None => Entity::translate(PLAYER, direction, &mut game.state, entities),
    }
}
