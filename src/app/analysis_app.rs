use super::{
    BufferApp,
    algebra::{DynamicVector as _, Field, Randomizable, error_count},
    core::LinearCode,
    ui,
};

/// App for analysing its code properties.
pub trait AnalysisApp: BufferApp + Sync
where
    <Self::Code as LinearCode>::RowVector: Sync + Send,
    Self::Channel: std::str::FromStr<Err: std::fmt::Display>,
    <<Self::Code as LinearCode>::Field as Field>::Element: Randomizable,
{
    /// App's main loop. It consists of:
    ///     1) Asking the number of vectors to use `vec_count`.
    ///     2) Generating `vec_count` vectors and sending them through app's channel with app's
    ///        coding and calculating the number of errors.
    ///     3) Printing the count and the proportion of errors.
    ///     4) Asking user whether they want another iteration.
    ///     5) Letting user to edit the channel before next iteration.
    fn analysis_loop(&mut self) {
        loop {
            eprintln!();
            eprintln!("{}", "#".repeat(100));
            eprintln!("{}", "#".repeat(100));
            eprintln!("{}", "#".repeat(100));
            eprintln!();

            let vec_count: usize = ui::get_from_line("Enter number of vectors");
            let vec_dim = self.code().k();
            let element_count: usize = vec_count * vec_dim;

            let error_count = {
                let input_vecs = {
                    let mut rng = rand::rng();
                    let mut input_vecs = Vec::with_capacity(vec_count);

                    for _ in 0..vec_count {
                        input_vecs.push(<Self::Code as LinearCode>::RowVector::generate_uniform(
                            vec_dim, &mut rng,
                        ));
                    }

                    input_vecs
                };

                let output_vecs = self.send_vecs_with_coding(&input_vecs);

                error_count(input_vecs.iter(), output_vecs.iter())
            };
            let error_proportion: f64 = error_count as f64 / element_count as f64;

            eprintln!();
            eprintln!("{}", "-".repeat(100));
            eprintln!("[RESULTS] {}", "-".repeat(100 - 10));
            eprintln!("{}", "-".repeat(100));
            eprintln!();

            eprint!("     Error count: ");
            println!("{error_count}");
            eprint!("Error proportion: ");
            println!("{error_proportion}");

            eprintln!();
            eprintln!("{}", "-".repeat(100));
            eprintln!();

            if !ui::get_yes_no("Try another sample?", Some(true)) {
                return;
            }

            if ui::get_yes_no("Edit channel?", Some(false)) {
                self.set_channel(ui::get_from_line("p_e"));
            }
        }
    }
}
