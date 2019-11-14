use super::{Point, Tile, MAP_HEIGHT, MAP_WIDTH};

pub struct Map(Vec<Tile>);

impl Map {
    pub fn new() -> Self {
        let mut map = Map(vec![Tile::empty(); MAP_HEIGHT * MAP_WIDTH]);
        map.set_tile(Point::new(30, 22), Tile::wall());
        map.set_tile(Point::new(50, 22), Tile::wall());

        map
    }

    fn tile_idx(&self, x: i32, y: i32) -> usize {
        (y * MAP_WIDTH as i32 + x) as usize
    }

    fn tile_at(&self, x: i32, y: i32) -> &Tile {
        &self.0[self.tile_idx(x, y)]
    }

    pub fn set_tile(&mut self, Point { x, y }: Point, tile: Tile) {
        let idx = self.tile_idx(x, y);
        self.0[idx] = tile;
    }

    pub fn is_blocked_tile(&self, Point { x, y }: Point) -> bool {
        self.tile_at(x, y).blocked
    }

    pub fn is_blocked_sight_tile(&self, Point { x, y }: Point) -> bool {
        self.tile_at(x, y).block_sight
    }

    pub fn out_of_bounds(&self, Point { x, y }: Point) -> bool {
        !(self.tile_idx(x, y) < self.0.len())
    }
}
