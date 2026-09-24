use crate::app::{app::App as _, binary_app::BinaryApp};

pub(crate) mod app;

#[show_image::main]
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let app = BinaryApp::new().unwrap();

    match args.get(1).map_or("", String::as_str) {
        "" | "vector" => app.single_vector_loop(),
        "text" => app.text_loop(),
        "image" => app.image_loop().unwrap(),
        unknown_arg => panic!("Unknown argument: {unknown_arg}"),
    }
}
