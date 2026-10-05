use std::path::Path;

use crate::app::{algebra::field::Field, buffer_app::BufferApp, core::linear_code::LinearCode, ui};

use image::{GenericImageView, ImageReader, RgbImage};

pub trait ImageApp: BufferApp + Sync
where
    <Self::Code as LinearCode>::RowVector: Sync + Send,
    <Self::Code as LinearCode>::RowVector: Clone + std::str::FromStr + std::fmt::Display,
    <<Self::Code as LinearCode>::Field as Field>::Element: std::fmt::Display,
{
    fn image_loop(&self) {
        loop {
            eprintln!();
            eprintln!("{}", "#".repeat(100));
            eprintln!("{}", "#".repeat(100));
            eprintln!("{}", "#".repeat(100));
            eprintln!();

            let original_filepath: String = ui::get_from_line("Enter filepath");

            let (filepath_without_extension, extension) = {
                let path = Path::new(&original_filepath);

                let filepath = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("output");

                let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("png");

                (filepath, extension)
            };

            let img = {
                let file = match ImageReader::open(&original_filepath) {
                    Ok(f) => f,
                    Err(e) => {
                        eprintln!("Failed to open image {}: {}", original_filepath, e);
                        continue;
                    }
                };

                match file.decode() {
                    Ok(i) => i,
                    Err(e) => {
                        eprintln!("Failed to decode image {}: {}", original_filepath, e);
                        continue;
                    }
                }
            };

            let original_rgb = img.to_rgb8();
            let (width, height) = img.dimensions();

            eprintln!();
            eprintln!("{}", "-".repeat(100));
            eprintln!("[WITHOUT CODING] {}", "-".repeat(100 - 17));
            eprintln!("{}", "-".repeat(100));
            eprintln!();

            let errors_without_coding = {
                let code = false;
                let output_filepath =
                    format!("{filepath_without_extension}-without-code.{extension}");

                let (output_bytes, error_count) =
                    self.send_bytes(original_rgb.as_flat_samples().samples.into(), code);

                if let Some(output_image) = RgbImage::from_raw(width, height, output_bytes) {
                    match output_image.save(&output_filepath) {
                        Ok(_) => eprintln!("Saved to {output_filepath}."),
                        Err(e) => eprintln!("Failed to save to {output_filepath}: {e}"),
                    }
                } else {
                    eprintln!("Failed to create (not coded) output image.");
                }

                error_count
            };

            eprintln!();
            eprintln!("{}", "-".repeat(100));
            eprintln!("[WITH CODING] {}", "-".repeat(100 - 14));
            eprintln!("{}", "-".repeat(100));
            eprintln!();

            let errors_with_coding = {
                let code = true;
                let output_filepath = format!("{filepath_without_extension}-with-code.{extension}");

                let (output_bytes, error_count) =
                    self.send_bytes(original_rgb.as_flat_samples().samples.into(), code);

                if let Some(output_image) = RgbImage::from_raw(width, height, output_bytes) {
                    match output_image.save(&output_filepath) {
                        Ok(_) => eprintln!("Saved to {output_filepath}."),
                        Err(e) => eprintln!("Failed to save to {output_filepath}: {e}"),
                    }
                } else {
                    eprintln!("Failed to create (not coded) output image.");
                }

                error_count
            };

            eprintln!();
            eprintln!("{}", "-".repeat(100));
            eprintln!("[RESULTS] {}", "-".repeat(100 - 10));
            eprintln!("{}", "-".repeat(100));
            eprintln!();

            eprint!("Errors without coding: ");
            println!("{errors_without_coding}");

            eprint!("   Errors with coding: ");
            println!("{errors_with_coding}");

            eprintln!();

            if !ui::get_yes_no("Try another image?", Some(true)) {
                return;
            }
        }
    }
}
