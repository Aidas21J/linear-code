use crate::{
    algebra::{
        dmat::DMat,
        drvec::DRVec,
        f2::{F2, F2Element},
        field::Field,
        randomizable::Randomizable,
    },
    core::{
        binary_channel::BinaryChannel, binary_linear_code::BinaryLinearCode, channel::Channel,
        linear_code::LinearCode as _,
    },
};

pub mod algebra;
pub mod core;

fn main() {
    test_encoding();
    println!();
    test_decoding();
    println!();
    test_random();
    println!();
    test_buffer();
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
    let encoded = code.encode(codeword.clone());
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
    let decoded = code.decode(codeword.clone()).unwrap();
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
    let encoded = code.encode(codeword.clone());
    let decoded = code.decode(encoded.clone()).unwrap();

    println!("                 Codeword: {codeword}");
    println!("         Encoded codeword: {encoded}");
    println!("         Decoded codeword: {decoded}");

    assert_eq!(
        codeword, decoded,
        "decoded vector doesn't match expectation"
    );
}

fn test_buffer() {
    let mut rng = rand::rng();

    let l = 5;
    let (n, k) = (15, 11);

    let code = BinaryLinearCode::new_random(n, k, &mut rng).unwrap();
    let data = (0..(l * k))
        .map(|_| F2Element::generate_uniform(&mut rng))
        .collect::<Vec<_>>();
    let channel = BinaryChannel::new(0.1);

    let (encoded, pad_size) = code.encode_buffer(&data);
    let received = channel.send_buffer(&encoded, &mut rng);
    let decoded = code.decode_buffer(&received, pad_size);

    let number_of_flips = encoded
        .iter()
        .zip(received.iter())
        .map(|(a, b)| (a != b) as usize)
        .sum::<usize>();
    let number_of_mistakes: usize = data
        .iter()
        .zip(decoded.iter())
        .map(|(a, b)| (a != b) as usize)
        .sum::<usize>();

    let data_to_string = |data: Vec<F2Element>| {
        data.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join("")
    };

    println!("         Codeword: {:?}", data_to_string(data));
    println!(" Encoded codeword: {:?}", data_to_string(encoded));
    println!("Received codeword: {:?}", data_to_string(received));
    println!(" Decoded codeword: {:?}", data_to_string(decoded));

    println!();
    println!("Bit flips: {number_of_flips}.");
    println!("Final mistakes: {number_of_mistakes}.");
}
