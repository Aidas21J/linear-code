use rand::{Rng, RngExt};

use crate::{
    algebra::f2::{F2, F2Element},
    core::channel::Channel,
};

pub struct BinaryChannel {
    p_e: f64,
}

impl BinaryChannel {
    pub fn new(p_e: f64) -> Self {
        Self { p_e }
    }
}

impl Channel<F2> for BinaryChannel {
    fn send(&self, value: F2Element, rng: &mut impl Rng) -> F2Element {
        if rng.random::<f64>() < self.p_e {
            value.flip_into()
        } else {
            value
        }
    }

    fn send_mut(&self, value: &mut F2Element, rng: &mut impl Rng) {
        if rng.random::<f64>() < self.p_e {
            value.flip()
        }
    }
}
