use super::{
    constants::*,
    entity::{actions::PlayerAction, Entity},
    map::Point,
    state::GameState,
    Game,
};
use tcod::input::{Key, KeyCode::*};

pub fn handle_keys(game: &mut Game) -> PlayerAction {
    let key = game.tcod.root.wait_for_keypress(true);
    match (key, key.text(), game.state.entities[PLAYER].is_alive()) {
        // Handle Movement
        (Key { code: Up, .. }, _, true) => {
            player_move_or_attack(Point::up(), &mut game.state);
            PlayerAction::TookTurn
        }
        (Key { code: Down, .. }, _, true) => {
            player_move_or_attack(Point::down(), &mut game.state);
            PlayerAction::TookTurn
        }
        (Key { code: Left, .. }, _, true) => {
            player_move_or_attack(Point::left(), &mut game.state);
            PlayerAction::TookTurn
        }
        (Key { code: Right, .. }, _, true) => {
            player_move_or_attack(Point::right(), &mut game.state);
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

fn player_move_or_attack(direction: Point, state: &mut GameState) {
    let new_transform = direction + state.entities[PLAYER].get_transform();

    let target_id = state
        .entities
        .iter()
        .position(|x| x.get_transform() == new_transform);

    match target_id {
        Some(target_id) => {
            println!(
                "The {} laughs at your attempt to attack him.",
                state.entities[target_id].name()
            );
        }
        None => Entity::translate(PLAYER, direction, state),
    }
}
