use {
    super::{corpus::Corpus, query::Query},
    eframe::{App, CreationContext, Frame},
    egui::{CentralPanel, Context, ScrollArea},
    std::path::PathBuf,
};

pub struct Engine {
    corpus: Corpus,
    search: String,
    document_data: Vec<(String, PathBuf)>,
}

impl From<&CreationContext<'_>> for Engine {
    fn from(_creation_ctx: &CreationContext) -> Self {
        Self {
            corpus: Corpus::try_from(
                PathBuf::from(format!("{}\\java\\net", env!("CARGO_MANIFEST_DIR"))).as_path(),
            )
            .unwrap(),
            search: String::default(),
            document_data: vec![],
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
                    self.document_data = Query::from(self.search.as_str()).search(&self.corpus);
                }
            });
            ui.horizontal_top(|ui| ui.label(format!("{} Results", self.document_data.len())));
            ScrollArea::vertical().auto_shrink(false).show_rows(
                ui,
                0.,
                self.document_data.len(),
                |ui, row_count| {
                    for i in row_count {
                        if ui
                            .link(&self.document_data[i].0)
                            .clicked()
                        {
                            open::that(self.document_data[i].1.clone()).unwrap();
                        }
                    }
                },
            );
        });
    }
}
