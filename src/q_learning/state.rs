use crate::util::pick_random_element_from_vec;
use std::hash::Hash;

pub trait State: Eq + Hash + Clone {
    type Action: Eq + Hash + Clone;

    fn new() -> Self;

    fn reward(&self) -> f64;

    fn actions(&self, step: i32) -> Vec<Self::Action>;

    fn pick_random_action(&self, step: i32) -> Self::Action {
        pick_random_element_from_vec(&self.actions(step)).clone()
    }
}
