use std::path::Path;

use crate::app::{
    algebra::field::Field, base_app::log_activity, buffer_app::BufferApp,
    core::linear_code::LinearCode, ui,
};

pub trait TextApp: BufferApp
where
    <Self::Code as LinearCode>::RowVector: Sync + Send,
    <Self::Code as LinearCode>::RowVector: Clone + std::str::FromStr + std::fmt::Display,
    <<Self::Code as LinearCode>::Field as Field>::Element: std::fmt::Display,
{
    fn text_loop(&self) {
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
        }
    }
}

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
