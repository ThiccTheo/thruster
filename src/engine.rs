use {
    super::{corpus::Corpus, document::Document, query::Query},
    eframe::{App, CreationContext, Frame},
    egui::{CentralPanel, Context, ScrollArea},
    std::path::PathBuf,
};

pub struct Engine {
    corpus: Corpus,
    search: String,
    result: Vec<Document>,
}

impl From<&CreationContext<'_>> for Engine {
    fn from(creation_ctx: &CreationContext) -> Self {
        creation_ctx.egui_ctx.set_zoom_factor(1.25);

        Self {
            corpus: Corpus::try_from(
                PathBuf::from(format!("{}\\java\\net", env!("CARGO_MANIFEST_DIR"))).as_path(),
            )
            .unwrap(),
            search: String::default(),
            result: vec![],
        }
    }
}

impl App for Engine {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_top(|ui| {
                ui.label("Query: ");
                ui.text_edit_singleline(&mut self.search);
                if ui.button("Search").clicked() {
                    self.result = Query::from(self.search.as_str()).search(&self.corpus);
                }
            });
            ui.horizontal_top(|ui| ui.label(format!("{} Results", self.result.len())));
            ScrollArea::vertical().auto_shrink(false).show_rows(
                ui,
                0.,
                self.result.len(),
                |ui, row_count| {
                    for i in row_count {
                        if ui.link(self.result[i].title()).clicked() {
                            open::that(self.result[i].path()).unwrap();
                        }
                    }
                },
            );
        });
    }
}
