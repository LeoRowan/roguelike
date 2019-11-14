use tcod::{self, console::*};
pub mod entity;
pub mod graphics;
pub mod input;
pub mod map;
pub mod state;

use entity::Entity;
use graphics::{Tcod, LIMIT_FPS, SCREEN_HEIGHT, SCREEN_WIDTH};
use map::{Map, Point, MAP_HEIGHT, MAP_WIDTH};
use state::GameState;
use tcod::colors;

pub struct Game {
    pub tcod: graphics::Tcod,
    pub state: state::GameState,
}

impl Game {
    pub fn new() -> Self {
        let root = Root::initializer()
            .font("assets/fonts/arial10x10.png", FontLayout::Tcod)
            .font_type(FontType::Greyscale)
            .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
            .title("Roguelike/Libtcod Tutorial")
            .init();
        let con = Offscreen::new(MAP_WIDTH as i32, MAP_HEIGHT as i32);

        let tcod = Tcod { con, root };
        tcod::system::set_fps(LIMIT_FPS as i32);

        let state = GameState { map: Map::new() };

        Game { tcod, state }
    }

    pub fn start(mut self) {
        let half_width = SCREEN_WIDTH as i32 / 2;
        let half_height = SCREEN_HEIGHT as i32 / 2;

        let player = Entity::new(Point::new(25, 23), '@', colors::WHITE);
        let npc = Entity::new(Point::new(55, 23), '@', colors::YELLOW);
        let mut entities = [player, npc];

        while !self.tcod.root.window_closed() {
            self.tcod.con.clear();

            graphics::render_all(&mut self, &entities);
            self.tcod.root.flush();

            let player = &mut entities[0];
            let exit = input::handle_keys(&mut self, player);
            if exit {
                break;
            }
        }
    }
}
