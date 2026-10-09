use crate::app::{
    algebra::{
        dynamic_vector::{DynamicVector as _, error_count},
        field::Field,
        randomizable::Randomizable,
    },
    buffer_app::BufferApp,
    core::linear_code::LinearCode,
    ui,
};

pub trait AnalysisApp: BufferApp + Sync
where
    <Self::Code as LinearCode>::RowVector: Sync + Send,
    Self::Channel: std::str::FromStr<Err: std::fmt::Display>,
    <<Self::Code as LinearCode>::Field as Field>::Element: Randomizable,
{
    fn analysis_loop(&mut self) {
        loop {
            eprintln!();
            eprintln!("{}", "#".repeat(100));
            eprintln!("{}", "#".repeat(100));
            eprintln!("{}", "#".repeat(100));
            eprintln!();

            let byte_count: usize = ui::get_from_line("Enter number of bytes");
            let vec_dim = self.code().k();
            let bit_count: usize = byte_count * vec_dim;

            let error_count = {
                let input_vecs = {
                    let mut rng = rand::rng();
                    let mut input_vecs = Vec::with_capacity(byte_count);

                    for _ in 0..byte_count {
                        input_vecs.push(<Self::Code as LinearCode>::RowVector::generate_uniform(
                            vec_dim, &mut rng,
                        ));
                    }

                    input_vecs
                };

                let output_vecs = self.send_vecs_with_coding(&input_vecs);

                error_count(input_vecs.iter(), output_vecs.iter())
            };
            let error_proportion: f64 = error_count as f64 / bit_count as f64;

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
