use std::collections::HashMap;

use crate::app::{
    algebra::{dmat::DMat, drvec::DRVec, f2::F2},
    core::linear_code::LinearCode,
};

pub struct BinaryLinearCode {
    generator: DMat<F2>,
    parity_check: DMat<F2>,
    syndrome_min_weight: HashMap<DRVec<F2>, usize>,
}

impl LinearCode for BinaryLinearCode {
    type Field = F2;

    fn generator(&self) -> &DMat<Self::Field> {
        &self.generator
    }

    fn parity_check(&self) -> &DMat<Self::Field> {
        &self.parity_check
    }

    fn get_syndrome_min_weight(&self, syndrome: &DRVec<Self::Field>) -> Option<&usize> {
        self.syndrome_min_weight.get(syndrome)
    }
}

impl BinaryLinearCode {
    pub fn from_parity(parity: DMat<F2>) -> Option<Self> {
        let generator = Self::generator_from_parity(parity)?;
        let parity_check = Self::parity_check_from_generator(&generator)?;
        let syndrome_min_weight = Self::syndrome_min_weight_from_parity_check(&parity_check)?;

        Some(Self {
            generator,
            parity_check,
            syndrome_min_weight,
        })
    }

    pub fn new_random(n: usize, k: usize, rng: &mut impl rand::prelude::Rng) -> Option<Self> {
        if n < k {
            return None;
        }

        let random_parity = DMat::generate_uniform(k, n - k, rng);

        let generator = Self::generator_from_parity(random_parity)?;
        let parity_check = Self::parity_check_from_generator(&generator)?;
        let syndrome_min_weight = Self::syndrome_min_weight_from_parity_check(&parity_check)?;

        Some(Self {
            generator,
            parity_check,
            syndrome_min_weight,
        })
    }
}
