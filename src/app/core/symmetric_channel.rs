use crate::app::algebra::F2Element;

use super::MemorylessChannel;

/// Symmetric channel representation.
pub struct SymmetricChannel {
    /// Probability of an error.
    p_e: f64,
}

impl MemorylessChannel<F2Element> for SymmetricChannel {
    /// Send a GF(2) element over a binary symmetric channel.
    ///
    /// Takes `value` to send and random number generator `rng`.
    ///
    /// Returns received value.
    fn send<R>(&self, value: F2Element, rng: &mut R) -> F2Element
    where
        R: rand::Rng + rand::RngExt,
    {
        value.conditionally_flipped(rng.random::<f64>() < self.p_e)
    }
}

impl std::str::FromStr for SymmetricChannel {
    type Err = String;

    /// Construct symmeric channel from a string.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<f64>() {
            Ok(p_e) if (0.0..=1.0).contains(&p_e) => Ok(Self { p_e }),
            Ok(p_e) => Err(format!(
                "p_e value {p_e} is not between 0 and 1 (inclusive)"
            )),
            Err(e) => Err(format!("{e}")),
        }
    }
}
