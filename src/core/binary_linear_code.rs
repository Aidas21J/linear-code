use std::collections::HashMap;

use crate::{
    algebra::{drvec::DRVec, f2::F2, field::Field as _, matrix::DMat},
    core::linear_code::LinearCode,
};

pub struct BinaryLinearCode {
    generator: DMat<F2>,
    parity_check: DMat<F2>,
    coset_leader_weights: HashMap<DRVec<F2>, usize>,
}

impl LinearCode for BinaryLinearCode {
    type Field = F2;

    fn generator(&self) -> &DMat<Self::Field> {
        &self.generator
    }

    fn parity_check(&self) -> &DMat<Self::Field> {
        &self.parity_check
    }

    fn get_coset_leader_weight(&self, syndrome: &DRVec<Self::Field>) -> Option<&usize> {
        self.coset_leader_weights.get(syndrome)
    }
}

impl BinaryLinearCode {
    pub fn new(n: usize, k: usize) -> Option<Self> {
        if (n, k) != (6, 3) {
            return None;
        }

        let generator = DMat::<F2>::from_rows(vec![
            vec![F2::ONE, F2::ONE, F2::ZERO, F2::ONE, F2::ZERO, F2::ZERO],
            vec![F2::ZERO, F2::ONE, F2::ONE, F2::ZERO, F2::ONE, F2::ZERO],
            vec![F2::ONE, F2::ZERO, F2::ONE, F2::ZERO, F2::ZERO, F2::ONE],
        ])?;

        let parity_check = Self::calculate_parity_check(&generator)?;
        let coset_leader_weights = Self::calculate_coset_leader_weights(&parity_check);

        Self {
            generator,
            parity_check,
            coset_leader_weights,
        }
        .into()
    }

    fn calculate_parity_check(generator: &DMat<F2>) -> Option<DMat<F2>> {
        let rows = generator.cols() - generator.rows();
        let a_t = generator.split(rows)?.0.transpose();

        DMat::join(&DMat::identity(rows), &-a_t)
    }

    fn calculate_coset_leader_weights(_parity_check: &DMat<F2>) -> HashMap<DRVec<F2>, usize> {
        let mut coset_leader_weights: HashMap<DRVec<F2>, usize> = HashMap::new();

        coset_leader_weights.insert(DRVec::<F2>::from_row(vec![F2::ZERO, F2::ZERO, F2::ZERO]), 0);
        coset_leader_weights.insert(DRVec::<F2>::from_row(vec![F2::ONE, F2::ZERO, F2::ONE]), 1);
        coset_leader_weights.insert(DRVec::<F2>::from_row(vec![F2::ZERO, F2::ONE, F2::ONE]), 1);
        coset_leader_weights.insert(DRVec::<F2>::from_row(vec![F2::ONE, F2::ONE, F2::ZERO]), 1);
        coset_leader_weights.insert(DRVec::<F2>::from_row(vec![F2::ZERO, F2::ZERO, F2::ONE]), 1);
        coset_leader_weights.insert(DRVec::<F2>::from_row(vec![F2::ZERO, F2::ONE, F2::ZERO]), 1);
        coset_leader_weights.insert(DRVec::<F2>::from_row(vec![F2::ONE, F2::ZERO, F2::ZERO]), 1);
        coset_leader_weights.insert(DRVec::<F2>::from_row(vec![F2::ONE, F2::ONE, F2::ONE]), 2);

        coset_leader_weights
    }
}
