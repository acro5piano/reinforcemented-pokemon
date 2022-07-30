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
    DoNothing,
}

impl State {
    pub fn new() -> State {
        State { x: 1, y: 1 }
    }

    pub fn reward(&self) -> f64 {
        let distance_power_2 = ((1 - self.x).pow(2) + (1 - self.y).pow(2)) as f64;
        10.0 * distance_power_2.sqrt()
    }

    pub fn pick_action(&mut self) -> Action {
        // TODO: consider already learned state
        let actions = vec![
            Action::UP,
            Action::LEFT,
            Action::DOWN,
            Action::RIGHT,
            Action::DoNothing,
        ];
        let a_t = rand::random::<usize>() % actions.len();
        actions[a_t].clone()
    }
}
