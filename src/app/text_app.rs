use std::path::Path;

use super::{BufferApp, algebra::Field, core::LinearCode, log_activity, ui};

/// App that works with text input and output.
pub trait TextApp: BufferApp
where
    <Self::Code as LinearCode>::RowVector: Sync + Send,
    <Self::Code as LinearCode>::RowVector: Clone + std::str::FromStr + std::fmt::Display,
    Self::Channel: std::str::FromStr<Err: std::fmt::Display>,
    <<Self::Code as LinearCode>::Field as Field>::Element: std::fmt::Display,
{
    /// App's main loop. It consists of:
    ///     1) Getting text from user either from file or console (console accepts only a single
    ///        line).
    ///     2) Sending input through channel without coding it and outputting the result. If the
    ///        input was taken from a file, then it is saved to a file. If the input was taken from
    ///        console then the output is printed to it as well.
    ///     3) Doing exactly the same as (2), but sending input through channel with coding and (if
    ///        input was from a file) saving output to a different file.
    ///     4) Printing error counts of not coded and coded versions.
    ///     5) Asking user whether they want another iteration.
    ///     6) Letting user to edit the channel before next iteration.
    fn text_loop(&mut self) {
        loop {
            eprintln!();
            eprintln!("{}", "#".repeat(100));
            eprintln!("{}", "#".repeat(100));
            eprintln!("{}", "#".repeat(100));
            eprintln!();

            let (text, maybe_filename_without_extension) = get_text_input();

            eprintln!();
            eprintln!("{}", "-".repeat(100));
            eprintln!("[WITHOUT CODING] {}", "-".repeat(100 - 17));
            eprintln!("{}", "-".repeat(100));
            eprintln!();

            let errors_without_coding = {
                let code = false;

                let (output_bytes, error_count) = self.send_bytes(text.clone().into(), code);

                match maybe_filename_without_extension.clone() {
                    Some(filename_without_extension) => {
                        output_text_to_file(filename_without_extension, output_bytes, code);
                    }
                    None => {
                        eprintln!("Original: {}", &text);
                        eprintln!("Received: {}", String::from_utf8_lossy(&output_bytes));
                    }
                };

                error_count
            };

            eprintln!();
            eprintln!("{}", "-".repeat(100));
            eprintln!("[WITH CODING] {}", "-".repeat(100 - 14));
            eprintln!("{}", "-".repeat(100));
            eprintln!();

            let errors_with_coding = {
                let code = true;

                let (output_bytes, error_count) = self.send_bytes(text.clone().into(), code);

                match maybe_filename_without_extension {
                    Some(filename_without_extension) => {
                        output_text_to_file(filename_without_extension, output_bytes, code);
                    }
                    None => {
                        eprintln!("Original: {}", &text);
                        eprintln!("Received: {}", String::from_utf8_lossy(&output_bytes));
                    }
                };

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

            if !ui::get_yes_no("Try another text?", Some(true)) {
                return;
            }

            if ui::get_yes_no("Edit channel?", Some(false)) {
                self.set_channel(ui::get_from_line("p_e"));
            }
        }
    }
}

/// Gets text input from user.
///
/// The user is asked whether to read the input from a file or from console (console only allows one
/// line inputs). If the user wants input from console, they are asked to enter the text into the
/// console. If the user wants input from a file, they are asked to enter the path to the file. That
/// file is read and the contents are stored.
///
/// Returns tuple where first member is the text (from a file or not) and the second is the filename
/// to the file or `None` if input came from the console.
fn get_text_input() -> (String, Option<String>) {
    loop {
        if !ui::get_yes_no("Read from file?", None) {
            return (ui::get_from_line("Enter text"), None);
        }

        eprintln!();

        let filename: String = ui::get_from_line("Enter filename");
        let path = Path::new(&filename);

        eprintln!();

        let read_result = log_activity!(
            format!("Reading from {filename} file..."),
            std::fs::read_to_string(path)
        );

        match read_result {
            Ok(text) => {
                let maybe_filename_without_extension = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .map(|s| s.to_string());

                return (text, maybe_filename_without_extension);
            }
            Err(e) => eprintln!("Failed to open file: {e}. Try again."),
        }
    }
}

/// Outputs the text input to appropriate text file.
///
/// Takes original filename without extension `filename_without_extension`, raw output bytes
/// `output_text` and `bool` representing whether coding was used for sending through the channel
/// `code`.
///
/// Output filename is kept the same but with added extension to signify whether it's the coded or
/// not coded output. For example, if original file was original.txt, then the coded output will be
/// saved to original-with-coding.txt and the not coded to original-without-coding.txt.
fn output_text_to_file(filename_without_extension: String, output_text: Vec<u8>, code: bool) {
    let filepath = if code {
        format!("{filename_without_extension}-with-coding.txt")
    } else {
        format!("{filename_without_extension}-without-coding.txt")
    };

    let path = Path::new(&filepath);

    let write_result = log_activity!("Writting to file...", std::fs::write(path, output_text));

    match write_result {
        Ok(_) => eprintln!("Saved to {filepath}."),
        Err(e) => eprintln!("Failed to write to {filepath}: {e}."),
    };
}
