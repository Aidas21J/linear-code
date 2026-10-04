use crate::app::{
    algebra::{dynamic_vector::error_count, field::Field},
    base_app::BaseApp,
    core::{
        linear_code::LinearCode, memoryless_channel::MemorylessChannel as _,
        vec_codec::VecCodec as _,
    },
    ui,
};

use rayon::prelude::*;

pub trait TextApp: BaseApp
where
    Self: Sync,
    <Self::Code as LinearCode>::RowVector:
        Clone + Sync + Send + std::str::FromStr + std::fmt::Display,
    <<Self::Code as LinearCode>::Field as Field>::Element: std::fmt::Display,
{
    fn text_loop(&self) {
        loop {
            eprintln!();
            eprintln!("{}", "#".repeat(100));
            eprintln!("{}", "#".repeat(100));
            eprintln!("{}", "#".repeat(100));
            eprintln!();

            let text: String = {
                if ui::get_yes_no("Read from file?", None) {
                    ui::get_from_file()
                } else {
                    ui::get_text("Enter text")
                }
            };

            let (input_vecs, pad_size) = self.vec_codec().parse_vecs(text.into());

            let (errors_without_coding, duration_without_coding, text_without_coding) = {
                let start = std::time::Instant::now();
                let output_vecs: Vec<_> = input_vecs
                    .par_iter()
                    .cloned()
                    .map_init(|| rand::rng(), |rng, vec| self.channel().send_vec(vec, rng))
                    .collect();
                let duration = start.elapsed();

                (
                    error_count(input_vecs.iter(), output_vecs.iter()),
                    duration,
                    String::from_utf8_lossy(&self.vec_codec().parse_bytes(output_vecs, pad_size))
                        .to_string(),
                )
            };

            println!();
            println!("[WITHOUT CODING]:");
            println!("{text_without_coding}");

            let (errors_with_coding, duration_with_coding, text_with_coding) = {
                let start = std::time::Instant::now();
                let output_vecs: Vec<_> = input_vecs
                    .par_iter()
                    .map(|vec| self.code().encode(vec))
                    .map_init(|| rand::rng(), |rng, vec| self.channel().send_vec(vec, rng))
                    .map(|vec| self.code().decode(vec))
                    .collect();
                let duration = start.elapsed();

                (
                    error_count(input_vecs.iter(), output_vecs.iter()),
                    duration,
                    String::from_utf8_lossy(&self.vec_codec().parse_bytes(output_vecs, pad_size))
                        .to_string(),
                )
            };

            println!();
            println!("[WITH CODING]:");
            println!("{text_with_coding}");

            let duration_without_coding_us = duration_without_coding.as_micros();
            let duration_with_coding_us = duration_with_coding.as_micros();

            println!();
            println!("[STATS]:");
            println!("Errors without coding: {errors_without_coding}");
            println!("   Errors with coding: {errors_with_coding}");
            println!();
            println!("Duration without coding: {duration_without_coding_us} us");
            println!("   Duration with coding: {duration_with_coding_us} us");
            println!();

            if !ui::get_yes_no("Try another text?", Some(true)) {
                return;
            }
        }
    }
}
