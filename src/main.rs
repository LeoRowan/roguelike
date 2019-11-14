#[allow(clippy::all)]
use tcod::{
    colors::{self, Color},
    console::*,
};

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const LIMIT_FPS: i32 = 20;

const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 45;

const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_DARK_GROUND: Color = Color {
    r: 50,
    g: 50,
    b: 150,
};

struct Tcod {
    con: Offscreen,
    root: Root,
}

/// This is a generic entity: the player, a monster, an item, the stairs...
/// It's always represented by a character on screen
#[derive(Debug)]
struct Entity {
    x: i32,
    y: i32,
    char: char,
    color: Color,
}

impl Entity {
    fn new((x, y): (i32, i32), char: char, color: Color) -> Self {
        Entity { x, y, char, color }
    }

    /// Move the entity by the given amount
    fn move_by(&mut self, (dx, dy): (i32, i32), game: &Game) {
        let new_x = self.x + dx;
        let new_y = self.y + dy;

        if !game.map.out_of_bounds(new_x, new_y) && !game.map.is_blocked_tile(new_x, new_y) {
            self.x = new_x;
            self.y = new_y;
        }
    }

    /// set the color and draw the character that represents this entity at its
    /// given position
    fn draw<T: Console>(&self, con: &mut T) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }
}

/// A tile for the map
#[derive(Clone, Copy, Debug)]
struct Tile {
    blocked: bool,
    block_sight: bool,
}

impl Tile {
    fn empty() -> Self {
        Tile {
            blocked: false,
            block_sight: false,
        }
    }

    fn wall() -> Self {
        Tile {
            blocked: true,
            block_sight: true,
        }
    }
}

struct Map(Vec<Tile>);

impl Map {
    fn new() -> Self {
        let mut map = Map(vec![Tile::empty(); (MAP_HEIGHT * MAP_WIDTH) as usize]);
        map.set_tile((30, 22), Tile::wall());
        map.set_tile((50, 22), Tile::wall());

        map
    }

    fn tile_idx(&self, x: i32, y: i32) -> usize {
        (y * MAP_WIDTH + x) as usize
    }

    fn tile_at<'a>(&'a self, x: i32, y: i32) -> &'a Tile {
        &self.0[self.tile_idx(x, y)]
    }

    fn set_tile(&mut self, (x, y): (i32, i32), tile: Tile) {
        let idx = self.tile_idx(x, y);
        self.0[idx] = tile;
    }

    fn is_blocked_tile(&self, x: i32, y: i32) -> bool {
        self.tile_at(x, y).blocked
    }

    fn is_blocked_sight_tile(&self, x: i32, y: i32) -> bool {
        self.tile_at(x, y).block_sight
    }

    fn out_of_bounds(&self, x: i32, y: i32) -> bool {
        !(self.tile_idx(x, y) < self.0.len())
    }
}

struct Game {
    map: Map,
}

fn main() {
    let root = Root::initializer()
        .font("assets/fonts/arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Roguelike/Libtcod Tutorial")
        .init();
    let con = Offscreen::new(MAP_WIDTH, MAP_HEIGHT);

    let mut tcod = Tcod { con, root };

    tcod::system::set_fps(LIMIT_FPS);

    let player = Entity::new((SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2), '@', colors::WHITE);
    let npc = Entity::new(
        (SCREEN_WIDTH / 2 - 5, SCREEN_HEIGHT / 2),
        '@',
        colors::YELLOW,
    );

    let game = Game { map: Map::new() };
    let mut entities = [player, npc];

    while !tcod.root.window_closed() {
        tcod.con.clear();

        render_all(&mut tcod, &game, &entities);
        tcod.root.flush();

        let player = &mut entities[0];
        let exit = handle_keys(&mut tcod, &game, player);
        if exit {
            break;
        }
    }
}

fn handle_keys(tcod: &mut Tcod, game: &Game, player: &mut Entity) -> bool {
    use tcod::input::{Key, KeyCode::*};

    let key = tcod.root.wait_for_keypress(true);
    match key {
        // Handle Movement
        Key { code: Up, .. } => player.move_by((0, -1), game),
        Key { code: Down, .. } => player.move_by((0, 1), game),
        Key { code: Left, .. } => player.move_by((-1, 0), game),
        Key { code: Right, .. } => player.move_by((1, 0), game),

        // Handle Fulscreen
        Key {
            code: Enter,
            alt: true,
            ..
        } => {
            let fullscreen = tcod.root.is_fullscreen();
            tcod.root.set_fullscreen(!fullscreen);
        }
        Key { code: Escape, .. } => return true,
        _ => (),
    }
    false
}

fn render_all(tcod: &mut Tcod, game: &Game, entities: &[Entity]) {
    for entity in entities {
        entity.draw(&mut tcod.con);
    }

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let color = if game.map.is_blocked_sight_tile(x, y) {
                COLOR_DARK_WALL
            } else {
                COLOR_DARK_GROUND
            };

            tcod.con
                .set_char_background(x, y, color, BackgroundFlag::Set);
        }
    }

    blit(
        &tcod.con,
        (0, 0),
        (SCREEN_WIDTH, SCREEN_HEIGHT),
        &mut tcod.root,
        (0, 0),
        1.0,
        1.0,
    );
}
