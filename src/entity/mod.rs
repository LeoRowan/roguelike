use super::{
    components::{Ai, Fighter},
    map::Point,
    state::GameState,
};
use tcod::{
    colors::Color,
    console::{BackgroundFlag, Console},
};

pub mod actions;

/// This is a generic entity: the player, a monster, an item, the stairs...
/// It's always represented by a character on screen
#[derive(Debug)]
pub struct Entity {
    pub position: Point,
    pub char: char,
    pub color: Color,
    pub name: String,
    pub blocks: bool,
    pub is_alive: bool,
    pub fighter: Option<Fighter>,
    pub ai: Option<Ai>,
}

impl Entity {
    /// Move the entity by the given amount
    pub fn translate(id: usize, dest: Point, state: &mut GameState) {
        let new_transform = state.entities[id].position + dest;

        if !state.map.out_of_bounds(new_transform)
            && !state.map.is_blocked_tile(new_transform, &state.entities)
        {
            state.entities[id].position = new_transform;
        }
    }

    pub fn move_towards(id: usize, target: Point, state: &mut GameState) {
        let dx = target.x - state.entities[id].position.x;
        let dy = target.y - state.entities[id].position.y;

        let distance = state.entities[id].position.distance_to(target);

        let x = (dx as f32 / distance).round() as i32;
        let y = (dy as f32 / distance).round() as i32;

        Self::translate(id, Point { x, y }, state);
    }

    pub fn new(position: Point, char: char, color: Color, name: &str, blocks: bool) -> Self {
        Entity {
            position,
            char,
            color,
            name: name.into(),
            blocks,
            is_alive: false,
            fighter: None,
            ai: None,
        }
    }

    pub fn with_fighter_component(mut self, fighter: Fighter) -> Self {
        self.fighter = Some(fighter);
        self
    }

    pub fn with_ai_component(mut self, ai: Ai) -> Self {
        self.ai = Some(ai);
        self
    }

    pub fn take_damage(&mut self, damage: i32) {
        if let Some(fighter) = self.fighter.as_mut() {
            fighter.hp -= damage;

            if fighter.hp <= 0 {
                self.is_alive = false;
                fighter.on_death.callback(self);
            }
        }
    }

    pub fn attack(&self, other: &mut Entity) {
        let ap = self.fighter.map_or(0, |f| f.power);
        let ad = other.fighter.map_or(0, |f| f.defense);
        let damage = ap - ad;

        if damage > 0 {
            println!("{} attacks {} for {} hp", self.name, other.name, damage);
            other.take_damage(damage);
        } else {
            println!("{} attacks {} but has no effect", self.name, other.name);
        }
    }

    /// set the color and draw the character that represents this entity at its
    /// given position
    pub fn draw<T: Console>(&self, con: &mut T) {
        con.set_default_foreground(self.color);
        con.put_char(
            self.position.x,
            self.position.y,
            self.char,
            BackgroundFlag::None,
        );
    }
}
