mod corpus;
mod document;
mod engine;
mod query;

use {
    eframe::{self, NativeOptions},
    engine::Engine,
};

fn main() {
    eframe::run_native(
        "thRUSTer",
        NativeOptions::default(),
        Box::new(|creation_ctx| Ok(Box::new(Engine::from(creation_ctx)))),
    )
    .unwrap();
}
