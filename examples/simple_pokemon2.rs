use rand;
use rand::prelude::*;
use reinforcemented_pokemon::pokemon::{
    battle::calculate_damage,
    player::{Player, PokemonAction},
    pokemon,
};
use reinforcemented_pokemon::q_learning::{agent::Agent, state::State, trainer::Trainer};
use std::collections::HashMap;

// Let's advance the rule to the next step.
// Think this basic Pokemon battle:
//
// - No critical, no miss, no status.
// - Player chooses **two** of the Pokemon to use.
// - Available Pokemons are:
//     Rhydon:
//       - Earthquake
//     Jolteon:
//       - Thunderbolt
//     Starmie:
//       - Surf
//   How the AI learn it?

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub struct SimplePokemonState {
    pub learner: Player,
    pub competitor: Player,
    pub is_choosing: bool,
}

impl State for SimplePokemonState {
    type Action = PokemonAction;

    fn new() -> Self {
        SimplePokemonState {
            is_choosing: true,
            learner: Player {
                active_pokemon_idx: 0,
                pokemon0: pokemon::RHYDON,
                pokemon1: pokemon::RHYDON,
            },
            competitor: Player {
                active_pokemon_idx: 0,
                pokemon0: pokemon::RHYDON,
                pokemon1: pokemon::RHYDON,
            },
        }
    }

    fn reward(&self) -> f64 {
        if self.competitor.pokemon0.hp == 0 && self.competitor.pokemon1.hp == 0 {
            1.0
        } else if self.learner.pokemon0.hp == 0 && self.learner.pokemon1.hp == 0 {
            -1.0
        } else {
            -0.01
        }
    }

