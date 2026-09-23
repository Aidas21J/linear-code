use crate::app::{app::App as _, binary_app::BinaryApp};

pub(crate) mod app;

fn main() {
    let app = BinaryApp::new().unwrap();
    app.single_vector_loop();
}
