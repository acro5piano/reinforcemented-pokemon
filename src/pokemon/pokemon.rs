use super::movement;

pub struct Pokemon {
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub special: i32,
    pub speed: i32,
    pub moves: Vec<movement::Movement>,
}

pub const JOLTEON: Pokemon = Pokemon {
    hp: 65,
    attack: 65,
    defense: 60,
    special: 110,
    speed: 130,
    moves: vec![movement::THUNDERBOLT],
};
