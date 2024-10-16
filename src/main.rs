mod corpus;
mod document;
mod engine;
mod query;

use {
    corpus::Corpus,
    eframe::{self, NativeOptions},
    engine::Engine,
    query::Query,
    std::path::PathBuf,
};

fn main() {
    // let corpus =
    //     Corpus::try_from(PathBuf::from(format!("{}/java", env!("CARGO_MANIFEST_DIR"))).as_path())
    //         .unwrap();

    // let qry = Query::from("Arraylist");
    // let res = qry.search(&corpus);
    // println!("{:#?}", res);
    eframe::run_native(
        "thRUSTer",
        NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(Engine::from(cc)))),
    )
    .unwrap();
}
