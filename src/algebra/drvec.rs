use crate::algebra::{dcvec::DCVec, field::Field, matrix::DMat};

pub struct DRVec<F: Field> {
    cols: usize,
    data: Vec<F::Element>,
}

// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// Row vector getters --------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------

impl<F: Field> DRVec<F> {
    pub fn cols(&self) -> usize {
        self.cols
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// Row vector creation -------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------

impl<F: Field> DRVec<F> {
    pub fn from_row(row: Vec<F::Element>) -> Self {
        Self {
            cols: row.len(),
            data: row,
        }
    }

    pub fn zeros(cols: usize) -> Self {
        Self {
            cols,
            data: vec![F::ZERO; cols],
        }
    }

    pub fn e_i(zeros_before: usize, zeros_after: usize) -> Self {
        let mut row = vec![F::ZERO; zeros_before + 1 + zeros_after];
        row[zeros_before] = F::ONE;

        Self::from_row(row)
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// Row vector manipulation ---------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------

impl<F: Field> DRVec<F> {
    pub fn join(a: &Self, b: &Self) -> Self {
        let cols = a.cols + b.cols;
        let data = [a.data.clone(), b.data.clone()].concat();

        Self { cols, data }
    }

    pub fn split(&self, cols: usize) -> Option<(Self, Self)> {
        if self.cols < cols {
            return None;
        }

        let (lcols, rcols) = (cols, self.cols - cols);
        let (ldata, rdata): (&[_], &[_]) = self.data.split_at(lcols);

        Some((
            Self {
                cols: lcols,
                data: ldata.to_vec(),
            },
            Self {
                cols: rcols,
                data: rdata.to_vec(),
            },
        ))
    }

    pub fn take(&self, cols: usize) -> Self {
        Self {
            cols,
            data: self.data.iter().take(cols).cloned().collect(),
        }
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// Row vector unary operations -----------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------

impl<F: Field> DRVec<F> {
    pub fn transpose(&self) -> DCVec<F> {
        DCVec::<F>::from_col(self.data.clone())
    }

    pub fn into_transpose(self) -> DCVec<F> {
        DCVec::<F>::from_col(self.data)
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// Row vector binary operations ----------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------

impl<F: Field> DRVec<F> {
    pub fn scalar_mul(s: &F::Element, m: &Self) -> Self {
        Self {
            cols: m.cols,
            data: m.data.iter().map(|element| F::mul(s, element)).collect(),
        }
    }

    pub fn into_scalar_mul(s: &F::Element, mut m: Self) -> Self {
        m.data
            .iter_mut()
            .for_each(|element| *element = F::mul(s, element));

        m
    }
}

impl<F: Field> std::ops::Add<&DRVec<F>> for &DRVec<F> {
    type Output = DRVec<F>;

    fn add(self, rhs: &DRVec<F>) -> Self::Output {
        debug_assert_eq!(
            self.cols, rhs.cols,
            "cannot add row vectors with different number of cols"
        );

        Self::Output {
            cols: self.cols,
            data: self
                .data
                .iter()
                .zip(rhs.data.iter())
                .map(|(x, y)| F::add(x, y))
                .collect(),
        }
    }
}

impl<F: Field> std::ops::Add<&DRVec<F>> for DRVec<F> {
    type Output = DRVec<F>;

    fn add(self, rhs: &DRVec<F>) -> Self::Output {
        debug_assert_eq!(
            self.cols, rhs.cols,
            "cannot add row vectors with different number of cols"
        );

        Self::Output {
            cols: self.cols,
            data: self
                .data
                .into_iter()
                .zip(rhs.data.iter())
                .map(|(x, y)| F::add(&x, y))
                .collect(),
        }
    }
}

impl<F: Field> std::ops::Add<DRVec<F>> for &DRVec<F> {
    type Output = DRVec<F>;

    fn add(self, rhs: DRVec<F>) -> Self::Output {
        debug_assert_eq!(
            self.cols, rhs.cols,
            "cannot add row vectors with different number of cols"
        );

        Self::Output {
            cols: self.cols,
            data: rhs
                .data
                .into_iter()
                .zip(self.data.iter())
                .map(|(y, x)| F::add(x, &y))
                .collect(),
        }
    }
}

impl<F: Field> std::ops::Add<DRVec<F>> for DRVec<F> {
    type Output = DRVec<F>;

    fn add(self, rhs: DRVec<F>) -> Self::Output {
        self + &rhs
    }
}

impl<F: Field> std::ops::Mul<&DMat<F>> for &DRVec<F> {
    type Output = DRVec<F>;

    fn mul(self, rhs: &DMat<F>) -> Self::Output {
        debug_assert_eq!(
            self.cols(),
            rhs.rows(),
            "matrix dimentions do not match for multiplication"
        );

        let mut res = DRVec::zeros(rhs.cols());
        let b_rows = rhs.rows_iter();

        for (a_val, b_row) in self.data.iter().zip(b_rows) {
            for (res_val, b_val) in res.data.iter_mut().zip(b_row.iter()) {
                *res_val = F::add(res_val, &F::mul(a_val, b_val));
            }
        }

        res
    }
}

impl<F: Field> std::ops::Mul<&DMat<F>> for DRVec<F> {
    type Output = DRVec<F>;

    fn mul(self, rhs: &DMat<F>) -> Self::Output {
        &self * rhs
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// Row vector formatting -----------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------

impl<F: Field> std::fmt::Debug for DRVec<F>
where
    F::Element: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DRVec")
            .field("cols", &self.cols)
            .field("data", &self.data)
            .finish()
    }
}

impl<F: Field> std::fmt::Display for DRVec<F>
where
    F::Element: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let self_str: String = self
            .data
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        write!(f, "|{self_str}|")
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// Row vector equality -------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------

impl<F: Field> PartialEq for DRVec<F> {
    fn eq(&self, other: &Self) -> bool {
        self.cols == other.cols && self.data == other.data
    }
}

impl<F: Field> Eq for DRVec<F> {}

// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// Row vector hash -----------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------

impl<F: Field> std::hash::Hash for DRVec<F> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.cols.hash(state);
        self.data.hash(state);
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// Row vector cloning --------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------

impl<F: Field> Clone for DRVec<F> {
    fn clone(&self) -> Self {
        Self {
            cols: self.cols.clone(),
            data: self.data.clone(),
        }
    }
}
