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

    #[cfg(debug_assertions)]
    fn get_syndrome_min_weight(&self, syndrome: &DRVec<Self::Field>) -> usize {
        let weight_option = self.syndrome_min_weight.get(syndrome);
        debug_assert!(weight_option.is_some(), "could not find {syndrome} weight");

        *weight_option.unwrap_or(&0)
    }

    #[cfg(not(debug_assertions))]
    fn get_syndrome_min_weight(&self, syndrome: &DRVec<Self::Field>) -> usize {
        *self.syndrome_min_weight.get(syndrome).unwrap_or(&0)
    }
}

impl BinaryLinearCode {
    pub fn from_parity(parity: DMat<F2>) -> Self {
        let generator = Self::generator_from_parity(parity);
        let parity_check = Self::parity_check_from_generator(&generator);
        let syndrome_min_weight = Self::syndrome_min_weight_from_parity_check(&parity_check);

        Self {
            generator,
            parity_check,
            syndrome_min_weight,
        }
    }
}
