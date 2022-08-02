use crate::q_learning::state::State;

pub trait Agent<S: State> {
    fn current_state(&self) -> &S;

    fn take_action(&mut self, action: &S::Action);

    fn is_completed(&self, step: i32) -> bool;
}
