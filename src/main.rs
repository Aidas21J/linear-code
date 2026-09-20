use crate::{
    algebra::{dmat::DMat, drvec::DRVec, f2::F2, field::Field},
    core::{binary_linear_code::BinaryLinearCode, linear_code::LinearCode as _},
};

pub mod algebra;
pub mod core;

fn main() {
    test_encoding();
    println!();
    test_decoding();
    println!();
    test_random();
}

fn test_encoding() {
    let parity = DMat::<F2>::from_rows(vec![
        vec![F2::ONE, F2::ONE, F2::ZERO],
        vec![F2::ZERO, F2::ONE, F2::ONE],
        vec![F2::ONE, F2::ZERO, F2::ONE],
    ])
    .unwrap();

    let code = BinaryLinearCode::from_parity(parity).unwrap();

    let codeword = DRVec::<F2>::from_row(vec![F2::ONE, F2::ZERO, F2::ONE]);
    let encoded = code.encode(&codeword);
    let expected =
        DRVec::<F2>::from_row(vec![F2::ZERO, F2::ONE, F2::ONE, F2::ONE, F2::ZERO, F2::ONE]);

    println!("                 Codeword: {codeword}");
    println!("         Encoded codeword: {encoded}");
    println!("Expected encoded codeword: {expected}");

    assert_eq!(
        encoded, expected,
        "encoded vector doesn't match expectation"
    );
}

fn test_decoding() {
    let parity = DMat::<F2>::from_rows(vec![
        vec![F2::ONE, F2::ONE, F2::ZERO],
        vec![F2::ZERO, F2::ONE, F2::ONE],
        vec![F2::ONE, F2::ZERO, F2::ONE],
    ])
    .unwrap();

    let code = BinaryLinearCode::from_parity(parity).unwrap();

    let codeword = DRVec::<F2>::from_row(vec![
        F2::ONE,
        F2::ONE,
        F2::ONE,
        F2::ZERO,
        F2::ZERO,
        F2::ZERO,
    ]);
    let decoded = code.decode(&codeword).unwrap();
    let expected = DRVec::<F2>::from_row(vec![F2::ZERO, F2::ONE, F2::ZERO]);

    println!("                 Codeword: {codeword}");
    println!("         Decoded codeword: {decoded}");
    println!("Expected decoded codeword: {decoded}");

    assert_eq!(
        decoded, expected,
        "decoded vector doesn't match expectation"
    );
}

fn test_random() {
    let mut rng = rand::rng();

    let (n, k) = (10, 7);
    let code = BinaryLinearCode::new_random(n, k, &mut rng).unwrap();

    let codeword = DRVec::<F2>::from_row(vec![
        F2::ONE,
        F2::ZERO,
        F2::ONE,
        F2::ONE,
        F2::ZERO,
        F2::ONE,
        F2::ONE,
    ]);
    let encoded = code.encode(&codeword);
    let decoded = code.decode(&encoded).unwrap();

    println!("                 Codeword: {codeword}");
    println!("         Encoded codeword: {encoded}");
    println!("         Decoded codeword: {decoded}");

    assert_eq!(
        codeword, decoded,
        "decoded vector doesn't match expectation"
    );
}
