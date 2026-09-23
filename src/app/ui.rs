use std::{
    io::{self, Write as _},
    str::FromStr,
};

use crate::app::algebra::{dmat::DMat, drvec::DRVec, field::Field};

pub fn get_yes_no(message: &str, default: Option<bool>) -> bool {
    loop {
        print!(
            "{message} {default_option}: ",
            default_option = match default {
                Some(false) => "(y/N)",
                Some(true) => "(Y/n)",
                None => "(y/n)",
            }
        );
        io::stdout().flush().unwrap();

        let mut input_str = String::new();
        if io::stdin().read_line(&mut input_str).is_err() {
            println!("Failed to read input. Try again.");
            continue;
        }

        match input_str.trim() {
            "" => match default {
                Some(d) => return d,
                None => continue,
            },
            "n" | "no" => return false,
            "y" | "yes" => return true,
            _ => println!("Invalid input. Try again."),
        }
    }
}

pub fn get_from_line<T: FromStr>(message: &str) -> T {
    loop {
        print!("{message}: ");
        io::stdout().flush().unwrap();

        let mut input_str = String::new();
        if io::stdin().read_line(&mut input_str).is_err() {
            println!("Failed to read input. Try again.");
            continue;
        }
        input_str = input_str.replace(",", ".");

        match input_str.trim().parse::<T>() {
            Ok(value) => return value,
            Err(_) => println!("Invalid input. Try again."),
        }
    }
}

pub fn get_row_vector<F: Field>(message: &str, expected_cols: usize) -> DRVec<F>
where
    F::Element: std::fmt::Display,
{
    loop {
        let v: DRVec<_> = get_from_line(message);

        if v.cols() == expected_cols {
            break v;
        }

        println!("Vector must have exactly {expected_cols} elements. Try again.");
    }
}

pub fn vector_error_string<F: Field>(
    a: &DRVec<F>,
    b: &DRVec<F>,
    no_error_char: char,
    error_char: char,
) -> String {
    debug_assert_eq!(a.cols(), b.cols(), "expected same size vectors");

    a.iter()
        .zip(b.iter())
        .map(|(a_i, b_i)| match a_i != b_i {
            false => no_error_char,
            true => error_char,
        })
        .collect()
}

pub fn get_matrix<F: Field>(message: &str, rows: usize, cols: usize) -> DMat<F>
where
    F::Element: std::fmt::Display,
{
    let mut data: Vec<F::Element> = Vec::new();
    data.reserve_exact(rows * cols);

    for _ in 0..rows {
        for _ in 0..cols {
            let data_str = data
                .iter()
                .map(|x| x.to_string())
                .chain(std::iter::once("x".to_string()))
                .chain(std::iter::repeat(".".to_string()))
                .take(rows * cols)
                .collect::<Vec<String>>()
                .chunks(cols)
                .map(|row| row.join(" "))
                .map(|row_str| format!("|{row_str}|").to_string())
                .collect::<Vec<_>>()
                .join("\n");

            println!();
            println!("{message}:");
            println!("{data_str}");

            let x = get_from_line::<F::Element>("Enter x value");
            data.push(x);
        }
    }

    DMat::from_data(rows, data).unwrap()
}
