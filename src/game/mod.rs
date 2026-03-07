mod ui;

use self::ui::*;

use crate::{model::*, prelude::*, render::*, ui::context::UiContext};

pub struct GameState {
    context: Context,
    ui_context: UiContext,

    render: GameRender,
    model: Model,
    ui: GameUI,
}

impl GameState {
    pub fn new(context: Context) -> Self {
        Self {
            render: GameRender::new(context.clone()),
            model: Model::new(),
            ui: GameUI::new(),

            ui_context: UiContext::new(context.clone()),
            context,
        }
    }
}

impl geng::State for GameState {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(
            framebuffer,
            Some(self.context.assets.palette.background),
            None,
            None,
        );

        self.render.draw_game(&self.model, framebuffer);
    }

    fn update(&mut self, delta_time: f64) {
        let delta_time = Time::new(delta_time as f32);
        self.model.update(delta_time);
    }
}
