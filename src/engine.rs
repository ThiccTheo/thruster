use {
    super::{corpus::Corpus, state::State},
    eframe::{App, CreationContext, Frame},
    egui::Context,
    std::path::PathBuf,
};

pub struct Engine {
    state: State,
}

impl From<&CreationContext<'_>> for Engine {
    fn from(creation_ctx: &CreationContext) -> Self {
        creation_ctx.egui_ctx.set_zoom_factor(1.25);

        Self {
            // state: State::Search {
            //     corpus: Corpus::try_from(
            //         PathBuf::from(format!("{}\\java\\net", env!("CARGO_MANIFEST_DIR"))).as_path(),
            //     )
            //     .unwrap(),
            //     search: String::default(),
            //     result: vec![],
            // },
            state: State::Home,
        }
    }
}

impl App for Engine {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {

        self.state.update(ctx, frame);
    }
}
