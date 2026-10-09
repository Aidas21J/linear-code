use std::io::{Write as _, stderr, stdin, stdout};

use super::algebra::{DynamicMatrix, DynamicVector, Field};

pub fn get_yes_no(message: &str, default: Option<bool>) -> bool {
    loop {
        let _ = stdout().flush();
        eprint!(
            "{message} {default_option}: ",
            default_option = match default {
                Some(false) => "(y/N)",
                Some(true) => "(Y/n)",
                None => "(y/n)",
            }
        );
        let _ = stderr().flush();

        let mut input_str = String::new();
        if let Err(e) = stdin().read_line(&mut input_str) {
            eprintln!("Failed to read input: {e}. Try again.");
            continue;
        }

        match input_str.trim() {
            "" => match default {
                Some(d) => return d,
                None => continue,
            },
            "n" | "no" => return false,
            "y" | "yes" => return true,
            _ => eprintln!("Invalid input. Try again."),
        }
    }
}

pub fn get_from_line<T>(message: &str) -> T
where
    T: std::str::FromStr<Err: std::fmt::Display>,
{
    loop {
        eprint!("{message}: ");
        let _ = stderr().flush();

        let mut input_str = String::new();
        if let Err(e) = stdin().read_line(&mut input_str) {
            eprintln!("Failed to read input: {e}. Try again.");
            continue;
        }
        input_str = input_str.replace(",", ".");

        match input_str.trim().parse::<T>() {
            Ok(value) => return value,
            Err(e) => eprintln!("Failed to parse input: {e}. Try again."),
        }
    }
}

pub fn get_row_vector<F, V>(message: &str, expected_cols: usize) -> V
where
    F: Field,
    V: DynamicVector<Field = F> + std::str::FromStr<Err: std::fmt::Display>,
{
    loop {
        let v: V = get_from_line(message);

        if v.dim() == expected_cols {
            break v;
        }

        eprintln!("Vector must have exactly {expected_cols} elements. Try again.");
    }
}

pub fn vector_error_string<F, V>(a: &V, b: &V, match_marker: &str, error_marker: &str) -> String
where
    F: Field,
    V: DynamicVector<Field = F>,
{
    debug_assert_eq!(a.dim(), b.dim(), "expected same size vectors");

    a.iter()
        .zip(b.iter())
        .map(|(a_i, b_i)| match a_i != b_i {
            false => match_marker,
            true => error_marker,
        })
        .collect()
}

pub fn get_matrix<F, M>(message: &str, rows: usize, cols: usize) -> M
where
    F: Field,
    M: DynamicMatrix<Field = F>,
    F::Element: std::fmt::Display + std::str::FromStr<Err: std::fmt::Display>,
{
    let mut data: Vec<F::Element> = Vec::with_capacity(rows.saturating_mul(cols));

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
                .map(|row_str| format!("|{row_str}|"))
                .collect::<Vec<_>>()
                .join("\n");

            eprintln!();
            eprintln!("{message}:");
            eprintln!("{data_str}");

            let x: F::Element = get_from_line("Enter x value");
            data.push(x);
        }
    }

    M::from_data(rows, data).unwrap()
}
