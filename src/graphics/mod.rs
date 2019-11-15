use super::{
    entity::Entity,
    map::Point,
    map::{
        COLOR_DARK_GROUND, COLOR_DARK_WALL, COLOR_LIGHT_GROUND, COLOR_LIGHT_WALL, FOV_ALGO,
        FOV_LIGTH_WALLS, MAP_HEIGHT, MAP_WIDTH, TORCH_RADIUS,
    },
    Game,
};
use tcod::{console::*, map::Map as FovMap};

pub const SCREEN_WIDTH: usize = 80;
pub const SCREEN_HEIGHT: usize = 50;

pub const LIMIT_FPS: usize = 20;

pub struct Tcod {
    pub con: Offscreen,
    pub root: Root,
    pub fov: FovMap,
}

pub fn render_all(game: &mut Game, entities: &Vec<Entity>, fov_recompute: bool) {
    for entity in entities {
        let Point { x, y } = entity.get_transform();
        if game.tcod.fov.is_in_fov(x, y) {
            entity.draw(&mut game.tcod.con);
        };
    }

    if fov_recompute {
        let Point { x, y } = entities[0].get_transform();
        game.tcod
            .fov
            .compute_fov(x, y, TORCH_RADIUS as i32, FOV_LIGTH_WALLS, FOV_ALGO);
    }

    for y in 0..MAP_HEIGHT as i32 {
        for x in 0..MAP_WIDTH as i32 {
            let visible = game.tcod.fov.is_in_fov(x, y);
            let wall = game.state.map.is_blocked_sight_tile(Point { x, y });
            let color = match (visible, wall) {
                (false, true) => COLOR_DARK_WALL,
                (false, false) => COLOR_DARK_GROUND,
                (true, true) => COLOR_LIGHT_WALL,
                (true, false) => COLOR_LIGHT_GROUND,
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
