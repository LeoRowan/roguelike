use super::{
    entity::Entity,
    map::Point,
    map::{COLOR_DARK_GROUND, COLOR_DARK_WALL, MAP_HEIGHT, MAP_WIDTH},
    Game,
};
use tcod::console::*;

pub const SCREEN_WIDTH: usize = 80;
pub const SCREEN_HEIGHT: usize = 50;

pub const LIMIT_FPS: usize = 20;

pub struct Tcod {
    pub con: Offscreen,
    pub root: Root,
}

pub fn render_all(game: &mut Game, entities: &[Entity]) {
    for entity in entities {
        entity.draw(&mut game.tcod.con);
    }

    for y in 0..MAP_HEIGHT as i32 {
        for x in 0..MAP_WIDTH as i32 {
            let color = if game.state.map.is_blocked_sight_tile(Point { x, y }) {
                COLOR_DARK_WALL
            } else {
                COLOR_DARK_GROUND
            };

            game.tcod
                .con
                .set_char_background(x, y, color, BackgroundFlag::Set);
        }
    }

    blit(
        &game.tcod.con,
        (0, 0),
        (SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32),
        &mut game.tcod.root,
        (0, 0),
        1.0,
        1.0,
    );
}
