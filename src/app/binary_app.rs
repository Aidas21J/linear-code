use crate::app::{
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
        let p_e: f64 = ui::get_from_line("p_e");
        let n: usize = ui::get_from_line("n");
        let k: usize = loop {
            let k_input: usize = ui::get_from_line("k");
            if k_input <= n {
                break k_input;
            }

            println!("Invalid input. k cannot be greater than n. Try again.");
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
}
