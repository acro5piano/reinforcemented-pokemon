// use super::movement;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Pokemon {
    pub hp: i32,
    // pub attack: i32,
    // pub defense: i32,
    // pub special: i32,
    // pub speed: i32,
    // pub movement1: movement::Movement,
}

pub const RHYDON: Pokemon = Pokemon {
    hp: 413,
    // attack: 130,
    // defense: 120,
    // special: 45,
    // speed: 40,
    // movement1: movement::THUNDERBOLT,
};

pub const JOLTEON: Pokemon = Pokemon {
    hp: 333,
    // attack: 65,
    // defense: 60,
    // special: 110,
    // speed: 130,
    // movement1: movement::THUNDERBOLT,
};

pub const STARMIE: Pokemon = Pokemon {
    hp: 323,
    // attack: 75,
    // defense: 85,
    // special: 100,
    // speed: 115,
    // movement1: movement::SURF,
};

pub const CLEFAIRY: Pokemon = Pokemon {
    hp: 343,
    // movement1: movement::SURF,
};
