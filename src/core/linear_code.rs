use crate::algebra::{
    drvec::DRVec, field::Field, finite_field::FiniteField,
    finite_underlying_set::FiniteUnderlyingSet, matrix::DMat,
};

pub trait LinearCode
where
    <Self::Field as Field>::Element: FiniteUnderlyingSet,
{
    type Field: FiniteField;

    fn generator(&self) -> &DMat<Self::Field>;
    fn parity_check(&self) -> &DMat<Self::Field>;
    fn get_coset_leader_weight(&self, syndrome: &DRVec<Self::Field>) -> Option<&usize>;

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
            let w = self.get_coset_leader_weight(&syndrome)?;

            if *w == 0 {
                break;
            }

            for coef in Self::Field::non_zero_elements() {
                let e_i = DRVec::e_i(i - 1, r.cols() - i);
                let new_r = &r + DRVec::into_scalar_mul(&coef, e_i);

                let new_syndrome = (self.parity_check() * new_r.transpose()).into_transpose();
                let new_w = self.get_coset_leader_weight(&new_syndrome)?;

                if new_w < w {
                    r = new_r;
                    break;
                }
            }
        }

        r.split(self.n() - self.k()).map(|(_, original)| original)
    }
}
