use tcod::{colors::Color, map::FovAlgorithm};

pub const SCREEN_WIDTH: usize = 80;
pub const SCREEN_HEIGHT: usize = 50;

pub const LIMIT_FPS: usize = 20;

pub const MAP_WIDTH: usize = 80;
pub const MAP_HEIGHT: usize = 43;

pub const HUD_HEIGHT: usize = 7;
pub const HUD_Y: usize = SCREEN_HEIGHT - HUD_HEIGHT;
pub const BAR_WIDTH: usize = 20;

pub const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
pub const COLOR_LIGHT_WALL: Color = Color {
    r: 130,
    g: 110,
    b: 50,
};
pub const COLOR_DARK_GROUND: Color = Color {
    r: 50,
    g: 50,
    b: 150,
};
pub const COLOR_LIGHT_GROUND: Color = Color {
    r: 200,
    g: 180,
    b: 50,
};

pub const FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic;
pub const FOV_LIGTH_WALLS: bool = true;
pub const TORCH_RADIUS: usize = 10;

pub const ROOM_MAX_SIZE: usize = 10;
pub const ROOM_MIN_SIZE: usize = 6;
pub const MAX_ROOMS: usize = 30;
pub const MAX_ROOM_MONSTERS: usize = 3;

pub const PLAYER: usize = 0;
