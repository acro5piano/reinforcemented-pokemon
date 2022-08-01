use crate::q_learning::{agent::Agent, state::State};
use rand::prelude::*;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

pub type ActionValue<A> = HashMap<A, f64>;
pub type Q<S, A> = HashMap<S, ActionValue<A>>;

const INITIAL_VALUE: f64 = 2.0;
const ALPHA: f64 = 0.1;
const GAMMA: f64 = 0.9;

pub fn train<S, A>(q: &mut Q<S, A>, agent: &mut dyn Agent<S>)
where
    S: State + State<Action = A>,
    A: Eq + Hash + Clone + Debug,
{
    let s_t = agent.current_state().clone();
    let action = {
        let rng = rand::thread_rng().gen::<f32>();
        let existing = q.get(&s_t);
        if existing.is_some() && rng > 0.8 {
            existing
                .unwrap()
                .iter()
                .max_by(|(_, v1), (_, v2)| v1.partial_cmp(v2).unwrap())
                .unwrap()
                .0
                .clone()
        } else {
            s_t.pick_random_action()
        }
    };

    agent.take_action(&action);

    let s_t_next = agent.current_state().clone();
    let r_t_next = s_t_next.reward();

    let v_t = {
        let old_value = q
            .get(&s_t)
            .and_then(|m| m.get(&action))
            .unwrap_or(&INITIAL_VALUE);
        let max_next = q
            .get(&s_t_next)
            .and_then(|m| m.values().max_by(|a, b| a.partial_cmp(b).unwrap()))
            .unwrap_or(&INITIAL_VALUE);
        (1.0 - ALPHA) * (old_value) + ALPHA * (r_t_next + GAMMA * max_next)
    };

    q.entry(s_t)
        .or_insert_with(HashMap::new)
        .insert(action, v_t);

    #[cfg(feature = "visual")]
    {
        print!("\x1B[2J\x1B[1;1H");
        println!("step: {}\n", _step);
        agent.state.render();
        println!();
        dbg!(&q.get(&state::State { x: 1, y: 1 }).unwrap());
        sleep(Duration::from_millis(20));
    }
}
