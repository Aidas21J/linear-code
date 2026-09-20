use std::collections::HashMap;

use crate::algebra::{
    dmat::DMat,
    drvec::{DRVec, DRVecIterator},
    finite_field::FiniteField,
};

pub trait LinearCode {
    type Field: FiniteField;

    fn generator(&self) -> &DMat<Self::Field>;
    fn parity_check(&self) -> &DMat<Self::Field>;
    fn get_syndrome_min_weight(&self, syndrome: &DRVec<Self::Field>) -> Option<&usize>;

    fn generator_from_parity(parity: DMat<Self::Field>) -> Option<DMat<Self::Field>> {
        DMat::join(&parity, &DMat::identity(parity.rows()))
    }

    fn parity_check_from_generator(generator: &DMat<Self::Field>) -> Option<DMat<Self::Field>> {
        let (k, n) = generator.dims();
        let minus_a_t = -generator.split(n - k)?.0.transpose();

        DMat::join(&DMat::identity(n - k), &minus_a_t)
    }

    fn syndrome_min_weight_from_parity_check<F: FiniteField>(
        parity_check: &DMat<F>,
    ) -> Option<HashMap<DRVec<F>, usize>> {
        let received_word_cols = parity_check.cols();
        let mut coset_leader_weights: HashMap<DRVec<F>, usize> = HashMap::new();

        for codeword in DRVecIterator::<F>::new(received_word_cols, F::ONE)? {
            let w = codeword.weight();
            let syndrome = (parity_check * codeword.into_transpose()).into_transpose();

            coset_leader_weights
                .entry(syndrome)
                .and_modify(|current_w| *current_w = std::cmp::min(*current_w, w))
                .or_insert(w);
        }

        Some(coset_leader_weights)
    }

    fn n(&self) -> usize {
        self.generator().cols()
    }

    fn k(&self) -> usize {
        self.generator().rows()
    }

    fn encode(&self, codeword: &DRVec<Self::Field>) -> DRVec<Self::Field> {
        codeword * &self.generator()
    }

    fn decode(&self, codeword: &DRVec<Self::Field>) -> Option<DRVec<Self::Field>> {
        let mut r = codeword.clone();

        for i in 1..=r.cols() {
            let syndrome = (self.parity_check() * r.transpose()).into_transpose();
            let w = self.get_syndrome_min_weight(&syndrome)?;

            if *w == 0 {
                break;
            }

            for coef in Self::Field::non_zero_elements() {
                let e_i = DRVec::e_i(i - 1, r.cols() - i);
                let new_r = &r + DRVec::into_scalar_mul(&coef, e_i);

                let new_syndrome = (self.parity_check() * new_r.transpose()).into_transpose();
                let new_w = self.get_syndrome_min_weight(&new_syndrome)?;

                if new_w < w {
                    r = new_r;
                    break;
                }
            }
        }

        r.split(self.n() - self.k()).map(|(_, original)| original)
    }
}
