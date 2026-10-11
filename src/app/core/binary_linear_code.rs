use crate::app::algebra::{DCVec, DMat, DRVec, DynamicVector as _, F2};

use super::LinearCode;

/// Type representing binary linear code.
pub struct BinaryLinearCode {
    /// Generator matrix.
    generator: DMat<F2>,
    /// Parity-check matrix.
    parity_check: DMat<F2>,
    /// List of coset leader weights. Weights are arranged in syndorme lexicographic order.
    syndrome_min_weight: Vec<usize>,
}

impl LinearCode for BinaryLinearCode {
    type Field = F2;

    type RowVector = DRVec<Self::Field>;
    type ColVector = DCVec<Self::Field>;

    type Matrix = DMat<Self::Field>;

    /// Return generator matrix.
    fn generator(&self) -> &Self::Matrix {
        &self.generator
    }

    /// Returns parity-check matrix.
    fn parity_check(&self) -> &Self::Matrix {
        &self.parity_check
    }

    /// Returns coset leader weight for a given syndrome.
    fn get_syndrome_min_weight(&self, syndrome: &Self::RowVector) -> usize {
        self.syndrome_min_weight[syndrome.index()]
    }
}

impl BinaryLinearCode {
    /// Create binary linear code from generator matrix parity part.
    ///
    /// Takes `parity` matrix.
    ///
    /// Returns the code.
    pub fn from_parity(parity: <Self as LinearCode>::Matrix) -> Self {
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
