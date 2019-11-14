use super::{Point, Rect, Tile, MAP_HEIGHT, MAP_WIDTH};

pub struct Map(Vec<Tile>);

impl Map {
    pub fn new() -> Self {
        let mut map = Map(vec![Tile::wall(); MAP_HEIGHT * MAP_WIDTH]);
        map.set_tile(Point::new(30, 22), Tile::wall());
        map.set_tile(Point::new(50, 22), Tile::wall());

        let room1 = Rect::new(Point::new(20, 15), 10, 15);
        let room2 = Rect::new(Point::new(50, 15), 10, 15);
        map.make_room(room1);
        map.make_room(room2);
        map.make_h_corridor(Point::new(25, 23), 55);

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

    pub fn make_room(&mut self, room: Rect) {
        for x in (room.p1.x + 1)..room.p2.x {
            for y in (room.p1.y + 1)..room.p2.y {
                self.set_tile(Point { x, y }, Tile::empty());
            }
        }
    }

    pub fn make_h_corridor(&mut self, start: Point, length: i32) {
        use std::cmp::{max, min};

        for x in min(start.x, length)..=max(start.x, length) {
            self.set_tile(Point { x, y: start.y }, Tile::empty());
        }
    }

    pub fn make_v_corridor(&mut self, start: Point, height: i32) {
        use std::cmp::{max, min};

        for y in min(start.y, height)..=max(start.y, height) {
            self.set_tile(Point { x: start.x, y }, Tile::empty());
        }
    }
}
