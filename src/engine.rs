use {
    eframe::{App, CreationContext, Frame},
    egui::{CentralPanel, Context},
};

pub struct Engine;

impl From<&CreationContext<'_>> for Engine {
    fn from(creation_ctx: &CreationContext) -> Self {
        Self
    }
}

impl App for Engine {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
        });
    }
}
