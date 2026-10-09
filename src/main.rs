use crate::app::{
    analysis_app::AnalysisApp, app::App, image_app::ImageApp as _,
    single_vector_app::SingleVectorApp as _, text_app::TextApp as _,
};

pub(crate) mod app;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let app = App::new();

    match args.get(1).map_or("", String::as_str) {
        "" | "vector" => app.single_vector_loop(),
        "text" => app.text_loop(),
        "image" => app.image_loop(),
        "analysis" => app.analysis_loop(),
        unknown_arg => panic!("Unknown argument: {unknown_arg}"),
    }
}
