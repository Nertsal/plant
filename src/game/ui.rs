use super::*;

use crate::ui::widget::WidgetState;

pub struct GameUI {
    pub scissors: WidgetState,
}

impl GameUI {
    pub fn new() -> Self {
        Self {
            scissors: WidgetState::new(),
        }
    }

    pub fn layout(&mut self, screen: Aabb2<f32>, context: &mut UiContext) {}
}
