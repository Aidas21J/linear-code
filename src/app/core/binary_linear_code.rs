use crate::app::{
    algebra::{dmat::DMat, drvec::DRVec, f2::F2},
    core::linear_code::LinearCode,
};

pub struct BinaryLinearCode {
    generator: DMat<F2>,
    parity_check: DMat<F2>,
    syndrome_min_weight: Vec<usize>,
}

impl LinearCode for BinaryLinearCode {
    type Field = F2;

    fn generator(&self) -> &DMat<Self::Field> {
        &self.generator
    }

    fn parity_check(&self) -> &DMat<Self::Field> {
        &self.parity_check
    }

    fn get_syndrome_min_weight(&self, syndrome: &DRVec<Self::Field>) -> usize {
        self.syndrome_min_weight[syndrome.index()]
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
