use rand::prelude::*;
use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

mod agent;
mod state;

const INITIAL_VALUE: f64 = 2.0;

type ActionValue = HashMap<state::Action, f64>;
type Q = HashMap<state::State, ActionValue>;

fn main() {
    let mut q: Q = HashMap::new();

    for _ in 0..10000 {
        let mut agent = agent::Agent {
            state: state::State::new(),
        };

        let alpha = 0.1;
        let gamma = 0.9;

        for _step in 0..1000 {
            if agent.state.x == 3 && agent.state.y == 3 {
                break;
            }

            let s_t = agent.state.clone();
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
                    agent.state.pick_random_action()
                }
            };

            agent.take_action(&action);

            let s_t_next = agent.state.clone();
            let r_t_next = agent.state.reward();

            let v_t = {
                let old_value = q
                    .get(&s_t)
                    .and_then(|m| m.get(&action))
                    .unwrap_or(&INITIAL_VALUE);
                let max_next = q
                    .get(&s_t_next)
                    .and_then(|m| m.values().max_by(|a, b| a.partial_cmp(b).unwrap()))
                    .unwrap_or(&INITIAL_VALUE);
                (1.0 - alpha) * (old_value) + alpha * (r_t_next + gamma * max_next)
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
    }

    dbg!(&q);
}
