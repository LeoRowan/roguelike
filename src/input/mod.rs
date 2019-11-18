use super::{constants::*, entity::Entity, map::Point, Game};
use tcod::input::{Key, KeyCode::*};

pub fn handle_keys(game: &mut Game) -> bool {
    let key = game.tcod.root.wait_for_keypress(true);
    match key {
        // Handle Movement
        Key { code: Up, .. } => Entity::translate(PLAYER, Point::up(), &mut game.state),
        Key { code: Down, .. } => Entity::translate(PLAYER, Point::down(), &mut game.state),
        Key { code: Left, .. } => Entity::translate(PLAYER, Point::left(), &mut game.state),
        Key { code: Right, .. } => Entity::translate(PLAYER, Point::right(), &mut game.state),

        // Handle Fulscreen
        Key {
            code: Enter,
            alt: true,
            ..
        } => {
            let fullscreen = game.tcod.root.is_fullscreen();
            game.tcod.root.set_fullscreen(!fullscreen);
        }
        Key { code: Escape, .. } => return true,
        _ => (),
    }
    false
}
