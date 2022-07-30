use std::collections::HashMap;
use std::io::{stdin, Read};

mod agent;
mod state;

const INITIAL_VALUE: f64 = 2.0;

type ActionValue = HashMap<state::Action, f64>;
type Q = HashMap<state::State, ActionValue>;

fn main() {
    let mut agent = agent::Agent {
        state: state::State::new(),
    };

    let alpha = 0.2;
    let gamma = 0.01;

    let mut q: Q = HashMap::new();

    for step in 0..10000 {
        println!("-----------------------------");
        dbg!(&step);

        let s_t = agent.state.clone();
        let action = agent.state.pick_action();
        dbg!(&action);

        agent.take_action(&action);
        dbg!(&agent.state);

        let s_t_next = agent.state.clone();
        let r_t_next = agent.state.reward();

        let v_t = {
            let old_value = q.get(&s_t).and_then(|m| m.get(&action));
            dbg!(&old_value);
            let new_action_values = q.get(&s_t_next);
            let max_next = new_action_values
                .and_then(|m| m.values().max_by(|a, b| a.partial_cmp(b).unwrap()))
                .unwrap_or(&INITIAL_VALUE);
            old_value.map_or(INITIAL_VALUE, |x| {
                x + alpha * (r_t_next + gamma * max_next - x)
            })
        };
        dbg!(&v_t);

        q.entry(s_t)
            .or_insert_with(HashMap::new)
            .insert(action, v_t);

        dbg!(&q);
        println!("-----------------------------");

        stdin().read(&mut [0]).unwrap();
    }
    dbg!(&q);
}
