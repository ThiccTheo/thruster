use {
    super::{corpus::Corpus, document::Document, query::Query},
    eframe::Frame,
    egui::{CentralPanel, Context, ScrollArea},
    rfd::FileDialog,
};

pub enum State {
    Home,
    Search {
        corpus: Corpus,
        search: String,
        result: Vec<Document>,
    },
}

impl State {
    pub fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        match self {
            Self::Home => {
                CentralPanel::default().show(ctx, |ui| {
                    ui.heading("Thruster - Local Search Engine");
                    if ui.button("Select folder(s) to index").clicked() {
                        let Some(corpus) = FileDialog::new()
                            .pick_folders()
                            .into_iter()
                            .flatten()
                            .flat_map(|path| Corpus::try_from(path.as_path()))
                            .reduce(|mut accumulator, corpus| {
                                accumulator.extend(corpus);
                                accumulator
                            })
                        else {
                            return;
                        };

                        *self = Self::Search {
                            corpus,
                            search: String::default(),
                            result: vec![],
                        };
                    }
                });
            }
            Self::Search {
                corpus,
                search,
                result,
            } => {
                CentralPanel::default().show(ctx, |ui| {
                    ui.horizontal_top(|ui| {
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
                            }
                        },
                    );
                });
            }
        }
    }
}
