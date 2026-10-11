use super::{
    AnalysisApp, BaseApp, BufferApp, ImageApp, SingleVectorApp, TextApp,
    algebra::{DMat, DRVec, DynamicMatrix as _, F2},
    core::{BinaryLinearCode, LinearCode as _, SymmetricChannel, UniversalVecCodec},
    ui,
};

/// Main app object.
pub struct App {
    /// Linear code used for coding.
    code: BinaryLinearCode,
    /// Channel that the data is sent through.
    channel: SymmetricChannel,
    /// Vector codec for parsing to and from raw bytes.
    vec_codec: UniversalVecCodec<DRVec<F2>>,
}

impl App {
    /// Create new app.
    ///
    /// First, the user to input code length `n` and dimension `k`.
    ///
    /// Second, the user is asked whether they want to put in a custom generator matrix or to have a
    /// random generator matrix. This is how the parity part (generator matrix except identity
    /// columns) is created:
    ///     - If the user wants a random matrix, random parity part is generated for them.
    ///     - If the user wants a custom matrix, they are asked to put in every element of the
    ///     parity part one by one.
    /// The final parity part is joined to the identity part and the generator matrix (and later the
    /// whole linear code) is constructed.
    ///
    /// Lastly, the user is asked for the error probability and the app's symmetric channel is
    /// constructed from that.
    ///
    /// Note: vector codec is always the same and is constructed automatically under the hood every
    /// time.
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

    /// Return a reference to app's code.
    fn code(&self) -> &BinaryLinearCode {
        &self.code
    }

    /// Return a reference to app's channel.
    fn channel(&self) -> &SymmetricChannel {
        &self.channel
    }

    /// Sets app's channel.
    ///
    /// Takes new `channel` and replaces the old one with it.
    fn set_channel(&mut self, channel: Self::Channel) {
        self.channel = channel;
    }
}

impl BufferApp for App {
    type VecCodec = UniversalVecCodec<DRVec<F2>>;

    /// Return a reference to app's vector codec.
    fn vec_codec(&self) -> &Self::VecCodec {
        &self.vec_codec
    }
}

impl SingleVectorApp for App {}
impl TextApp for App {}
impl ImageApp for App {}
impl AnalysisApp for App {}
