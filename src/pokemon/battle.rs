use super::pokemon;

pub fn calculate_damage(attacker: &pokemon::Pokemon, defender: &pokemon::Pokemon) -> u32 {
    let damage = match (attacker.id, defender.id) {
        (1, 1) => 251,
        (1, 2) => 388,
        (1, 3) => 156,
        (1, 4) => 217,

        (2, 1) => 0,
        (2, 2) => 55,
        (2, 3) => 240,
        (2, 4) => 163,

        (3, 1) => 703,
        (3, 2) => 105,
        (3, 3) => 55,
        (3, 4) => 153,

        (4, 1) => 27,
        (4, 2) => 86,
        (4, 3) => 72,
        (4, 4) => 98,

        (_, _) => panic!("This should not happen."),
    };

    if damage > defender.hp {
        defender.hp
    } else {
        damage
    }
}
