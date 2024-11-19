use {
    super::state::State,
    eframe::{App, CreationContext, Frame},
    egui::Context,
};

pub struct Engine {
    state: State,
}

impl From<&CreationContext<'_>> for Engine {
    fn from(creation_ctx: &CreationContext) -> Self {
        creation_ctx.egui_ctx.set_zoom_factor(1.25);

        Self { state: State::Home }
    }
}

impl App for Engine {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        self.state.update(ctx, frame);
    }
}
