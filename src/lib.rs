use tcod::{self, console::*, map::Map as FovMap};
pub mod entity;
pub mod graphics;
pub mod input;
pub mod map;
pub mod state;

pub use entity::Entity;
use graphics::{Tcod, LIMIT_FPS, SCREEN_HEIGHT, SCREEN_WIDTH};
use map::{Point, MAP_HEIGHT, MAP_WIDTH};
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

        let mut tcod = Tcod {
            root,
            con: Offscreen::new(MAP_WIDTH as i32, MAP_HEIGHT as i32),
            fov: FovMap::new(MAP_WIDTH as i32, MAP_HEIGHT as i32),
        };

        tcod::system::set_fps(LIMIT_FPS as i32);
        for x in 0..MAP_WIDTH as i32 {
            for y in 0..MAP_HEIGHT as i32 {
                tcod.fov.set(
                    x,
                    y,
                    !state.map.is_blocked_sight_tile(Point { x, y }),
                    !state.map.is_blocked_tile(Point { x, y }),
                );
            }
        }

        Game { tcod, state }
    }

    pub fn start(mut self, mut entities: Vec<Entity>) {
        let mut previous_player_transform = Point::new(-1, -1);
        while !self.tcod.root.window_closed() {
            self.tcod.con.clear();

            let fov_recompute = previous_player_transform != entities[0].get_transform();
            graphics::render_all(&mut self, &entities, fov_recompute);
            self.tcod.root.flush();

            previous_player_transform = entities[0].get_transform();
            let exit = input::handle_keys(&mut self, &mut entities[0]);
            if exit {
                break;
            }
        }
    }
}
