use super::{
    super::entity::Entity, Point, Rect, Tile, MAP_HEIGHT, MAP_WIDTH, MAX_ROOMS, ROOM_MAX_SIZE,
    ROOM_MIN_SIZE,
};
use rand::{thread_rng, Rng};

pub struct Map(Vec<Tile>);

impl Map {
    pub fn new(player: &mut Entity) -> Self {
        let mut map = Map(vec![Tile::wall(); MAP_HEIGHT * MAP_WIDTH]);
        let mut rooms = vec![];

        for _ in 0..MAX_ROOMS {
            let w = thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
            let h = thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
            let x = thread_rng().gen_range(0, MAP_WIDTH - w) as i32;
            let y = thread_rng().gen_range(0, MAP_HEIGHT - h) as i32;

            let room = Rect::new(Point { x, y }, w as i32, h as i32);
            let failed = rooms.iter().any(|other| room.intersects_with(other));

            if !failed {
                map.make_room(room);

                let center = room.center();
                if rooms.is_empty() {
                    // This is the first room, where the player starts
                    player.set_transform(center);
                } else {
                    // For all other rooms, connect to previous with corridor
                    let prev_center = rooms[rooms.len() - 1].center();

                    if rand::random() {
                        map.make_h_corridor(center, prev_center.x);
                        map.make_v_corridor(prev_center, center.y);
                    } else {
                        map.make_v_corridor(center, prev_center.y);
                        map.make_h_corridor(prev_center, center.x);
                    }
                }

                rooms.push(room);
            }
        }
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
