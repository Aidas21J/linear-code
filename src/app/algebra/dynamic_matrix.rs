use super::{DynamicVector, Field, Randomizable, TransposableTo};

type Scalar<M> = <<M as DynamicMatrix>::Field as Field>::Element;

/// Abstract dynamic (resizable at runtime) matrix.
pub trait DynamicMatrix: Sized {
    type Field: Field;

    type RowVector: DynamicVector<Field = Self::Field> + TransposableTo<Self::ColVector>;
    type ColVector: DynamicVector<Field = Self::Field> + TransposableTo<Self::RowVector>;

    /// Iterate through elements in row-major order.
    fn flat_iter(&self) -> impl ExactSizeIterator<Item = &Scalar<Self>>;
    /// Iterate (and mutate) through elements in row-major order.
    fn flat_iter_mut(&mut self) -> impl ExactSizeIterator<Item = &mut Scalar<Self>>;

    /// Iterate through rows.
    fn rows_iter(&self) -> std::slice::ChunksExact<'_, Scalar<Self>>;

    /// Get row count.
    fn rows(&self) -> usize;
    /// Get column count.
    fn cols(&self) -> usize;

    /// Get dimensions.
    ///
    /// Returns (row count, column count) tuple.
    fn dims(&self) -> (usize, usize) {
        (self.rows(), self.cols())
    }

    /// Construct matrix from an iterator.
    ///
    /// Takes row count `rows` and row-major element iterator `data`.
    ///
    /// Returns new matrix if it's possible to divide `data` elements into `rows` number
    /// of rows. Otherwise, `None` is returned.
    fn from_data<I>(rows: usize, data: I) -> Option<Self>
    where
        I: IntoIterator<Item = <Self::Field as Field>::Element>;

    /// Construct matrix from iterator.
    ///
    /// Takes row count `rows` and row-major element iterator `data`. `data` length must be
    /// divisible by `rows`.
    ///
    /// Returns new matrix with `data` elements.
    fn from_data_unchecked<I>(rows: usize, data: I) -> Self
    where
        I: IntoIterator<Item = <Self::Field as Field>::Element>;

    /// Construct matrix from generator function.
    ///
    /// Takes row count `rows`, column count `cols` and generator function `generate` what maps row
    /// index and column index to element value.
    ///
    /// Returns new matrix where each element was generated using `generate`.
    fn generate<F>(rows: usize, cols: usize, generate: F) -> Self
    where
        F: FnMut(usize, usize) -> Scalar<Self>;

    /// Construct identity matrix.
    ///
    /// Takes dimension `dim`.
    ///
    /// Returns new square matrix of size `dim` with 1s in the main diagonal and 0s elsewhere, i.e.
    /// identity matrix.
    fn identity(dim: usize) -> Self {
        Self::generate(dim, dim, |x, y| {
            if x == y {
                Self::Field::ONE
            } else {
                Self::Field::ZERO
            }
        })
    }

    /// Generates uniformly random matrix.
    ///
    /// Takes row count `rows`, column count `cols` and reference to random number generator `rng`.
    ///
    /// Returns matrix with uniformly random entries.
    fn generate_uniform<R>(rows: usize, cols: usize, rng: &mut R) -> Self
    where
        Scalar<Self>: Randomizable,
        R: rand::Rng,
    {
        Self::generate(rows, cols, |_, _| {
            <Self::Field as Field>::Element::generate_uniform(rng)
        })
    }

    /// Joins two matrices side-by-side.
    ///
    /// Takes matrices `a` and `b`.
    ///
    /// Returns combined matrix (`a` | `b`).
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

    /// Splits off the end of each matrix row.
    ///
    /// Takes column to split off count `cols`.
    ///
    /// Returns matrix with at least `cols` less columns. If `cols` is greater than the number of
    /// matrix columns, an empty matrix is returned.
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

    /// Returns new transposed matrix.
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

    /// Negates the matrix.
    fn neg(mut self) -> Self {
        self.flat_iter_mut()
            .for_each(|element| *element = Self::Field::neg(element.clone()));
        self
    }

    /// Multiplies a row vector by a matrix.
    ///
    /// Takes row vector `vec` and matrix `mat`.
    ///
    /// Returns `vec` * `mat`.
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

    /// Multiplies a matrix by a column vector.
    ///
    /// Takes matrix `mat` and column vector `vec`.
    ///
    /// Returns `mat` * `vec`.
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