    fn actions(&self, step: i32) -> Vec<PokemonAction> {
        match step {
            0 => {
                vec![
                    PokemonAction::Choose(pokemon::RHYDON, pokemon::JOLTEON),
                    PokemonAction::Choose(pokemon::RHYDON, pokemon::STARMIE),
                    PokemonAction::Choose(pokemon::JOLTEON, pokemon::RHYDON),
                    PokemonAction::Choose(pokemon::JOLTEON, pokemon::STARMIE),
                    PokemonAction::Choose(pokemon::STARMIE, pokemon::RHYDON),
                    PokemonAction::Choose(pokemon::STARMIE, pokemon::JOLTEON),
                ]
            }
            _ => {
                let inactive_pokemon = self.learner.get_inactive_pokemon_unmut();
                if inactive_pokemon.hp == 0 {
                    vec![PokemonAction::Fight]
                } else {
                    vec![PokemonAction::Fight, PokemonAction::Change]
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct SimplePokemonAgent {
    pub state: SimplePokemonState,
}

impl Agent<SimplePokemonState> for SimplePokemonAgent {
    fn current_state(&self) -> &SimplePokemonState {
        &self.state
    }

    fn take_action(&mut self, step: i32, action: &PokemonAction) {
        if step == 0 {
            // The competitor is stupid and take a random action.
            self.state.competitor.pokemon0 = pokemon::pick_random_pokemon();
            self.state.competitor.pokemon1 = pokemon::pick_random_pokemon();

            // The learner is smart and take the action.
            match action {
                PokemonAction::Choose(pokemon0, pokemon1) => {
                    self.state.learner.pokemon0 = *pokemon0;
                    self.state.learner.pokemon1 = *pokemon1;
                }
                _ => panic!("This should not happen."),
            };
        } else {
            self.state.is_choosing = false;

            // let learner_inactive_pokemon = self.state.competitor.get_inactive_pokemon();
            // assert!(
            //     (action == &PokemonAction::Change && learner_inactive_pokemon.hp == 0) == false
            // );

            // Random player
            let competitor_inactive_pokemon = self.state.competitor.get_inactive_pokemon();
            let competitor_action = if competitor_inactive_pokemon.hp > 0 {
                self.state.pick_random_action(step)
            } else {
                PokemonAction::Fight
            };

            match (&action, &competitor_action) {
                (PokemonAction::Fight, PokemonAction::Fight) => {
                    let mut learner_active_pokemon = self.state.learner.get_active_pokemon();
                    let mut competitor_active_pokemon = self.state.competitor.get_active_pokemon();
                    let is_learner_move_first = {
                        if &learner_active_pokemon.speed == &competitor_active_pokemon.speed {
                            let mut rng = rand::thread_rng();
                            rng.gen::<bool>()
                        } else {
                            &learner_active_pokemon.speed > &competitor_active_pokemon.speed
                        }
                    };
                    if is_learner_move_first {
                        competitor_active_pokemon.hp -=
                            calculate_damage(&learner_active_pokemon, &competitor_active_pokemon);
                        if competitor_active_pokemon.hp > 0 {
                            learner_active_pokemon.hp -= calculate_damage(
                                &competitor_active_pokemon,
                                &learner_active_pokemon,
                            );
                        }
                    } else {
                        learner_active_pokemon.hp -=
                            calculate_damage(&competitor_active_pokemon, &learner_active_pokemon);
                        if learner_active_pokemon.hp > 0 {
                            competitor_active_pokemon.hp -= calculate_damage(
                                &learner_active_pokemon,
                                &competitor_active_pokemon,
                            );
                        }
                    }
                }
                (PokemonAction::Fight, PokemonAction::Change) => {
                    self.state.competitor.change_active_pokemon();
                    let learner_active_pokemon = self.state.learner.get_active_pokemon();
                    let mut competitor_active_pokemon = self.state.competitor.get_active_pokemon();
                    competitor_active_pokemon.hp -=
                        calculate_damage(&learner_active_pokemon, &competitor_active_pokemon);
                }
                (PokemonAction::Change, PokemonAction::Fight) => {
                    self.state.learner.change_active_pokemon();
                    let mut learner_active_pokemon = self.state.learner.get_active_pokemon();
                    let competitor_active_pokemon = self.state.competitor.get_active_pokemon();
                    learner_active_pokemon.hp -=
                        calculate_damage(&competitor_active_pokemon, &learner_active_pokemon);
                }
                (PokemonAction::Change, PokemonAction::Change) => {
                    self.state.competitor.change_active_pokemon();
                    self.state.learner.change_active_pokemon();
                }
                _ => {
                    panic!("This should not happen.");
                }
            }
            if self.state.learner.get_active_pokemon().hp == 0 {
                self.state.learner.change_active_pokemon();
            }
            if self.state.competitor.get_active_pokemon().hp == 0 {
                self.state.competitor.change_active_pokemon();
            }
        }
    }

    fn is_completed(&self, _step: i32) -> bool {
        (self.state.learner.pokemon0.hp == 0 && self.state.learner.pokemon1.hp == 0)
            || (self.state.competitor.pokemon0.hp == 0 && self.state.competitor.pokemon1.hp == 0)
    }
}

fn main() {
    let mut trainer = Trainer {
        initial_value: 0.0,
        alpha: 0.5,
        gamma: 0.9,
        q: HashMap::new(),
        e: 0.9,
        max_step: 100,
        episodes: 100000,
        on_step: None,
    };

    // trainer.on_step = Some(|step, state: &SimplePokemonState, q| {
    //     use std::io::{stdin, Read};
    //     use std::thread::sleep;
    //     use std::time::Duration;
    //     print!("\x1B[2J\x1B[1;1H");
    //     println!("step: {}\n", step);
    //     dbg!(state);
    //     stdin().read(&mut [0]).unwrap();
    //     sleep(Duration::from_millis(20));
    // });

    trainer.train(|| {
        Box::new(SimplePokemonAgent {
            state: SimplePokemonState::new(),
        })
    });

    dbg!(&trainer.q);
}
