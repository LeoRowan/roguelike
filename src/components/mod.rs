use super::{entity::Entity, Game};
use tcod::colors;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Fighter {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
    pub on_death: DeathCallback,
}

impl Fighter {
    pub fn new(max_hp: i32, hp: i32, defense: i32, power: i32, on_death: DeathCallback) -> Self {
        Fighter {
            max_hp,
            hp,
            defense,
            power,
            on_death,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Ai {
    Basic,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DeathCallback {
    Player,
    Monster,
}

impl DeathCallback {
    pub fn callback(self, entity: &mut Entity, game: &mut Game) {
        use DeathCallback::*;

        let callback: fn(&mut Entity, &mut Game) = match self {
            Player => player_death,
            Monster => monster_death,
        };
        callback(entity, game);

        fn player_death(player: &mut Entity, game: &mut Game) {
            game.state.messages.add("You died!", colors::RED);

            player.char = '%';
            player.color = colors::DARK_RED;
        }

        fn monster_death(monster: &mut Entity, game: &mut Game) {
            game.state
                .messages
                .add(format!("{} is dead!", monster.name), colors::ORANGE);

            monster.char = '%';
            monster.color = colors::DARK_RED;
            monster.blocks = false;
            monster.fighter = None;
            monster.ai = None;
            monster.name = format!("Remains of {}", monster.name);
        }
    }
}
