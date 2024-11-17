use {
    super::{corpus::Corpus, query::Query},
    eframe::{App, CreationContext, Frame},
    egui::{CentralPanel, Context, ScrollArea},
    std::path::PathBuf,
};

pub struct Engine {
    corpus: Corpus,
    search: String,
    links: Vec<PathBuf>,
}

impl From<&CreationContext<'_>> for Engine {
    fn from(_creation_ctx: &CreationContext) -> Self {
        Self {
            corpus: Corpus::try_from(
                PathBuf::from(format!("{}\\java\\net", env!("CARGO_MANIFEST_DIR"))).as_path(),
            )
            .unwrap(),
            search: String::default(),
            links: vec![],
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
                    self.links = Query::from(self.search.as_str()).search(&self.corpus);
                }
            });
            ui.horizontal_top(|ui| ui.label(format!("{} Results", self.links.len())));
            ScrollArea::vertical().auto_shrink(false).show_rows(
                ui,
                0.,
                self.links.len(),
                |ui, row_count| {
                    for i in row_count {
                        if ui.link(self.links[i].as_path().to_str().unwrap()).clicked() {
                            open::that(self.links[i].clone()).unwrap();
                        }
                    }
                },
            );
        });
    }
}
