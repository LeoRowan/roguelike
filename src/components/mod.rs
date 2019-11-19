#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Fighter {
    pub max_hp: u32,
    pub hp: u32,
    pub defense: u32,
    pub power: u32,
}

impl Fighter {
    pub fn new(max_hp: u32, hp: u32, defense: u32, power: u32) -> Self {
        Fighter {
            max_hp,
            hp,
            defense,
            power,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Ai {
    Basic,
}
