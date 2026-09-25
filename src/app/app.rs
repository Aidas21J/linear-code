use image::{GenericImageView, ImageReader, RgbImage};
use show_image::{ImageInfo, ImageView, create_window};

use crate::app::{
    algebra::{drvec::DRVec, field::Field, randomizable::Randomizable},
    core::{channel, linear_code},
    ui,
};

pub trait App<LinearCode, Channel>
where
    LinearCode: linear_code::LinearCode,
    Channel: channel::Channel<LinearCode::Field>,
    <LinearCode::Field as Field>::Element: Randomizable + std::fmt::Display,
{
    fn code(&self) -> &LinearCode;
    fn channel(&self) -> &Channel;

    fn bytes_to_vecs(&self, bytes: Vec<u8>) -> (Vec<DRVec<LinearCode::Field>>, usize);
    fn vecs_to_bytes(&self, vecs: Vec<DRVec<LinearCode::Field>>, pad_size: usize) -> Vec<u8>;

    fn send_without_coding(
        &self,
        input_vecs: &Vec<DRVec<LinearCode::Field>>,
        pad_size: usize,
        rng: &mut impl rand::Rng,
    ) -> (Vec<u8>, usize) {
        let output_vecs: Vec<_> = input_vecs
            .iter()
            .map(|v| self.channel().send(v.clone(), rng))
            .collect();

        let error_count: usize = input_vecs
            .iter()
            .zip(output_vecs.iter())
            .map(|(a, b)| DRVec::hamming_distance(a, b))
            .sum();

        (self.vecs_to_bytes(output_vecs, pad_size), error_count)
    }

    fn send_with_coding(
        &self,
        input_vecs: &Vec<DRVec<LinearCode::Field>>,
        pad_size: usize,
        rng: &mut impl rand::Rng,
    ) -> (Vec<u8>, usize) {
        let output_vecs: Vec<_> = input_vecs
            .iter()
            .map(|v| self.code().encode(v))
            .map(|v| self.channel().send(v, rng))
            .map(|v| self.code().decode(v))
            .collect();

        let error_count: usize = input_vecs
            .iter()
            .zip(output_vecs.iter())
            .map(|(a, b)| DRVec::hamming_distance(a, b))
            .sum();

        (self.vecs_to_bytes(output_vecs, pad_size), error_count)
    }

    fn single_vector_loop(&self) {
        let mut rng = rand::rng();

        loop {
            eprintln!();
            eprintln!("{}", "#".repeat(100));
            eprintln!("{}", "#".repeat(100));
            eprintln!("{}", "#".repeat(100));
            eprintln!();

            let initial = ui::get_row_vector("Enter vector", self.code().k());

            eprintln!();
            eprintln!("{}", "#".repeat(100));
            eprintln!();
            eprintln!("Encoding...");

            let encoded = self.code().encode(&initial);

            println!("Encoded: {encoded}");

            eprintln!();
            eprintln!("{}", "#".repeat(100));
            eprintln!();
            eprintln!("Sending through the channel...");

            let received = {
                let mut received = self.channel().send(encoded.clone(), &mut rng);
                let mut encoded_to_received_err_str =
                    ui::vector_error_string(&encoded, &received, ' ', '^');

                println!();
                println!(" Encoded: {encoded}");
                println!("Received: {received}");
                println!("  Errors: {encoded_to_received_err_str}");

                println!();
                let edit_received =
                    ui::get_yes_no("Do you want to edit received vector?", Some(false));
                if edit_received {
                    received = ui::get_row_vector("Enter received", self.code().n());
                    encoded_to_received_err_str =
                        ui::vector_error_string(&encoded, &received, ' ', '^');

                    eprintln!();
                    eprintln!("{}", "#".repeat(100));
                    println!();
                    println!(" Encoded: {encoded}");
                    println!("Received: {received}");
                    println!("  Errors: {encoded_to_received_err_str}");
                }

                received
            };

            eprintln!();
            eprintln!("{}", "#".repeat(100));
            eprintln!();
            eprintln!("Decoding...");

            let decoded = self.code().decode(received.clone());

            println!("Decoded: {decoded}");

            eprintln!();
            eprintln!("{}", "#".repeat(100));
            eprintln!();

            let initial_to_final_err_str = ui::vector_error_string(&initial, &decoded, ' ', '^');

            println!("Initial vector: {initial}");
            println!("  Final vector: {decoded}");
            println!("        Errors: {initial_to_final_err_str}");

            let transition_errors = DRVec::hamming_distance(&encoded, &received);
            let decoded_vector_errors = DRVec::hamming_distance(&initial, &decoded);

            println!();
            println!("       Errors from channel: {transition_errors}.");
            println!("Errors in the final vector: {decoded_vector_errors}.");
            println!();

            if !ui::get_yes_no("Try another vector?", Some(true)) {
                return;
            }
        }
    }

    fn text_loop(&self) {
        let mut rng = rand::rng();

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

            let (input_vecs, pad_size) = self.bytes_to_vecs(text.as_bytes().to_vec());

            let (text_without_coding, errors_without_coding) = {
                let (output_raw, error_count) =
                    self.send_without_coding(&input_vecs, pad_size, &mut rng);
                (
                    String::from_utf8_lossy(&output_raw).to_string(),
                    error_count,
                )
            };

            println!();
            println!("[WITHOUT CODING]:");
            println!("{text_without_coding}");

            let (text_with_coding, errors_with_coding) = {
                let (output_raw, error_count) =
                    self.send_with_coding(&input_vecs, pad_size, &mut rng);
                (
                    String::from_utf8_lossy(&output_raw).to_string(),
                    error_count,
                )
            };

            println!();
            println!("[WITH CODING]:");
            println!("{text_with_coding}");

            println!();
            println!("[STATS]:");
            println!("Errors without coding: {errors_without_coding}");
            println!("   Errors with coding: {errors_with_coding}");
            println!();

            if !ui::get_yes_no("Try another text?", Some(true)) {
                return;
            }
        }
    }

    fn image_loop(&self) {
        let mut rng = rand::rng();

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

            let (input_vecs, pad_size) =
                self.bytes_to_vecs(original_rgb.as_flat_samples().samples.to_vec());

            let (image_without_coding, errors_without_coding) = {
                let (output_raw, error_count) =
                    self.send_without_coding(&input_vecs, pad_size, &mut rng);

                if let Some(output_image) = RgbImage::from_raw(width, height, output_raw) {
                    (output_image, error_count)
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

            let (image_with_coding, errors_with_coding) = {
                let (output_raw, error_count) =
                    self.send_with_coding(&input_vecs, pad_size, &mut rng);

                if let Some(output_image) = RgbImage::from_raw(width, height, output_raw) {
                    (output_image, error_count)
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

            println!();
            println!("[STATS]:");
            println!("Errors without coding: {errors_without_coding}");
            println!("   Errors with coding: {errors_with_coding}");
            println!();

            if !ui::get_yes_no("Try another image?", Some(true)) {
                return;
            }
        }
    }
}
