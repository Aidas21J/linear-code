use crate::app::{
    algebra::{
        drvec::DRVec,
        f2::{F2, F2Element},
        field::Field as _,
    },
    app::App,
    core::{
        binary_channel::BinaryChannel, binary_linear_code::BinaryLinearCode,
        linear_code::LinearCode,
    },
    ui,
};

pub struct BinaryApp {
    code: BinaryLinearCode,
    channel: BinaryChannel,
}

impl BinaryApp {
    pub fn new() -> Option<Self> {
        let p_e: f64 = loop {
            let p_e_input = ui::get_from_line("p_e");
            if 0.0 <= p_e_input && p_e_input <= 1.0 {
                break p_e_input;
            }
            println!("Invalid input. p_e has to be between 0 and 1 (inclusive). Try again.");
        };

        let n: usize = ui::get_from_line("n");

        let k: usize = loop {
            let k_input = ui::get_from_line("k");
            if k_input > n {
                println!("Invalid input. k cannot be greater than n. Try again.");
            } else if k_input <= 0 {
                println!("Invalid input. k must be positive. Try again.");
            } else {
                break k_input;
            }
        };

        println!();
        let input_generator_parity =
            ui::get_yes_no("Do you want to enter custom generator parity?", Some(false));

        let code = if input_generator_parity {
            let generator_parity = ui::get_matrix(
                "Entering G parity part (identity part will be added automatically)",
                k,
                n - k,
            );
            BinaryLinearCode::from_parity(generator_parity)?
        } else {
            let mut rng = rand::rng();
            BinaryLinearCode::new_random(n, k, &mut rng)?
        };

        println!();
        println!("G:");
        println!("{}", code.generator());
        println!();
        println!("H:");
        println!("{}", code.parity_check());
        println!();

        let channel = BinaryChannel::new(p_e);

        Some(Self { code, channel })
    }
}

impl App<BinaryLinearCode, BinaryChannel> for BinaryApp {
    fn code(&self) -> &BinaryLinearCode {
        &self.code
    }

    fn channel(&self) -> &BinaryChannel {
        &self.channel
    }

    fn bytes_to_vecs(&self, bytes: Vec<u8>) -> (Vec<DRVec<F2>>, usize) {
        const BITS_PER_BYTE: usize = 8;
        let chunk_size = self.code().k();
        let pad_size = {
            let a = bytes.len() % chunk_size;
            let b = BITS_PER_BYTE % chunk_size;
            let remainder = (a * b) % chunk_size;

            (chunk_size - remainder) % chunk_size
        };

        let mut vecs = Vec::with_capacity(bytes.len() + pad_size);

        let mut bits_iter = bytes
            .into_iter()
            .flat_map(|byte| {
                [
                    (0b1000_0000 & byte) != 0,
                    (0b0100_0000 & byte) != 0,
                    (0b0010_0000 & byte) != 0,
                    (0b0001_0000 & byte) != 0,
                    (0b0000_1000 & byte) != 0,
                    (0b0000_0100 & byte) != 0,
                    (0b0000_0010 & byte) != 0,
                    (0b0000_0001 & byte) != 0,
                ]
            })
            .map(F2Element::from)
            .chain(std::iter::repeat_n(F2::ZERO, pad_size));

        loop {
            let mut chunk = bits_iter.by_ref().take(chunk_size).peekable();

            if chunk.peek().is_none() {
                break;
            }

            vecs.push(chunk.collect::<Vec<_>>().into());
        }

        (vecs, pad_size)
    }

    fn vecs_to_bytes(&self, vecs: Vec<DRVec<F2>>, pad_size: usize) -> Vec<u8> {
        const BITS_PER_BYTE: usize = 8;
        let bits_to_take = (vecs.len() * self.code().k()).saturating_sub(pad_size);
        let mut bytes = Vec::with_capacity(bits_to_take / BITS_PER_BYTE);

        let mut bit_iter = vecs
            .into_iter()
            .flat_map(|v| v.into_data())
            .take(bits_to_take);

        loop {
            let chunk = bit_iter.by_ref().take(BITS_PER_BYTE);

            let (byte, bit_count) = chunk.fold((0u8, 0), |(byte, bit_count), bit| {
                ((byte << 1u8) | u8::from(bit), bit_count + 1)
            });

            if bit_count != BITS_PER_BYTE {
                break;
            }

            bytes.push(byte);
        }

        bytes
    }
}
