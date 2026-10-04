use crate::app::algebra::{
    dynamic_vector::DynamicVector, field::Field, randomizable::Randomizable,
    transposable_vector::TransposableTo,
};

type Scalar<M> = <<M as DynamicMatrix>::Field as Field>::Element;

#[expect(unused)]
pub trait DynamicMatrix: Sized {
    type Field: Field;

    type RowVector: DynamicVector<Field = Self::Field> + TransposableTo<Self::ColVector>;
    type ColVector: DynamicVector<Field = Self::Field> + TransposableTo<Self::RowVector>;

    fn flat_iter(&self) -> impl ExactSizeIterator<Item = &Scalar<Self>>;
    fn flat_iter_mut(&mut self) -> impl ExactSizeIterator<Item = &mut Scalar<Self>>;
    fn into_flat_iter(self) -> impl ExactSizeIterator<Item = Scalar<Self>>;

    fn rows_iter(&self) -> std::slice::ChunksExact<'_, Scalar<Self>>;
    fn rows_iter_mut(&mut self) -> std::slice::ChunksExactMut<'_, Scalar<Self>>;

    fn rows(&self) -> usize;
    fn cols(&self) -> usize;

    fn dims(&self) -> (usize, usize) {
        (self.rows(), self.cols())
    }

    fn from_data<I>(rows: usize, data: I) -> Option<Self>
    where
        I: IntoIterator<Item = <Self::Field as Field>::Element>;

    fn from_data_unchecked<I>(rows: usize, data: I) -> Self
    where
        I: IntoIterator<Item = <Self::Field as Field>::Element>;

    fn generate<F>(rows: usize, cols: usize, generate: F) -> Self
    where
        F: FnMut(usize, usize) -> Scalar<Self>;

    fn from_value(rows: usize, cols: usize, value: Scalar<Self>) -> Self {
        Self::generate(rows, cols, |_, _| value.clone())
    }

    fn zeros(rows: usize, cols: usize) -> Self {
        Self::from_value(rows, cols, Self::Field::ZERO)
    }

    fn identity(dim: usize) -> Self {
        Self::generate(dim, dim, |x, y| {
            if x == y {
                Self::Field::ONE
            } else {
                Self::Field::ZERO
            }
        })
    }

    fn generate_uniform<R>(rows: usize, cols: usize, rng: &mut R) -> Self
    where
        Scalar<Self>: Randomizable,
        R: rand::Rng,
    {
        Self::generate(rows, cols, |_, _| {
            <Self::Field as Field>::Element::generate_uniform(rng)
        })
    }

    fn join(a: &Self, b: &Self) -> Self {
        debug_assert_eq!(
            a.rows(),
            b.rows(),
            "cannot join matrices with different number of rows"
        );

        Self::from_data_unchecked(
            a.rows(),
            a.rows_iter()
                .zip(b.rows_iter())
                .flat_map(|(a_row, b_row)| [a_row, b_row].concat()),
        )
    }

    fn split_off(&self, cols: usize) -> Self {
        let skip_cols = self.cols().saturating_sub(cols);

        Self::from_data_unchecked(
            self.rows(),
            self.rows_iter()
                .map(|row| row.iter().skip(skip_cols))
                .flatten()
                .cloned(),
        )
    }

    fn transpose(&self) -> Self {
        Self::from_data_unchecked(
            self.cols(),
            (0..self.cols()).flat_map(|col| {
                self.flat_iter()
                    .skip(col)
                    .step_by(self.cols())
                    .take(self.rows())
                    .cloned()
            }),
        )
    }

    fn neg(mut self) -> Self {
        self.flat_iter_mut()
            .for_each(|element| *element = Self::Field::neg(element.clone()));
        self
    }

    fn mul_row(vec: &Self::RowVector, mat: &Self) -> Self::RowVector {
        debug_assert_eq!(
            vec.dim(),
            mat.rows(),
            "vector and matrix dimentions do not match for multiplication"
        );

        let mut res = Self::RowVector::zeros(mat.cols());
        let mat_rows = mat.rows_iter();

        for (vec_val, mat_row) in vec.iter().zip(mat_rows) {
            for (res_val, mat_val) in res.iter_mut().zip(mat_row.iter()) {
                *res_val = Self::Field::add(res_val, &Self::Field::mul(vec_val, mat_val));
            }
        }

        res
    }

    fn mul_col(mat: &Self, vec: &Self::ColVector) -> Self::ColVector {
        debug_assert_eq!(
            mat.cols(),
            vec.dim(),
            "matrix dimentions do not match for multiplication"
        );

        let mut res = Self::ColVector::zeros(mat.rows());

        for (mat_row, res_val) in mat.rows_iter().zip(res.iter_mut()) {
            for (mat_val, vec_val) in mat_row.iter().zip(vec.iter()) {
                *res_val = Self::Field::add(res_val, &Self::Field::mul(mat_val, vec_val));
            }
        }

        res
    }
}
