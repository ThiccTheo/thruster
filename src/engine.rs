use {
    super::{corpus::Corpus, document::Document, query::Query},
    eframe::{App, CreationContext, Frame},
    egui::{CentralPanel, Context, ScrollArea},
    rfd::FileDialog,
    std::path::PathBuf,
};

pub enum Engine {
    Home,
    Search {
        corpus: Corpus,
        search: String,
        result: Box<[Document]>,
    },
}

impl From<&CreationContext<'_>> for Engine {
    fn from(creation_ctx: &CreationContext) -> Self {
        creation_ctx.egui_ctx.set_zoom_factor(2.);
        Self::Home
    }
}

impl App for Engine {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        match self {
            Self::Home => {
                CentralPanel::default().show(ctx, |ui| {
                    ui.vertical_centered_justified(|ui| {
                        ui.heading("Thruster - Local Search Engine");
                        if ui.button("Select folder(s) to index").clicked() {
                            let Ok(corpus) = Corpus::try_from(
                                FileDialog::new()
                                    .pick_folders()
                                    .iter()
                                    .flatten()
                                    .map(PathBuf::as_path)
                                    .collect::<Vec<_>>()
                                    .as_slice(),
                            ) else {
                                return;
                            };

                            *self = Self::Search {
                                corpus,
                                search: String::default(),
                                result: Box::default(),
                            };
                        }
                    });
                });
            }
            Self::Search {
                corpus,
                search,
                result,
            } => {
                CentralPanel::default().show(ctx, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.label("Query: ");
                        ui.text_edit_singleline(search);
                        if ui.button("Search").clicked() {
                            *result = Query::from(search.as_str()).search(corpus);
                        }
                    });
                    ui.horizontal_top(|ui| ui.label(format!("{} Results", result.len())));
                    ScrollArea::vertical().auto_shrink(false).show_rows(
                        ui,
                        0.,
                        result.len(),
                        |ui, row_count| {
                            for i in row_count {
                                if ui.link(result[i].title()).clicked() {
                                    open::that(result[i].path()).unwrap();
                                }
                                ui.add_space(15.);
                            }
                        },
                    );
                });
            }
        }
    }
}
