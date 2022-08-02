use crate::util::pick_random_element_from_vec;
use std::hash::Hash;

pub trait State: Eq + Hash + Clone {
    type Action: Eq + Hash + Clone;

    fn new() -> Self;

    fn reward(&self) -> f64;

    fn actions(&self) -> Vec<Self::Action>;

    fn pick_random_action(&self) -> Self::Action {
        pick_random_element_from_vec(&self.actions()).clone()
    }
}
