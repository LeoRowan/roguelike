use super::{
    components::{Ai, Fighter},
    map::Point,
    state::GameState,
    Game,
};
use tcod::{
    colors::{self, Color},
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
    pub fn translate(id: usize, dest: Point, state: &mut GameState, entities: &mut Vec<Entity>) {
        let new_transform = entities[id].position + dest;

        if !state.map.out_of_bounds(new_transform)
            && !state.map.is_blocked_tile(new_transform, entities)
        {
            entities[id].position = new_transform;
        }
    }

    pub fn move_towards(
        id: usize,
        target: Point,
        state: &mut GameState,
        entities: &mut Vec<Entity>,
    ) {
        let dx = target.x - entities[id].position.x;
        let dy = target.y - entities[id].position.y;

        let distance = entities[id].position.distance_to(target);

        let x = (dx as f32 / distance).round() as i32;
        let y = (dy as f32 / distance).round() as i32;

        Self::translate(id, Point { x, y }, state, entities);
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

    pub fn take_damage(&mut self, damage: i32, game: &mut Game) {
        if let Some(fighter) = self.fighter.as_mut() {
            fighter.hp -= damage;

            if fighter.hp <= 0 {
                self.is_alive = false;
                fighter.on_death.callback(self, game);
            }
        }
    }

    pub fn attack(&self, other: &mut Entity, game: &mut Game) {
        let ap = self.fighter.map_or(0, |f| f.power);
        let ad = other.fighter.map_or(0, |f| f.defense);
        let damage = ap - ad;

        if damage > 0 {
            game.state.messages.add(
                format!("{} attacks {} for {} hp", self.name, other.name, damage),
                colors::WHITE,
            );
            other.take_damage(damage, game);
        } else {
            game.state.messages.add(
                format!("{} attacks {} but has no effect", self.name, other.name),
                colors::WHITE,
            );
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
