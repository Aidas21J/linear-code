use image::{GenericImageView, ImageReader, RgbImage};
use show_image::{ImageInfo, ImageView, create_window};

use crate::app::{
    algebra::{drvec::DRVec, field::Field},
    core::{
        linear_code::LinearCode,
        memoryless_channel::MemorylessChannel,
        vec_buffer::{VecBuffer, error_count},
        vec_codec::VecCodec,
    },
    ui,
};

pub trait App
where
    <<Self::Code as LinearCode>::Field as Field>::Element: std::fmt::Display,
{
    type Code: LinearCode;
    type Channel: MemorylessChannel<<<Self::Code as LinearCode>::Field as Field>::Element>;
    type VecCodec: VecCodec<<Self::Code as LinearCode>::Field>;

    fn code(&self) -> &Self::Code;
    fn channel(&self) -> &Self::Channel;
    fn vec_codec(&self) -> &Self::VecCodec;

    fn single_vector_loop(&self) {
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
                let mut received = self.channel().send_vec(encoded.clone(), &mut rand::rng());
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

            let (errors_without_coding, text_without_coding) = {
                let output_vecs: Vec<_> = input_vecs
                    .clone()
                    .send_through_channel(self.channel())
                    .collect();
                (
                    error_count(input_vecs.iter(), output_vecs.iter()),
                    String::from_utf8_lossy(&self.vec_codec().parse_bytes(output_vecs, pad_size))
                        .to_string(),
                )
            };

            println!();
            println!("[WITHOUT CODING]:");
            println!("{text_without_coding}");

            let (errors_with_coding, text_with_coding) = {
                let output_vecs: Vec<_> = input_vecs
                    .clone()
                    .send_through_channel_with_coding(self.channel(), self.code())
                    .collect();
                (
                    error_count(input_vecs.iter(), output_vecs.iter()),
                    String::from_utf8_lossy(&self.vec_codec().parse_bytes(output_vecs, pad_size))
                        .to_string(),
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

            let (errors_without_coding, image_without_coding) = {
                let output_vecs: Vec<_> = input_vecs
                    .clone()
                    .send_through_channel(self.channel())
                    .collect();
                let error_count = error_count(input_vecs.iter(), output_vecs.iter());
                let output_raw = self.vec_codec().parse_bytes(output_vecs, pad_size);

                if let Some(output_image) = RgbImage::from_raw(width, height, output_raw) {
                    (error_count, output_image)
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

            let (errors_with_coding, image_with_coding) = {
                let output_vecs: Vec<_> = input_vecs
                    .clone()
                    .send_through_channel_with_coding(self.channel(), self.code())
                    .collect();
                let error_count = error_count(input_vecs.iter(), output_vecs.iter());
                let output_raw = self.vec_codec().parse_bytes(output_vecs, pad_size);

                if let Some(output_image) = RgbImage::from_raw(width, height, output_raw) {
                    (error_count, output_image)
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
