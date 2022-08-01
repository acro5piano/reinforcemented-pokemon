use std::hash::Hash;

pub trait State: Eq + Hash + Clone {
    type Action: Eq + Hash + Clone;

    fn new() -> Self;

    fn reward(&self) -> f64;

    fn actions(&self) -> Vec<Self::Action>;

    fn pick_random_action(&self) -> Self::Action {
        let actions = self.actions();
        let a_t = rand::random::<usize>() % actions.len();
        actions[a_t].clone()
    }
}
