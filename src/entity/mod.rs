use super::{map::Point, state::GameState};
use tcod::{
    colors::Color,
    console::{BackgroundFlag, Console},
};

pub mod actions;

/// This is a generic entity: the player, a monster, an item, the stairs...
/// It's always represented by a character on screen
#[derive(Debug)]
pub struct Entity {
    transform: Point,
    char: char,
    color: Color,
    name: String,
    blocks: bool,
    alive: bool,
}

impl Entity {
    pub fn new(transform: Point, char: char, color: Color, name: &str, blocks: bool) -> Self {
        Entity {
            transform,
            char,
            color,
            name: name.into(),
            blocks,
            alive: false,
        }
    }

    pub fn get_transform(&self) -> Point {
        self.transform
    }

    pub fn set_transform(&mut self, transform: Point) {
        self.transform = transform;
    }

    pub fn blocks(&self) -> bool {
        self.blocks
    }

    pub fn set_alive(&mut self, alive: bool) {
        self.alive = alive;
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    /// Move the entity by the given amount
    pub fn translate(id: usize, dest: Point, state: &mut GameState) {
        let new_transform = state.entities[id].transform + dest;

        if !state.map.out_of_bounds(new_transform)
            && !state.map.is_blocked_tile(new_transform, &state.entities)
        {
            state.entities[id].set_transform(new_transform);
        }
    }

    /// set the color and draw the character that represents this entity at its
    /// given position
    pub fn draw<T: Console>(&self, con: &mut T) {
        con.set_default_foreground(self.color);
        con.put_char(
            self.transform.x,
            self.transform.y,
            self.char,
            BackgroundFlag::None,
        );
    }
}
