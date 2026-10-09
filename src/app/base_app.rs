use super::{
    algebra::Field,
    core::{LinearCode, MemorylessChannel},
};

pub trait BaseApp {
    type Code: LinearCode;
    type Channel: MemorylessChannel<<<Self::Code as LinearCode>::Field as Field>::Element>;

    fn code(&self) -> &Self::Code;
    fn channel(&self) -> &Self::Channel;

    fn set_channel(&mut self, channel: Self::Channel);
}

#[macro_export]
macro_rules! log_activity {
    ($name:expr, $activity:expr) => {{
        eprintln!("{}", $name);

        let start = std::time::Instant::now();
        let result = $activity;
        let duration = start.elapsed();

        eprintln!("Done.");
        eprintln!("Duration: {} us", duration.as_micros());
        eprintln!();

        result
    }};
}
pub use log_activity;
