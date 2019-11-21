use super::{
    super::{
        components::{Ai, DeathCallback, Fighter},
        constants::*,
        entity::Entity,
    },
    Point, Rect, Tile,
};
use rand::{thread_rng, Rng};
use tcod::colors;

pub struct Map(Vec<Tile>);

impl Map {
    pub fn new(entities: &mut Vec<Entity>) -> Self {
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
                    let player = &mut entities[PLAYER];
                    // This is the first room, where the player starts
                    player.position = center;
                } else {
                    // For all other rooms, place entities and connect to
                    // previous with corridor
                    map.place_entities(&room, entities);
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

    fn tile_at_mut(&mut self, x: i32, y: i32) -> &mut Tile {
        let idx = self.tile_idx(x, y);
        &mut self.0[idx]
    }

    pub fn set_tile(&mut self, Point { x, y }: Point, tile: Tile) {
        let idx = self.tile_idx(x, y);
        self.0[idx] = tile;
    }

    pub fn set_explored_tile(&mut self, Point { x, y }: Point) {
        let tile = self.tile_at_mut(x, y);
        tile.explored = true;
    }

    pub fn is_blocked_tile(&self, point: Point, entities: &Vec<Entity>) -> bool {
        self.tile_at(point.x, point.y).blocked
            || entities.iter().any(|x| x.blocks && x.position == point)
    }

    pub fn is_blocked_sight_tile(&self, Point { x, y }: Point) -> bool {
        self.tile_at(x, y).block_sight
    }

    pub fn is_explored_tile(&self, Point { x, y }: Point) -> bool {
        self.tile_at(x, y).explored
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

    fn place_entities(&self, room: &Rect, entities: &mut Vec<Entity>) {
        let num_monsters = rand::thread_rng().gen_range(0, MAX_ROOM_MONSTERS + 1);

        for _ in 0..num_monsters {
            let transform = Point {
                x: rand::thread_rng().gen_range(room.p1.x + 1, room.p2.x),
                y: rand::thread_rng().gen_range(room.p1.y + 1, room.p2.y),
            };

            if !self.is_blocked_tile(transform, entities) {
                let mut monster = if rand::random::<f32>() < 0.8 {
                    Entity::new(transform, 'o', colors::DESATURATED_GREEN, "orc", true)
                        .with_ai_component(Ai::Basic)
                        .with_fighter_component(Fighter::new(10, 10, 0, 3, DeathCallback::Monster))
                } else {
                    Entity::new(transform, 'T', colors::DARKER_GREEN, "troll", true)
                        .with_ai_component(Ai::Basic)
                        .with_fighter_component(Fighter::new(16, 16, 1, 4, DeathCallback::Monster))
                };
                monster.is_alive = true;

                entities.push(monster);
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
