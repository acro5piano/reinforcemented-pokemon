pub enum Type {
    Normal,
    Fire,
    Water,
    Electric,
    Grass,
    Ice,
    Fighting,
    Poison,
    Ground,
    Flying,
    Psychic,
    Bug,
    Rock,
    Ghost,
    // Not eligible for the first generation of Pokemon.
    // Dragon,
    // Dark,
    // Steel,
    // Fairy,
}

pub struct Movement {
    pub power: i32,
    pub accuracy: f32,
    pub pp: i32,
    pub type_: Type,
}

pub const THUNDERBOLT: Movement = Movement {
    power: 95,
    accuracy: 1.0,
    pp: 24,
    type_: Type::Electric,
};

pub const EARTHQUAKE: Movement = Movement {
    power: 100,
    accuracy: 1.0,
    pp: 16,
    type_: Type::Ground,
};

pub const SURF: Movement = Movement {
    power: 95,
    accuracy: 1.0,
    pp: 24,
    type_: Type::Water,
};
