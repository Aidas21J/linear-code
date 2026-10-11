use super::{
    algebra::Field,
    core::{LinearCode, MemorylessChannel},
};

/// Represents basic traits any kind of app has to have.
pub trait BaseApp {
    type Code: LinearCode;
    type Channel: MemorylessChannel<<<Self::Code as LinearCode>::Field as Field>::Element>;

    /// Return a reference to the linear code.
    fn code(&self) -> &Self::Code;
    /// Return a reference to the channel.
    fn channel(&self) -> &Self::Channel;

    /// Edit the channel
    fn set_channel(&mut self, channel: Self::Channel);
}

/// Macro to log an expression's execution duration.
///
/// Takes an expressions `$name` and `$activity`.
///
/// Returns `$activity` output and prints `$name` with progress and duration info.
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
