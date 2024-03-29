use super::{constants::*, entity::Entity, map::Point, Game};
use tcod::{
    colors::{self, Color},
    console::*,
    map::Map as FovMap,
};

pub struct Tcod {
    pub con: Offscreen,
    pub hud: Offscreen,
    pub root: Root,
    pub fov: FovMap,
}

pub fn render_all(game: &mut Game, fov_recompute: bool, entities: &Vec<Entity>) {
    recompute_fov(game, fov_recompute, entities);
    draw_map(game);
    draw_entities(game, entities);
    draw_hud(game, entities);

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

fn recompute_fov(game: &mut Game, fov_recompute: bool, entities: &Vec<Entity>) {
    if fov_recompute {
        let Point { x, y } = entities[PLAYER].position;
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

fn draw_entities(game: &mut Game, entities: &Vec<Entity>) {
    let mut to_draw: Vec<_> = entities
        .iter()
        .filter(|e| game.tcod.fov.is_in_fov(e.position.x, e.position.y))
        .collect();
    to_draw.sort_by(|a, b| a.blocks.cmp(&b.blocks));

    for entity in &to_draw {
        entity.draw(&mut game.tcod.con);
    }
}

fn draw_hud(game: &mut Game, entities: &Vec<Entity>) {
    game.tcod.hud.set_default_background(colors::BLACK);
    game.tcod.hud.clear();

    draw_hp(game, entities);
    draw_messages(game);

    blit(
        &mut game.tcod.hud,
        (0, 0),
        (SCREEN_WIDTH as i32, HUD_HEIGHT as i32),
        &mut game.tcod.root,
        (0, HUD_Y as i32),
        1.0,
        1.0,
    );

    fn draw_hp(game: &mut Game, entities: &Vec<Entity>) {
        let hp = entities[PLAYER].fighter.map_or(0, |f| f.hp);
        let max_hp = entities[PLAYER].fighter.map_or(0, |f| f.max_hp);

        draw_bar(
            &mut game.tcod.hud,
            Point::new(1, 1),
            BAR_WIDTH as i32,
            "HP",
            hp,
            max_hp,
            colors::DARK_RED,
            colors::DARK_GREY,
        );
    }

    fn draw_messages(game: &mut Game) {
        let mut y = MSG_HEIGHT as i32;
        for &(ref msg, color) in game.state.messages.iter().rev() {
            let msg_height =
                game.tcod
                    .hud
                    .get_height_rect(MSG_X as i32, y, MSG_WIDTH as i32, 0, msg);
            y -= msg_height;

            if y < 0 {
                break;
            }

            game.tcod.hud.set_default_foreground(color);
            game.tcod
                .hud
                .print_rect(MSG_X as i32, y, MSG_WIDTH as i32, 0, msg);
        }
    }
}

fn draw_bar(
    hud: &mut Offscreen,
    Point { x, y }: Point,
    total_width: i32,
    name: &str,
    value: i32,
    maximum: i32,
    bar_color: Color,
    back_color: Color,
) {
    let bar_width = (value as f64 / maximum as f64 * total_width as f64) as i32;

    hud.set_default_background(back_color);
    hud.rect(x, y, total_width, 1, false, BackgroundFlag::Set);

    hud.set_default_background(bar_color);
    if bar_width > 0 {
        hud.rect(x, y, bar_width, 1, false, BackgroundFlag::Set);
    }

    hud.set_default_foreground(colors::WHITE);
    hud.print_ex(
        x + total_width / 2,
        y,
        BackgroundFlag::None,
        TextAlignment::Center,
        &format!("{}: {}/{}", name, value, maximum),
    );
}
