use super::pokemon;

pub fn calculate_damage(attacker: pokemon::Pokemon, defender: pokemon::Pokemon) -> i32 {
    match (attacker, defender) {
        (pokemon::RHYDON, pokemon::JOLTEON) => 388,
        (pokemon::RHYDON, pokemon::STARMIE) => 156,
        (pokemon::RHYDON, pokemon::RHYDON) => 251,
        (pokemon::RHYDON, pokemon::CLEFAIRY) => 217,

        // TODO: add all combinations.
        (_, _) => 100,
    }
}
