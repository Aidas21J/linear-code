use super::{
    AnalysisApp, BaseApp, BufferApp, ImageApp, SingleVectorApp, TextApp,
    algebra::{DMat, DRVec, DynamicMatrix as _, F2},
    core::{BinaryLinearCode, LinearCode as _, SymmetricChannel, UniversalVecCodec},
    ui,
};

pub struct App {
    code: BinaryLinearCode,
    channel: SymmetricChannel,
    vec_codec: UniversalVecCodec<DRVec<F2>>,
}

impl App {
    pub fn new() -> Self {
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

        eprintln!("G:");
        eprintln!("{}", code.generator());
        eprintln!();
        eprintln!("H:");
        eprintln!("{}", code.parity_check());
        eprintln!();

        let channel = ui::get_from_line("p_e");
        let vec_codec = UniversalVecCodec::new(code.k());

        Self {
            code,
            channel,
            vec_codec,
        }
    }
}

impl BaseApp for App {
    type Code = BinaryLinearCode;
    type Channel = SymmetricChannel;

    fn code(&self) -> &BinaryLinearCode {
        &self.code
    }

    fn channel(&self) -> &SymmetricChannel {
        &self.channel
    }

    fn set_channel(&mut self, channel: Self::Channel) {
        self.channel = channel;
    }
}

impl BufferApp for App {
    type VecCodec = UniversalVecCodec<DRVec<F2>>;

    fn vec_codec(&self) -> &Self::VecCodec {
        &self.vec_codec
    }
}

impl SingleVectorApp for App {}
impl TextApp for App {}
impl ImageApp for App {}
impl AnalysisApp for App {}
