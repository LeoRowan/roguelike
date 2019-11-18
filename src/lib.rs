use tcod::{self, console::*, map::Map as FovMap};
pub mod constants;
pub mod entity;
pub mod graphics;
pub mod input;
pub mod map;
pub mod state;

use constants::*;
pub use entity::{actions::PlayerAction, Entity};
use graphics::{Tcod, LIMIT_FPS, SCREEN_HEIGHT, SCREEN_WIDTH};
use map::Point;
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
                    !state.map.is_blocked_tile(Point { x, y }, &state.entities),
                );
            }
        }

        Game { tcod, state }
    }

    pub fn start(mut self) {
        let mut previous_player_transform = Point::new(-1, -1);
        while !self.tcod.root.window_closed() {
            self.tcod.con.clear();

            let fov_recompute =
                previous_player_transform != self.state.entities[PLAYER].get_transform();
            graphics::render_all(&mut self, fov_recompute);
            self.tcod.root.flush();

            previous_player_transform = self.state.entities[PLAYER].get_transform();
            let player_action = input::handle_keys(&mut self);
            if player_action == PlayerAction::Exit {
                break;
            }

            if self.state.entities[PLAYER].is_alive() && player_action == PlayerAction::TookTurn {
                for entity in self.state.entities.iter().skip(1) {
                    println!("The {} growls", entity.name())
                }
            }
        }
    }
}
