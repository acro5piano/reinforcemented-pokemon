use crate::q_learning::{agent::Agent, state::State};
use rand::prelude::*;
use std::collections::HashMap;
use std::hash::Hash;

pub type AgentFactory<S> = fn() -> Box<dyn Agent<S>>;
pub type ActionValue<A> = HashMap<A, f64>;
pub type Q<S, A> = HashMap<S, ActionValue<A>>;

pub struct Trainer<S, A> {
    pub initial_value: f64,
    pub alpha: f64,
    pub gamma: f64,
    pub e: f32,
    pub max_step: i32,
    pub episodes: i32,
    pub q: Q<S, A>,
    pub on_step: Option<fn(i32, &S, q: &Q<S, A>) -> ()>,
}

impl<S, A> Trainer<S, A>
where
    S: State<Action = A>,
    A: Eq + Hash + Clone,
{
    pub fn train(&mut self, agent_factory: AgentFactory<S>) {
        for _ in 0..self.episodes {
            let mut agent = agent_factory();

            for step in 0..self.max_step {
                if agent.is_completed(step) {
                    break;
                }

                let s_t = agent.current_state().clone();

                let action = {
                    let rng = rand::thread_rng().gen::<f32>();
                    let existing = self.q.get(&s_t);
                    // TODO: consider step is >0
                    if existing.is_some() && rng > self.e {
                        existing
                            .unwrap()
                            .iter()
                            .max_by(|(_, v1), (_, v2)| v1.partial_cmp(v2).unwrap())
                            .unwrap()
                            .0
                            .clone()
                    } else {
                        s_t.pick_random_action(step)
                    }
                };

                agent.take_action(step, &action);

                let s_t_next = agent.current_state().clone();
                let r_t_next = s_t_next.reward();

                let v_t = {
                    let old_value = self
                        .q
                        .get(&s_t)
                        .and_then(|m| m.get(&action))
                        .unwrap_or(&self.initial_value);
                    let max_next = self
                        .q
                        .get(&s_t_next)
                        .and_then(|m| m.values().max_by(|a, b| a.partial_cmp(b).unwrap()))
                        .unwrap_or(&self.initial_value);
                    (1.0 - self.alpha) * (old_value)
                        + self.alpha * (r_t_next + self.gamma * max_next)
                };

                self.q
                    .entry(s_t)
                    .or_insert_with(HashMap::new)
                    .insert(action, v_t);

                if let Some(on_step) = self.on_step {
                    on_step(step, &agent.current_state(), &self.q);
                }
            }
        }
    }
}
