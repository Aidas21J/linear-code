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

            let encoded = self.code().encode(initial.clone());

            println!("Encoded: {encoded}");

            println!();
            println!("{}", "#".repeat(100));
            println!();
            println!("Sending through the channel...");

            let mut received = self.channel().send_drvec(encoded.clone(), &mut rng);
            let mut encoded_to_received_err_str =
                ui::vector_error_string(&encoded, &received, ' ', '^');

            println!();
            println!(" Encoded: {encoded}");
            println!("Received: {received}");
            println!("  Errors: {encoded_to_received_err_str}",);

            println!();
            let edit_received = ui::get_yes_no("Do you want to edit received vector?", Some(false));
            if edit_received {
                received = ui::get_row_vector("Enter received", self.code().n());
                encoded_to_received_err_str =
                    ui::vector_error_string(&encoded, &received, ' ', '^');

                println!();
                println!("{}", "#".repeat(100));
                println!();
                println!(" Encoded: {encoded}");
                println!("Received: {received}");
                println!("  Errors: {encoded_to_received_err_str}",);
            }

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
}
