mod corpus;
mod document;
mod engine;
mod query;
mod state;

use {
    eframe::{self, NativeOptions},
    egui::IconData,
    engine::Engine,
    image::{GenericImageView, ImageReader},
    std::path::PathBuf,
};

fn main() {
    eframe::run_native(
        "Thruster",
        {
            let mut options = NativeOptions::default();
            options.viewport = options.viewport.with_icon({
                let image = ImageReader::open(
                    PathBuf::from(format!("{}\\assets\\icon.png", env!("CARGO_MANIFEST_DIR")))
                        .as_path(),
                )
                .unwrap()
                .decode()
                .unwrap();

                let (width, height) = image.dimensions();

                IconData {
                    width,
                    height,
                    rgba: image.into_rgba8().into_vec(),
                }
            });
            options
        },
        Box::new(|creation_ctx| Ok(Box::new(Engine::from(creation_ctx)))),
    )
    .unwrap();
}
