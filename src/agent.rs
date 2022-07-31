use crate::state::{Action, State};

#[derive(Debug)]
pub struct Agent {
    pub state: State,
}

impl Agent {
    pub fn take_action(&mut self, action: &Action) -> () {
        match (self.state.x, self.state.y, action) {
            (3, 3, _) => {}

            (1, 1, Action::RIGHT) => self.state.x += 1,
            (1, 1, Action::DOWN) => self.state.y += 1,
            (1, 2, Action::UP) => self.state.y -= 1,
            (1, 2, Action::DOWN) => self.state.y += 1,
            (1, 3, Action::UP) => self.state.y -= 1,

            (2, 1, Action::RIGHT) => self.state.x += 1,
            (2, 1, Action::LEFT) => self.state.x -= 1,
            (2, 1, Action::DOWN) => self.state.y += 1,
            (2, 2, Action::UP) => self.state.y -= 1,
            (2, 2, Action::RIGHT) => self.state.x += 1,
            (2, 2, Action::DOWN) => self.state.y += 1,
            (2, 3, Action::UP) => self.state.y -= 1,
            (2, 3, Action::RIGHT) => self.state.x += 1,

            (3, 1, Action::DOWN) => self.state.y += 1,
            (3, 1, Action::LEFT) => self.state.x -= 1,
            (3, 2, Action::UP) => self.state.y -= 1,
            (3, 2, Action::LEFT) => self.state.x -= 1,

            (_, _, _) => {}
        }
    }
}
