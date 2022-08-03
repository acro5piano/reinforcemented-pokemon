use super::pokemon::Pokemon;

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub struct Player {
    pub active_pokemon_idx: i32,
    pub pokemon0: Pokemon,
    pub pokemon1: Pokemon,
}

impl Player {
    pub fn get_active_pokemon(&mut self) -> &mut Pokemon {
        match self.active_pokemon_idx {
            0 => &mut self.pokemon0,
            1 => &mut self.pokemon1,
            _ => panic!("This should not happen."),
        }
    }

    pub fn get_inactive_pokemon(&mut self) -> &mut Pokemon {
        match self.active_pokemon_idx {
            0 => &mut self.pokemon1,
            1 => &mut self.pokemon0,
            _ => panic!("This should not happen."),
        }
    }

    pub fn get_inactive_pokemon_unmut(&self) -> Pokemon {
        match self.active_pokemon_idx {
            0 => self.pokemon1,
            1 => self.pokemon0,
            _ => panic!("This should not happen."),
        }
    }

    pub fn change_active_pokemon(&mut self) {
        self.active_pokemon_idx = (self.active_pokemon_idx + 1) % 2;
    }
}

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub enum PokemonAction {
    Fight,
    Change,
    Choose(Pokemon, Pokemon),
}
