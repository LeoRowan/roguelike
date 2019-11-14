use tcod::{self, console::*};
pub mod entity;
pub mod graphics;
pub mod input;
pub mod map;
pub mod state;

pub use entity::Entity;
use graphics::{Tcod, LIMIT_FPS, SCREEN_HEIGHT, SCREEN_WIDTH};
use map::{MAP_HEIGHT, MAP_WIDTH};
pub use state::GameState;

pub struct Game {
    pub tcod: graphics::Tcod,
    pub state: state::GameState,
}

impl Game {
    pub fn new(state: GameState) -> Self {
        let root = Root::initializer()
            .font("assets/fonts/arial10x10.png", FontLayout::Tcod)
            .font_type(FontType::Greyscale)
            .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
            .title("Roguelike/Libtcod Tutorial")
            .init();
        let con = Offscreen::new(MAP_WIDTH as i32, MAP_HEIGHT as i32);

        let tcod = Tcod { con, root };
        tcod::system::set_fps(LIMIT_FPS as i32);

        Game { tcod, state }
    }

    pub fn start(mut self, mut entities: Vec<Entity>) {
        while !self.tcod.root.window_closed() {
            self.tcod.con.clear();

            graphics::render_all(&mut self, &entities);
            self.tcod.root.flush();

            let exit = input::handle_keys(&mut self, &mut entities[0]);
            if exit {
                break;
            }
        }
    }
}
