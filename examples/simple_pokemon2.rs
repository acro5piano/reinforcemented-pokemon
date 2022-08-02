use reinforcemented_pokemon::pokemon::{battle, player, pokemon};
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
    pub learner: player::Player,
    pub competitor: player::Player,
}

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub enum SimplePokemonAction {
    ChooseRhydon,
    ChooseJolteon,
    ChooseStarmie,
    ChooseClefairy,
}

impl State for SimplePokemonState {
    type Action = SimplePokemonAction;

    fn new() -> Self {
        SimplePokemonState {
            learner: player::Player {
                pokemon: Pokemon::Rhydon,
            },
            competitor: player::Player {
                pokemon: Pokemon::Rhydon,
            },
        }
    }

    fn reward(&self) -> f64 {
        match (&self.learner.pokemon, &self.competitor.pokemon) {
            (Pokemon::Rhydon, Pokemon::Rhydon) => 0.0,
            (Pokemon::Rhydon, Pokemon::Jolteon) => 1.0,
            (Pokemon::Rhydon, Pokemon::Starmie) => -1.0,
            (Pokemon::Rhydon, Pokemon::Clefairy) => 1.0,

            (Pokemon::Jolteon, Pokemon::Rhydon) => -1.0,
            (Pokemon::Jolteon, Pokemon::Jolteon) => 0.0,
            (Pokemon::Jolteon, Pokemon::Starmie) => 1.0,
            (Pokemon::Jolteon, Pokemon::Clefairy) => 1.0,

            (Pokemon::Starmie, Pokemon::Rhydon) => 1.0,
            (Pokemon::Starmie, Pokemon::Jolteon) => -1.0,
            (Pokemon::Starmie, Pokemon::Starmie) => 0.0,
            (Pokemon::Starmie, Pokemon::Clefairy) => 1.0,

            (Pokemon::Clefairy, Pokemon::Rhydon) => -1.0,
            (Pokemon::Clefairy, Pokemon::Jolteon) => -1.0,
            (Pokemon::Clefairy, Pokemon::Starmie) => -1.0,
            (Pokemon::Clefairy, Pokemon::Clefairy) => 0.0,
        }
    }

    fn actions(&self) -> Vec<player::PokemonAction> {
        vec![player::PokemonAction::Fight, player::PokemonAction::Change]
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

    fn take_action(&mut self, action: &SimplePokemonAction) {
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
