use crate::app::{algebra::f2::F2Element, core::memoryless_channel::MemorylessChannel};

pub struct SymmetricChannel {
    p_e: f64,
}

impl SymmetricChannel {
    pub fn new(p_e: f64) -> Self {
        Self { p_e }
    }
}

impl MemorylessChannel<F2Element> for SymmetricChannel {
    fn send<R>(&self, value: F2Element, rng: &mut R) -> F2Element
    where
        R: rand::Rng + rand::RngExt,
    {
        value.conditionally_fliped(rng.random::<f64>() < self.p_e)
    }
}
