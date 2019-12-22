use tcod::{self, colors, console::*, map::Map as FovMap};
pub mod components;
pub mod constants;
pub mod entity;
pub mod graphics;
pub mod input;
pub mod map;
pub mod messages;
pub mod state;
pub mod systems;

use constants::*;
pub use entity::{actions::PlayerAction, Entity};
use graphics::Tcod;
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

        let tcod = Tcod {
            root,
            con: Offscreen::new(MAP_WIDTH as i32, MAP_HEIGHT as i32),
            hud: Offscreen::new(MAP_WIDTH as i32, HUD_HEIGHT as i32),
            fov: FovMap::new(MAP_WIDTH as i32, MAP_HEIGHT as i32),
        };

        tcod::system::set_fps(LIMIT_FPS as i32);

        Game { tcod, state }
    }

    pub fn start(mut self, mut entities: Vec<Entity>) {
        self.state.messages.add(
            "Welcome adventurer! Prepare to perish in the Tombs of the Lost Kings.",
            colors::RED,
        );

        for x in 0..MAP_WIDTH as i32 {
            for y in 0..MAP_HEIGHT as i32 {
                self.tcod.fov.set(
                    x,
                    y,
                    !self.state.map.is_blocked_sight_tile(Point { x, y }),
                    !self.state.map.is_blocked_tile(Point { x, y }, &entities),
                );
            }
        }

        let mut previous_player_transform = Point::new(-1, -1);
        while !self.tcod.root.window_closed() {
            self.tcod.con.clear();

            let fov_recompute = previous_player_transform != entities[PLAYER].position;
            graphics::render_all(&mut self, fov_recompute, &entities);
            self.tcod.root.flush();

            previous_player_transform = entities[PLAYER].position;
            let player_action = input::handle_keys(&mut self, &mut entities);
            if player_action == PlayerAction::Exit {
                break;
            }

            if entities[PLAYER].is_alive && player_action == PlayerAction::TookTurn {
                for id in 1..entities.len() {
                    if entities[id].ai.is_some() {
                        systems::ai_take_turn(id, &mut self, &mut entities);
                    }
                }
            }
        }
    }
}
