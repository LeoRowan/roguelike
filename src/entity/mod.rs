use super::{map::Point, state::GameState};
use tcod::{
    colors::Color,
    console::{BackgroundFlag, Console},
};

/// This is a generic entity: the player, a monster, an item, the stairs...
/// It's always represented by a character on screen
#[derive(Debug)]
pub struct Entity {
    transform: Point,
    char: char,
    color: Color,
}

impl Entity {
    pub fn new(transform: Point, char: char, color: Color) -> Self {
        Entity {
            transform,
            char,
            color,
        }
    }

    pub fn set_transform(&mut self, transform: Point) {
        self.transform = transform;
    }

    /// Move the entity by the given amount
    pub fn translate(&mut self, dest: Point, state: &GameState) {
        let new_transform = self.transform + dest;

        if !state.map.out_of_bounds(new_transform) && !state.map.is_blocked_tile(new_transform) {
            self.transform = new_transform;
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
