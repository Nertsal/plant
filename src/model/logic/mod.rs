mod actions;
mod plants;

pub use self::{actions::*, plants::*};

use super::*;

impl Model {
    pub fn update(&mut self, delta_time: Time) {
        self.update_plants(delta_time);
    }
}
