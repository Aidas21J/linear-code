use crate::app::{
    BaseApp,
    algebra::{DynamicVector as _, Field},
    core::{LinearCode, MemorylessChannel as _},
    ui,
};

/// App for working with single vectors.
pub trait SingleVectorApp: BaseApp
where
    <Self::Code as LinearCode>::RowVector:
        Clone + std::fmt::Display + std::str::FromStr<Err: std::fmt::Display>,
    Self::Channel: std::str::FromStr<Err: std::fmt::Display>,
    <<Self::Code as LinearCode>::Field as Field>::Element: std::fmt::Display,
{
    /// App's main loop. It consists of:
    ///     1) Getting vector from the user.
    ///     2) Encoding and sending it through channel.
    ///     3) Showing what was received and (if the user wants) letting to edit it.
    ///     4) Decoding and showing final output with some stats.
    ///     5) Asking user whether they want another iteration.
    ///     6) Letting user to edit the channel before next iteration.
    fn single_vector_loop(&mut self) {
        loop {
            eprintln!();
            eprintln!("{}", "#".repeat(100));
            eprintln!("{}", "#".repeat(100));
            eprintln!("{}", "#".repeat(100));
            eprintln!();

            let initial = ui::get_row_vector("Enter vector", self.code().k());

            print_separator();
            eprintln!("Encoding...");

            let encoded = self.code().encode(&initial);

            println!("Encoded: {encoded}");

            print_separator();
            eprintln!("Sending through the channel...");

            let received = {
                let mut received = self.channel().send_vec(encoded.clone(), &mut rand::rng());
                let mut err_str = ui::vector_error_string(&encoded, &received, " ", "^");

                println!();
                println!(" Encoded: {encoded}");
                println!("Received: {received}");
                println!("  Errors: {err_str}");
                println!();

                let edit_received = ui::get_yes_no("Edit received vector?", Some(false));
                if edit_received {
                    received = ui::get_row_vector("Enter received", self.code().n());
                    err_str = ui::vector_error_string(&encoded, &received, " ", "^");

                    print_separator();
                    println!(" Encoded: {encoded}");
                    println!("Received: {received}");
                    println!("  Errors: {err_str}");
                }

                received
            };

            print_separator();
            eprintln!("Decoding...");

            let decoded = self.code().decode(received.clone());

            println!("Decoded: {decoded}");

            print_separator();

            let initial_to_final_err_str = ui::vector_error_string(&initial, &decoded, " ", "^");

            println!("Initial vector: {initial}");
            println!("  Final vector: {decoded}");
            println!("        Errors: {initial_to_final_err_str}");

            let transition_errors = encoded.hamming_distance_to(&received);
            let decoded_vector_errors = initial.hamming_distance_to(&decoded);

            println!();
            println!("       Errors from channel: {transition_errors}");
            println!("Errors in the final vector: {decoded_vector_errors}");

            print_separator();

            if !ui::get_yes_no("Try another vector?", Some(true)) {
                return;
            }

            if ui::get_yes_no("Edit channel?", Some(false)) {
                self.set_channel(ui::get_from_line("p_e"));
            }
        }
    }
}

fn print_separator() {
    eprintln!();
    eprintln!("{}", "-".repeat(100));
    eprintln!();
}
