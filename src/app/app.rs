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

    fn single_vector_loop(&self) {
        let mut rng = rand::rng();

        loop {
            println!();
            println!("{}", "#".repeat(100));
            println!("{}", "#".repeat(100));
            println!("{}", "#".repeat(100));
            println!();

            let initial = ui::get_row_vector("Enter vector", self.code().k());

            println!();
            println!("{}", "#".repeat(100));
            println!();
            println!("Encoding...");

            let encoded = self.code().encode(&initial);

            println!("Encoded: {encoded}");

            println!();
            println!("{}", "#".repeat(100));
            println!();
            println!("Sending through the channel...");

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

                    println!();
                    println!("{}", "#".repeat(100));
                    println!();
                    println!(" Encoded: {encoded}");
                    println!("Received: {received}");
                    println!("  Errors: {encoded_to_received_err_str}");
                }

                received
            };

            println!();
            println!("{}", "#".repeat(100));
            println!();
            println!("Decoding...");

            let decoded = self.code().decode(received.clone()).unwrap();

            println!("Decoded: {decoded}");

            println!();
            println!("{}", "#".repeat(100));
            println!();

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
        let code = self.code();
        let channel = self.channel();

        loop {
            println!();
            println!("{}", "#".repeat(100));
            println!("{}", "#".repeat(100));
            println!("{}", "#".repeat(100));
            println!();

            let text: String = {
                if ui::get_yes_no("Read from file?", Some(false)) {
                    ui::get_from_file()
                } else {
                    ui::get_text("Enter text")
                }
            };

            let (input_vecs, pad_size) = self.bytes_to_vecs(text.as_bytes().to_vec());

            let vecs_without_code: Vec<_> = input_vecs
                .iter()
                .map(|v| channel.send(v.clone(), &mut rng))
                .collect();

            let vecs_with_code: Vec<_> = input_vecs
                .iter()
                .map(|v| code.encode(v))
                .map(|v| channel.send(v, &mut rng))
                .map(|v| code.decode(v))
                .map(|v| v.unwrap_or(DRVec::zeros(code.k())))
                .collect();

            let errors_without_code: usize = input_vecs
                .iter()
                .zip(vecs_without_code.iter())
                .map(|(a, b)| DRVec::hamming_distance(a, b))
                .sum();

            let errors_with_code: usize = input_vecs
                .iter()
                .zip(vecs_with_code.iter())
                .map(|(a, b)| DRVec::hamming_distance(a, b))
                .sum();

            let without_code_output = {
                let without_code_bytes = self.vecs_to_bytes(vecs_without_code, pad_size);
                debug_assert_eq!(text.as_bytes().len(), without_code_bytes.len());

                String::from_utf8_lossy(&without_code_bytes).to_string()
            };

            let with_code_output = {
                let with_code_bytes = self.vecs_to_bytes(vecs_with_code, pad_size);
                debug_assert_eq!(text.as_bytes().len(), with_code_bytes.len());

                String::from_utf8_lossy(&with_code_bytes).to_string()
            };

            println!();
            println!("[WITHOUT CODE]:");
            println!("{without_code_output}");
            println!();
            println!("[WITH CODE]:");
            println!("{with_code_output}");
            println!();
            println!("[STATS]:");
            println!("Errors without code: {errors_without_code}");
            println!("   Errors with code: {errors_with_code}");
            println!();

            if !ui::get_yes_no("Try another text?", Some(true)) {
                return;
            }
        }
    }
}
