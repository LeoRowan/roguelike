use super::{constants::*, map::Point, Game};
use tcod::{colors, console::*, map::Map as FovMap};

pub const SCREEN_WIDTH: usize = 80;
pub const SCREEN_HEIGHT: usize = 50;

pub const LIMIT_FPS: usize = 20;

pub struct Tcod {
    pub con: Offscreen,
    pub root: Root,
    pub fov: FovMap,
}

pub fn render_all(game: &mut Game, fov_recompute: bool) {
    recompute_fov(game, fov_recompute);
    draw_map(game);
    draw_entities(game);
    draw_hud(game);

    blit(
        &game.tcod.con,
        (0, 0),
        (SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32),
        &mut game.tcod.root,
        (0, 0),
        1.0,
        1.0,
    );

    fn recompute_fov(game: &mut Game, fov_recompute: bool) {
        if fov_recompute {
            let Point { x, y } = game.state.entities[PLAYER].position;
            game.tcod
                .fov
                .compute_fov(x, y, TORCH_RADIUS as i32, FOV_LIGTH_WALLS, FOV_ALGO);
        }
    }

    fn draw_map(game: &mut Game) {
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

                if visible {
                    game.state.map.set_explored_tile(Point { x, y });
                }

                if game.state.map.is_explored_tile(Point { x, y }) {
                    game.tcod
                        .con
                        .set_char_background(x, y, color, BackgroundFlag::Set);
                }
            }
        }
    }

    fn draw_entities(game: &mut Game) {
        let mut to_draw: Vec<_> = game
            .state
            .entities
            .iter()
            .filter(|e| game.tcod.fov.is_in_fov(e.position.x, e.position.y))
            .collect();
        to_draw.sort_by(|a, b| a.blocks.cmp(&b.blocks));

        for entity in &to_draw {
            entity.draw(&mut game.tcod.con);
        }
    }

    fn draw_hud(game: &mut Game) {
        game.tcod.root.set_default_foreground(colors::WHITE);
        if let Some(fighter) = game.state.entities[PLAYER].fighter {
            game.tcod.root.print_ex(
                1,
                (SCREEN_HEIGHT - 2) as i32,
                BackgroundFlag::None,
                TextAlignment::Left,
                format!("HP: {}/{}", fighter.hp, fighter.max_hp),
            )
        }
    }
}
