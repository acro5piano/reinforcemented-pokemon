// use super::movement;
use rand;

#[derive(Copy, Debug, Hash, Clone, Eq, PartialEq)]
pub struct Pokemon {
    pub id: i32,
    pub hp: i32,
    // pub attack: i32,
    // pub defense: i32,
    // pub special: i32,
    pub speed: i32,
    // pub movement1: movement::Movement,
}

pub const RHYDON: Pokemon = Pokemon {
    id: 1,
    hp: 413,
    // attack: 130,
    // defense: 120,
    // special: 45,
    speed: 40,
    // movement1: movement::THUNDERBOLT,
};

pub const JOLTEON: Pokemon = Pokemon {
    id: 2,
    hp: 333,
    // attack: 65,
    // defense: 60,
    // special: 110,
    speed: 130,
    // movement1: movement::THUNDERBOLT,
};

pub const STARMIE: Pokemon = Pokemon {
    id: 3,
    hp: 323,
    // attack: 75,
    // defense: 85,
    // special: 100,
    speed: 115,
    // movement1: movement::SURF,
};

pub const CLEFAIRY: Pokemon = Pokemon {
    id: 4,
    hp: 343,
    speed: 35,
    // movement1: movement::SURF,
};

pub fn pick_random_pokemon() -> Pokemon {
    let pokemons = vec![RHYDON, JOLTEON, STARMIE, CLEFAIRY];
    let a_t = rand::random::<usize>() % pokemons.len();
    pokemons[a_t]
}
