use {
    super::{corpus::Corpus, query::Query},
    eframe::{App, CreationContext, Frame},
    egui::{CentralPanel, Context, ScrollArea},
    std::path::PathBuf,
};

pub struct Engine {
    qry: Query,
    corpus: Corpus,
    links: Vec<PathBuf>,
}

impl From<&CreationContext<'_>> for Engine {
    fn from(_creation_ctx: &CreationContext) -> Self {
        Self {
            links: vec![],
            qry: Query::default(),
            corpus: Corpus::try_from(
                PathBuf::from(format!("{}/java/math", env!("CARGO_MANIFEST_DIR"))).as_path(),
            )
            .unwrap(),
        }
    }
}

impl App for Engine {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_top(|ui| {
                ui.label("Query: ");
                ui.text_edit_singleline(self.qry.qry());
                if ui.button("Search").clicked() {
                    self.links = self.qry.search(&self.corpus);
                }
            });
            ScrollArea::vertical().auto_shrink(false).show_rows(
                ui,
                30.,
                self.links.len(),
                |ui, row_ct| {
                    for i in row_ct {
                        if ui.link(self.links[i].as_path().to_str().unwrap()).clicked() {
                            open::that(self.links[i].clone()).unwrap();
                        }
                    }
                },
            );
        });
    }
}
