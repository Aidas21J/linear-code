use std::fmt::Display;

use crate::algebra::{
    countable_underlying_set::CountableUnderlyingSet, f2::F2, field::Field, matrix::DMat,
};

pub mod algebra;

fn main() {
    println!("F2 FIELD:");
    println!();
    print_field::<F2>();

    println!();
    println!();

    println!("MATRICES OVER F2 FIELD:");
    println!();
    print_matrices::<F2>();
}

fn print_field<F: Field>()
where
    F::Element: CountableUnderlyingSet + Display,
{
    println!("Addition:");
    for (a, b) in F::Element::iter_pairs() {
        let res = F::add(&a, &b);
        println!("  {a} + {b} = {res}");
    }

    println!();
    println!("Multiplication:");
    for (a, b) in F::Element::iter_pairs() {
        let res = F::mul(&a, &b);
        println!("  {a} * {b} = {res}");
    }

    println!();
    println!("Negation:");
    for a in F::Element::iter() {
        let res = F::neg(&a);
        println!("  -{a} = {res}");
    }

    println!();
    println!("Reciprocal:");
    for a in F::Element::iter() {
        let res = F::recip(&a).map_or("undefined".to_string(), |r| format!("{r}"));
        println!("  {a}^(-1) = {res}");
    }
}

fn print_matrices<F: Field>()
where
    F::Element: Display,
{
    // Generator matrix G
    let generator_matrix = DMat::<F>::from_rows(vec![
        vec![F::ONE, F::ONE, F::ZERO, F::ONE, F::ZERO, F::ZERO],
        vec![F::ZERO, F::ONE, F::ONE, F::ZERO, F::ONE, F::ZERO],
        vec![F::ONE, F::ZERO, F::ONE, F::ZERO, F::ZERO, F::ONE],
    ])
    .unwrap();

    println!("G:");
    println!("{generator_matrix}");
    println!();

    // Parity-check matrix H
    let n = generator_matrix.cols();
    let k = generator_matrix.rows();

    let (a, _) = generator_matrix.split(n - k);
    let minus_at = DMat::scalar_mul(&F::neg(&F::ONE), &a.transpose());
    let parity_check_matrix = DMat::join(&DMat::identity(n - k), &minus_at);

    println!("H:");
    println!("{parity_check_matrix}");
    println!();

    // G + H
    let g_h = DMat::add(&generator_matrix, &parity_check_matrix);

    println!("G + H:");
    println!("{g_h}");
    println!();

    // G * H^T == 0
    let g_ht = DMat::mul(&generator_matrix, &parity_check_matrix.transpose());

    assert_eq!(g_ht, DMat::zeros(k, k));

    println!("G * H^T:");
    println!("{g_ht}");
    println!();
}
