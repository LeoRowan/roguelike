use super::{entity::Entity, map::Point, Game};
use tcod::input::{Key, KeyCode::*};

pub fn handle_keys(game: &mut Game, player: &mut Entity) -> bool {
    let key = game.tcod.root.wait_for_keypress(true);
    match key {
        // Handle Movement
        Key { code: Up, .. } => player.move_by(Point::up(), &game.state),
        Key { code: Down, .. } => player.move_by(Point::down(), &game.state),
        Key { code: Left, .. } => player.move_by(Point::left(), &game.state),
        Key { code: Right, .. } => player.move_by(Point::right(), &game.state),

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
