#[allow(clippy::all)]
use tcod::{colors, console::*};

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const LIMIT_FPS: i32 = 20;

struct Tcod {
    con: Offscreen,
    root: Root,
}

/// This is a generic entity: the player, a monster, an item, the stairs...
/// It's always represented by a character on screen
struct Entity {
    x: i32,
    y: i32,
    char: char,
    color: colors::Color,
}

impl Entity {
    fn new((x, y): (i32, i32), char: char, color: colors::Color) -> Self {
        Entity { x, y, char, color }
    }

    /// Move the entity by the given amount
    fn move_by(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    /// set the color and draw the character that represents this entity at its
    /// given position
    fn draw<T: Console>(&self, con: &mut T) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }
}

fn main() {
    let root = Root::initializer()
        .font("assets/fonts/arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Roguelike/Libtcod Tutorial")
        .init();
    let con = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    let mut tcod = Tcod { con, root };

    tcod::system::set_fps(LIMIT_FPS);

    let player = Entity::new((SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2), '@', colors::WHITE);
    let npc = Entity::new(
        (SCREEN_WIDTH / 2 - 5, SCREEN_HEIGHT / 2),
        '@',
        colors::YELLOW,
    );

    let mut entities = [player, npc];

    while !tcod.root.window_closed() {
        tcod.con.clear();

        for entity in &entities {
            entity.draw(&mut tcod.con);
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
        tcod.root.flush();

        let player = &mut entities[0];
        let exit = handle_keys(&mut tcod, player);
        if exit {
            break;
        }
    }
}

fn handle_keys(tcod: &mut Tcod, player: &mut Entity) -> bool {
    use tcod::input::{Key, KeyCode::*};

    let key = tcod.root.wait_for_keypress(true);
    match key {
        // Handle Movement
        Key { code: Up, .. } => player.move_by(0, -1),
        Key { code: Down, .. } => player.move_by(0, 1),
        Key { code: Left, .. } => player.move_by(-1, 0),
        Key { code: Right, .. } => player.move_by(1, 0),

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
