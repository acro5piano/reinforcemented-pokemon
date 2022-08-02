use super::pokemon::Pokemon;

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub struct Player {
    pub pokemon1: Pokemon,
    pub pokemon2: Pokemon,
}

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub enum PokemonAction {
    Fight,
    Change,
}
