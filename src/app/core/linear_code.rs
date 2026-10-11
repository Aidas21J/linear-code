use crate::app::algebra::{
    DynamicMatrix, DynamicVector, DynamicVectorValueIterator, Field, FiniteField, TransposableTo,
};

/// Abstract linear code trait that uses dynamic matrices and vectors.
pub trait LinearCode {
    type Field: FiniteField;

    type RowVector: DynamicVector<Field = Self::Field> + TransposableTo<Self::ColVector>;
    type ColVector: DynamicVector<Field = Self::Field> + TransposableTo<Self::RowVector>;

    type Matrix: DynamicMatrix<Field = Self::Field, RowVector = Self::RowVector, ColVector = Self::ColVector>;

    /// Returns generator matrix.
    fn generator(&self) -> &Self::Matrix;
    /// Returns parity-check matrix.
    fn parity_check(&self) -> &Self::Matrix;
    /// Returns coset leader weight for a given syndrome.
    fn get_syndrome_min_weight(&self, syndrome: &Self::RowVector) -> usize;

    /// Makes generator from the its parity part.
    fn generator_from_parity(parity: Self::Matrix) -> Self::Matrix {
        let i_k = Self::Matrix::identity(parity.rows());

        Self::Matrix::join(&i_k, &parity)
    }

    /// Makes parity-check matrix from the generator matrix.
    fn parity_check_from_generator(generator: &Self::Matrix) -> Self::Matrix {
        let (k, n) = generator.dims();
        let minus_a_t = Self::Matrix::neg(generator.split_off(n - k).transpose());
        let i_nk = Self::Matrix::identity(n - k);

        Self::Matrix::join(&minus_a_t, &i_nk)
    }

    /// Calculates coset leader weights for each syndrome.
    fn syndrome_min_weight_from_parity_check(parity_check: &Self::Matrix) -> Vec<usize>
    where
        usize: From<<Self::Field as Field>::Element>,
    {
        let (syndrome_length, received_word_cols) = parity_check.dims();
        let mut coset_leader_weights =
            vec![usize::MAX; Self::Field::ORDER.pow(syndrome_length as u32)];

        for codeword in DynamicVectorValueIterator::<Self::RowVector>::new(received_word_cols) {
            let w = codeword.weight();
            let syndrome =
                Self::Matrix::mul_col(parity_check, &codeword.into_transpose()).into_transpose();

            coset_leader_weights[syndrome.index()] = coset_leader_weights[syndrome.index()].min(w);
        }

        coset_leader_weights
    }

    /// Returns linear code length `n`.
    fn n(&self) -> usize {
        self.generator().cols()
    }

    /// Returns linear code dimension `k`.
    fn k(&self) -> usize {
        self.generator().rows()
    }

    /// Encodes `message`.
    ///
    /// Takes `message` vector.
    ///
    /// Returns encoded `message`: `message` multiplied by generator matrix.
    fn encode(&self, message: &Self::RowVector) -> Self::RowVector {
        Self::Matrix::mul_row(message, self.generator())
    }

    /// Decodes `codeword`.
    ///
    /// Takes `codeword` vector.
    ///
    /// Returns decoded `codeword`.
    ///
    /// # Decoding
    /// Reference: S.A.Vanstone, P.C. van Oorschot. An introduction to error correcting codes with applications. Kluwer Academic Publishers, Boston, 1989 (p. 79).
    ///
    /// Let H be the parity-check matrix, and r = (r_1 r_2 ... r_n) be the received word.
    ///     1) Set i = 1.
    ///     2) Compute H * r^T and the weight w of the corresponding coset leader.
    ///     3) If w = 0, stop with r as the transmitted codeword.
    ///     4) If H * (r + e_i)^T has smaller associated weight than H * r^T, set r = r + e_i.
    ///     5) Set i = i + 1 and go to (2).
    fn decode(&self, codeword: Self::RowVector) -> Self::RowVector {
        let mut r = codeword;

        for i in 1..=r.dim() {
            let syndrome =
                Self::Matrix::mul_col(self.parity_check(), &r.transpose()).into_transpose();
            let w = self.get_syndrome_min_weight(&syndrome);

            if w == 0 {
                break;
            }

            for coef in Self::Field::non_zero_elements() {
                let e_i = Self::RowVector::e(i - 1, r.dim() - i, coef);
                let new_r = e_i.into_add(&r);

                let new_syndrome =
                    Self::Matrix::mul_col(self.parity_check(), &new_r.transpose()).into_transpose();
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
