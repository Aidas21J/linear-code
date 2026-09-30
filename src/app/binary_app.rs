use crate::app::{
    algebra::{dmat::DMat, f2::F2},
    app::App,
    core::{
        binary_linear_code::BinaryLinearCode, linear_code::LinearCode,
        symmetric_channel::SymmetricChannel, universal_vec_codec::UniversalVecCodec,
    },
    ui,
};

pub struct BinaryApp {
    code: BinaryLinearCode,
    channel: SymmetricChannel,
    vec_codec: UniversalVecCodec<F2>,
}

impl BinaryApp {
    pub fn new() -> Self {
        let p_e: f64 = loop {
            let p_e_input = ui::get_from_line("p_e");
            if 0.0 <= p_e_input && p_e_input <= 1.0 {
                break p_e_input;
            }
            eprintln!("Invalid input. p_e has to be between 0 and 1 (inclusive). Try again.");
        };

        let n: usize = ui::get_from_line("n");

        let k: usize = loop {
            let k_input = ui::get_from_line("k");
            if k_input > n {
                eprintln!("Invalid input. k cannot be greater than n. Try again.");
            } else if k_input <= 0 {
                eprintln!("Invalid input. k must be positive. Try again.");
            } else {
                break k_input;
            }
        };
        eprintln!();

        let input_generator_parity =
            ui::get_yes_no("Do you want to enter custom generator parity?", Some(false));

        let code = {
            let rows = k;
            let cols = n.saturating_sub(k);

            let generator_parity = if input_generator_parity {
                ui::get_matrix(
                    "Entering G parity part (identity part will be added automatically)",
                    rows,
                    cols,
                )
            } else {
                DMat::generate_uniform(rows, cols, &mut rand::rng())
            };

            BinaryLinearCode::from_parity(generator_parity)
        };
        eprintln!();

        println!("G:");
        println!("{}", code.generator());
        println!();
        println!("H:");
        println!("{}", code.parity_check());
        println!();

        let channel = SymmetricChannel::new(p_e);
        let vec_codec = UniversalVecCodec::new(code.k());

        Self {
            code,
            channel,
            vec_codec,
        }
    }
}

impl App for BinaryApp {
    type Code = BinaryLinearCode;
    type Channel = SymmetricChannel;
    type VecCodec = UniversalVecCodec<F2>;

    fn code(&self) -> &BinaryLinearCode {
        &self.code
    }

    fn channel(&self) -> &SymmetricChannel {
        &self.channel
    }

    fn vec_codec(&self) -> &Self::VecCodec {
        &self.vec_codec
    }
}
