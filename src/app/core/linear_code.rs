use crate::app::algebra::{
    dmat::DMat,
    drvec::{DRVec, DRVecIterator},
    field::Field,
    finite_field::FiniteField,
};

pub trait LinearCode {
    type Field: FiniteField;

    fn generator(&self) -> &DMat<Self::Field>;
    fn parity_check(&self) -> &DMat<Self::Field>;
    fn get_syndrome_min_weight(&self, syndrome: &DRVec<Self::Field>) -> usize;

    fn generator_from_parity(parity: DMat<Self::Field>) -> DMat<Self::Field> {
        DMat::join(&DMat::identity(parity.rows()), &parity)
    }

    fn parity_check_from_generator(generator: &DMat<Self::Field>) -> DMat<Self::Field> {
        let (k, n) = generator.dims();
        let minus_a_t = -generator.split_off(n - k).transpose();

        DMat::join(&minus_a_t, &DMat::identity(n - k))
    }

    fn syndrome_min_weight_from_parity_check(parity_check: &DMat<Self::Field>) -> Vec<usize>
    where
        usize: From<<Self::Field as Field>::Element>,
    {
        let (syndrome_length, received_word_cols) = parity_check.dims();
        let mut coset_leader_weights =
            vec![usize::MAX; Self::Field::ORDER.pow(syndrome_length as u32)];

        for codeword in DRVecIterator::new(received_word_cols) {
            let w = codeword.weight();
            let syndrome = (parity_check * &codeword.into_transpose()).into_transpose();

            coset_leader_weights[syndrome.index()] = coset_leader_weights[syndrome.index()].min(w);
        }

        coset_leader_weights
    }

    fn n(&self) -> usize {
        self.generator().cols()
    }

    fn k(&self) -> usize {
        self.generator().rows()
    }

    fn encode(&self, codeword: &DRVec<Self::Field>) -> DRVec<Self::Field> {
        codeword * self.generator()
    }

    fn decode(&self, codeword: DRVec<Self::Field>) -> DRVec<Self::Field> {
        let mut r = codeword;

        for i in 1..=r.cols() {
            let syndrome = (self.parity_check() * &r.transpose()).into_transpose();
            let w = self.get_syndrome_min_weight(&syndrome);

            if w == 0 {
                break;
            }

            for coef in Self::Field::non_zero_elements() {
                let e_i = DRVec::e(i - 1, r.cols() - i, coef);
                let new_r = &r + e_i;

                let new_syndrome = (self.parity_check() * &new_r.transpose()).into_transpose();
                let new_w = self.get_syndrome_min_weight(&new_syndrome);

                if new_w < w {
                    r = new_r;
                    break;
                }
            }
        }

        r.take(self.k())
    }
}
