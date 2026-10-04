use crate::app::{
    algebra::{dynamic_vector::DynamicVector as _, field::Field},
    base_app::BaseApp,
    core::{linear_code::LinearCode, memoryless_channel::MemorylessChannel as _},
    ui,
};

pub trait SingleVectorApp: BaseApp
where
    <Self::Code as LinearCode>::RowVector: Clone + std::str::FromStr + std::fmt::Display,
    <<Self::Code as LinearCode>::Field as Field>::Element: std::fmt::Display,
{
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

            let transition_errors = encoded.hamming_distance_to(&received);
            let decoded_vector_errors = initial.hamming_distance_to(&decoded);

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
