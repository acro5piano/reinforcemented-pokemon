use super::pokemon;

pub fn calculate_damage(attacker: &pokemon::Pokemon, defender: &pokemon::Pokemon) -> i32 {
    match (attacker, defender) {
        (&pokemon::RHYDON, &pokemon::JOLTEON) => 388,
        (&pokemon::RHYDON, &pokemon::STARMIE) => 156,
        (&pokemon::RHYDON, &pokemon::RHYDON) => 251,
        (&pokemon::RHYDON, &pokemon::CLEFAIRY) => 217,

        (&pokemon::JOLTEON, &pokemon::RHYDON) => 0,
        (&pokemon::JOLTEON, &pokemon::JOLTEON) => 55,
        (&pokemon::JOLTEON, &pokemon::STARMIE) => 240,
        (&pokemon::JOLTEON, &pokemon::CLEFAIRY) => 163,

        (&pokemon::STARMIE, &pokemon::RHYDON) => 703,
        (&pokemon::STARMIE, &pokemon::JOLTEON) => 105,
        (&pokemon::STARMIE, &pokemon::STARMIE) => 55,
        (&pokemon::STARMIE, &pokemon::CLEFAIRY) => 153,

        (&pokemon::CLEFAIRY, &pokemon::RHYDON) => 27,
        (&pokemon::CLEFAIRY, &pokemon::JOLTEON) => 86,
        (&pokemon::CLEFAIRY, &pokemon::STARMIE) => 72,
        (&pokemon::CLEFAIRY, &pokemon::CLEFAIRY) => 98,

        // TODO: add all combinations.
        (_, _) => 100,
    }
}
