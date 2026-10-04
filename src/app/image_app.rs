use crate::app::{
    algebra::{dynamic_vector::error_count, field::Field},
    base_app::BaseApp,
    core::{
        linear_code::LinearCode, memoryless_channel::MemorylessChannel as _,
        vec_codec::VecCodec as _,
    },
    ui,
};

use image::{GenericImageView, ImageReader, RgbImage};
use show_image::{ImageInfo, ImageView, create_window};

use rayon::prelude::*;

pub trait ImageApp: BaseApp
where
    Self: Sync,
    <Self::Code as LinearCode>::RowVector:
        Clone + Sync + Send + std::str::FromStr + std::fmt::Display,
    <<Self::Code as LinearCode>::Field as Field>::Element: std::fmt::Display,
{
    fn image_loop(&self) {
        loop {
            let original_window = match create_window("Original", Default::default()) {
                Ok(w) => w,
                Err(e) => {
                    eprintln!("Failed to create window: {e}.");
                    return;
                }
            };
            let without_coding_window = match create_window("Without coding", Default::default()) {
                Ok(w) => w,
                Err(e) => {
                    eprintln!("Failed to create window: {e}.");
                    return;
                }
            };
            let with_coding_window = match create_window("With coding", Default::default()) {
                Ok(w) => w,
                Err(e) => {
                    eprintln!("Failed to create window: {e}.");
                    return;
                }
            };

            eprintln!();
            eprintln!("{}", "#".repeat(100));
            eprintln!("{}", "#".repeat(100));
            eprintln!("{}", "#".repeat(100));
            eprintln!();

            let filename: String = ui::get_from_line("Enter filepath");
            let img = {
                let file = match ImageReader::open(&filename) {
                    Ok(f) => f,
                    Err(e) => {
                        eprintln!("Failed to open image {}: {}", filename, e);
                        continue;
                    }
                };

                match file.decode() {
                    Ok(i) => i,
                    Err(e) => {
                        eprintln!("Failed to decode image {}: {}", filename, e);
                        continue;
                    }
                }
            };

            let original_rgb = img.to_rgb8();
            let (width, height) = img.dimensions();
            let image_info = ImageInfo::rgb8(width, height);

            if let Err(e) =
                original_window.set_image(&filename, ImageView::new(image_info, &original_rgb))
            {
                eprintln!("Failed to show original image: {e}");
                continue;
            }

            let (input_vecs, pad_size) = self
                .vec_codec()
                .parse_vecs(original_rgb.as_flat_samples().samples.into());

            let (errors_without_coding, duration_without_coding, image_without_coding) = {
                let start = std::time::Instant::now();
                let output_vecs: Vec<_> = input_vecs
                    .par_iter()
                    .cloned()
                    .map_init(|| rand::rng(), |rng, vec| self.channel().send_vec(vec, rng))
                    .collect();
                let duration = start.elapsed();

                let error_count = error_count(input_vecs.iter(), output_vecs.iter());
                let output_raw = self.vec_codec().parse_bytes(output_vecs, pad_size);

                if let Some(output_image) = RgbImage::from_raw(width, height, output_raw) {
                    (error_count, duration, output_image)
                } else {
                    eprintln!("Failed to create (not coded) output image");
                    continue;
                }
            };

            if let Err(e) = without_coding_window
                .set_image(&filename, ImageView::new(image_info, &image_without_coding))
            {
                eprintln!("Failed to show coded image: {e}");
                continue;
            }

            let (errors_with_coding, duration_with_coding, image_with_coding) = {
                let start = std::time::Instant::now();
                let output_vecs: Vec<_> = input_vecs
                    .par_iter()
                    .map(|vec| self.code().encode(vec))
                    .map_init(|| rand::rng(), |rng, vec| self.channel().send_vec(vec, rng))
                    .map(|vec| self.code().decode(vec))
                    .collect();
                let duration = start.elapsed();

                let error_count = error_count(input_vecs.iter(), output_vecs.iter());
                let output_raw = self.vec_codec().parse_bytes(output_vecs, pad_size);

                if let Some(output_image) = RgbImage::from_raw(width, height, output_raw) {
                    (error_count, duration, output_image)
                } else {
                    eprintln!("Failed to create (coded) output image");
                    continue;
                }
            };

            if let Err(e) = with_coding_window
                .set_image(&filename, ImageView::new(image_info, &image_with_coding))
            {
                eprintln!("Failed to show coded image: {e}");
                continue;
            }

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

            if !ui::get_yes_no("Try another image?", Some(true)) {
                return;
            }
        }
    }
}
