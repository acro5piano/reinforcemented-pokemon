mod q_learning;

use crate::q_learning::{agent::Agent, state::State, trainer::train};
use std::collections::HashMap;

// The state is this maze
//
//     1   2   3
//   |------------|
// 1 | S :    :   |
//   |------------|
// 2 |   X    :   |
//   |---------xxx|
// 3 |   X    : G |
//   |------------|
#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub struct MazeState {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub enum MazeAction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl State for MazeState {
    type Action = MazeAction;

    fn new() -> Self {
        MazeState { x: 1, y: 1 }
    }

    fn reward(&self) -> f64 {
        match (self.x, self.y) {
            (3, 3) => 1.0,
            _ => 0.0,
        }
    }

    fn actions(&self) -> Vec<MazeAction> {
        vec![
            MazeAction::UP,
            MazeAction::LEFT,
            MazeAction::DOWN,
            MazeAction::RIGHT,
        ]
    }

    #[cfg(feature = "visual")]
    fn render(&self) {
        println!();
        for y in 1..4 {
            print!("{}", "|");
            for x in 1..4 {
                let elm = if x == self.x && y == self.y {
                    "*"
                } else if x == 3 && y == 3 {
                    "G"
                } else {
                    " "
                };
                let wall = match (x, y) {
                    (1, 2) => "x",
                    (1, 3) => "x",
                    (_, _) => ":",
                };
                print!("{}{}", elm, wall);
            }
            println!("|");
        }
    }
}

#[derive(Debug)]
pub struct MazeAgent {
    pub state: MazeState,
}

impl Agent<MazeState> for MazeAgent {
    fn current_state(&self) -> &MazeState {
        &self.state
    }

    fn take_action(&mut self, action: &MazeAction) {
        match (self.state.x, self.state.y, action) {
            (3, 3, _) => {}

            (1, 1, MazeAction::RIGHT) => self.state.x += 1,
            (1, 1, MazeAction::DOWN) => self.state.y += 1,
            (1, 2, MazeAction::UP) => self.state.y -= 1,
            (1, 2, MazeAction::DOWN) => self.state.y += 1,
            (1, 3, MazeAction::UP) => self.state.y -= 1,

            (2, 1, MazeAction::RIGHT) => self.state.x += 1,
            (2, 1, MazeAction::LEFT) => self.state.x -= 1,
            (2, 1, MazeAction::DOWN) => self.state.y += 1,
            (2, 2, MazeAction::UP) => self.state.y -= 1,
            (2, 2, MazeAction::RIGHT) => self.state.x += 1,
            (2, 2, MazeAction::DOWN) => self.state.y += 1,
            (2, 3, MazeAction::UP) => self.state.y -= 1,
            (2, 3, MazeAction::RIGHT) => self.state.x += 1,

            (3, 1, MazeAction::DOWN) => self.state.y += 1,
            (3, 1, MazeAction::LEFT) => self.state.x -= 1,
            (3, 2, MazeAction::UP) => self.state.y -= 1,
            (3, 2, MazeAction::LEFT) => self.state.x -= 1,

            (_, _, _) => {}
        }
    }
}

type ActionValue = HashMap<MazeAction, f64>;
type Q = HashMap<MazeState, ActionValue>;

fn main() {
    let mut q: Q = HashMap::new();

    for _ in 0..10000 {
        let mut agent = MazeAgent {
            state: MazeState::new(),
        };

        for _step in 0..1000 {
            if agent.state.x == 3 && agent.state.y == 3 {
                break;
            }

            train(&mut q, &mut agent);

            #[cfg(feature = "visual")]
            {
                use std::thread::sleep;
                use std::time::Duration;
                print!("\x1B[2J\x1B[1;1H");
                println!("step: {}\n", _step);
                agent.state.render();
                println!();
                dbg!(&q.get(&state::State { x: 1, y: 1 }).unwrap());
                sleep(Duration::from_millis(20));
            }
        }
    }

    dbg!(&q);
}
