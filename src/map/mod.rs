use tcod::colors::Color;

mod geometry;
mod map;
mod tile;

pub const MAP_WIDTH: usize = 80;
pub const MAP_HEIGHT: usize = 45;

pub const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
pub const COLOR_DARK_GROUND: Color = Color {
    r: 50,
    g: 50,
    b: 150,
};

pub use geometry::{Point, Rect};
pub use map::Map;
pub use tile::Tile;
