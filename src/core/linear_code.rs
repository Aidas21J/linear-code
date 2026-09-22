use std::collections::HashMap;

use crate::algebra::{
    dmat::DMat,
    drvec::{DRVec, DRVecIterator},
    field::Field,
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

    fn encode(&self, codeword: DRVec<Self::Field>) -> DRVec<Self::Field> {
        codeword * &self.generator()
    }

    fn decode(&self, codeword: DRVec<Self::Field>) -> Option<DRVec<Self::Field>> {
        let mut r = codeword;

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

    fn encode_buffer(
        &self,
        buffer: &Vec<<Self::Field as Field>::Element>,
    ) -> (Vec<<Self::Field as Field>::Element>, usize) {
        let chunks_iter = buffer.chunks_exact(self.k());
        let remainder_chunk = chunks_iter.remainder();

        let mut data = chunks_iter
            .map(DRVec::from_row_slice)
            .map(|word| self.encode(word))
            .flat_map(DRVec::into_data)
            .collect::<Vec<_>>();

        let mut pad_size = 0;
        if !remainder_chunk.is_empty() {
            let mut last_chunk = remainder_chunk.to_vec();

            pad_size = self.k() - last_chunk.len();
            last_chunk.resize(self.k(), <Self::Field as Field>::ZERO);

            let word = DRVec::from_row(last_chunk);
            data.extend(self.encode(word).into_data());
        }

        (data, pad_size)
    }

    fn decode_buffer(
        &self,
        buffer: &Vec<<Self::Field as Field>::Element>,
        pad_size: usize,
    ) -> Vec<<Self::Field as Field>::Element> {
        let decode_result_on_fail = DRVec::from_row(vec![Self::Field::ZERO; self.k()]);
        let chunks_iter = buffer.chunks_exact(self.n());

        chunks_iter
            .map(DRVec::from_row_slice)
            .map(|word| self.decode(word).unwrap_or(decode_result_on_fail.clone()))
            .flat_map(DRVec::into_data)
            .take(buffer.len() * self.k() - pad_size)
            .collect::<Vec<_>>()
    }
}
