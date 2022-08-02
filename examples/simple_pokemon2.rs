use reinforcemented_pokemon::pokemon::{
    battle,
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
//     Clefairy:
//       - Body Slam
// - Obviously, Clefairy is the worst Pokemon. However, the rest of Pokemon is like Rock, Paper, Scissors.
//   How the AI learn it?

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub struct SimplePokemonState {
    pub learner: Player,
    pub competitor: Player,
}

impl State for SimplePokemonState {
    type Action = PokemonAction;

    fn new() -> Self {
        SimplePokemonState {
            learner: Player {
                pokemon1: pokemon::RHYDON,
                pokemon2: pokemon::RHYDON,
            },
            competitor: Player {
                pokemon1: pokemon::RHYDON,
                pokemon2: pokemon::RHYDON,
            },
        }
    }

    fn reward(&self) -> f64 {
        match (&self.competitor.pokemon1.hp, &self.competitor.pokemon2.hp) {
            (0, 0) => 1.0,
            _ => 0.0,
        }
    }

    fn actions(&self) -> Vec<PokemonAction> {
        vec![PokemonAction::Fight, PokemonAction::Change]
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

    fn take_action(&mut self, step: i32, action: &SimplePokemonAction) {
        if step == 0 {
            // The competitor is stupid and take a random action.
            self.state.competitor.pokemon = match self.state.pick_random_action() {
                SimplePokemonAction::ChooseRhydon => Pokemon::Rhydon,
                SimplePokemonAction::ChooseJolteon => Pokemon::Jolteon,
                SimplePokemonAction::ChooseStarmie => Pokemon::Starmie,
                SimplePokemonAction::ChooseClefairy => Pokemon::Clefairy,
            };

            // The learner is smart and take the action.
            self.state.learner = match action {
                SimplePokemonAction::ChooseRhydon => Player {
                    pokemon: Pokemon::Rhydon,
                },
                SimplePokemonAction::ChooseJolteon => Player {
                    pokemon: Pokemon::Jolteon,
                },
                SimplePokemonAction::ChooseStarmie => Player {
                    pokemon: Pokemon::Starmie,
                },
                SimplePokemonAction::ChooseClefairy => Player {
                    pokemon: Pokemon::Clefairy,
                },
            };
        }
    }

    fn is_completed(&self, step: i32) -> bool {
        step > 2
    }
}

fn main() {
    let mut trainer = Trainer {
        initial_value: 0.0,
        alpha: 0.1,
        gamma: 0.9,
        q: HashMap::new(),
        e: 0.8,
        max_step: 10,
        episodes: 1000000,
        // on_step: None,
        on_step: Some(|step, _state: &SimplePokemonState, q| {
            use std::thread::sleep;
            use std::time::Duration;
            print!("\x1B[2J\x1B[1;1H");
            println!("step: {}\n", step);
            dbg!(q);
            sleep(Duration::from_millis(20));
        }),
    };

    trainer.train(|| {
        Box::new(SimplePokemonAgent {
            state: SimplePokemonState::new(),
        })
    });

    dbg!(&trainer.q);
}
