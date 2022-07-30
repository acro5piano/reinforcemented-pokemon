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
pub struct State {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub enum Action {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl State {
    pub fn new() -> State {
        State { x: 1, y: 1 }
    }

    pub fn reward(&self) -> f64 {
        if self.x == 3 && self.y == 3 {
            1.0
        } else {
            0.0
            // let distance_power_2 = ((1 - self.x).pow(2) + (1 - self.y).pow(2)) as f64;
            // distance_power_2.sqrt()
        }

        // let distance_power_2 = ((1 - self.x).pow(2) + (1 - self.y).pow(2)) as f64;
        // 10.0 * distance_power_2.sqrt() - 0.001 * step as f64
    }

    pub fn actions(&mut self) -> Vec<Action> {
        vec![Action::UP, Action::LEFT, Action::DOWN, Action::RIGHT]
    }

    pub fn pick_random_action(&mut self) -> Action {
        // TODO: consider already learned state
        let actions = self.actions();
        let a_t = rand::random::<usize>() % actions.len();
        actions[a_t].clone()
    }
}
