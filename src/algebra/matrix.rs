use std::fmt::{Debug, Display};

use crate::algebra::field::Field;

pub struct DMat<F: Field> {
    rows: usize,
    cols: usize,
    data: Vec<F::Element>,
}

// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// Matrix getters ------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------

impl<F: Field> DMat<F> {
    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// Matrix creation -----------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------

impl<F: Field> DMat<F> {
    pub fn zeros(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: vec![F::ZERO; rows * cols],
        }
    }

    pub fn identity(dim: usize) -> Self {
        let mut data = vec![F::ZERO; dim * dim];

        data.iter_mut()
            .step_by(dim + 1)
            .for_each(|element| *element = F::ONE);

        Self {
            rows: dim,
            cols: dim,
            data,
        }
    }

    pub fn from_row(row: Vec<F::Element>) -> Self {
        Self {
            rows: 1,
            cols: row.len(),
            data: row,
        }
    }

    pub fn from_rows(rows_data: Vec<Vec<F::Element>>) -> Option<Self> {
        let rows = rows_data.len();
        let cols = match rows_data.first() {
            Some(row) => row.len(),
            None => return None,
        };

        if rows_data.iter().any(|row| row.len() != cols) {
            return None;
        }

        Some(Self {
            rows,
            cols,
            data: rows_data.into_iter().flatten().collect(),
        })
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// Matrix iteration ----------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------

impl<F: Field> DMat<F> {
    pub fn rows_iter(&self) -> std::slice::ChunksExact<'_, F::Element> {
        self.data.chunks_exact(self.cols)
    }

    pub fn rows_iter_mut(&mut self) -> std::slice::ChunksExactMut<'_, F::Element> {
        self.data.chunks_exact_mut(self.cols)
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// Matrix manipulation -------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------

impl<F: Field> DMat<F> {
    pub fn join(a: &Self, b: &Self) -> Option<Self> {
        if a.rows() != b.rows() {
            return None;
        }

        let rows = a.rows;
        let cols = a.cols + b.cols;
        let data = a
            .rows_iter()
            .zip(b.rows_iter())
            .flat_map(|(a_row, b_row)| [a_row, b_row].concat())
            .collect();

        Some(Self { rows, cols, data })
    }

    pub fn split(&self, cols: usize) -> Option<(Self, Self)> {
        if self.cols < cols {
            return None;
        }

        let rows = self.rows;
        let (lcols, rcols) = (cols, self.cols - cols);
        let (ldata, rdata): (Vec<&[_]>, Vec<&[_]>) =
            self.rows_iter().map(|row| row.split_at(lcols)).unzip();

        Some((
            Self {
                rows,
                cols: lcols,
                data: ldata.iter().flat_map(|row| row.to_vec()).collect(),
            },
            Self {
                rows,
                cols: rcols,
                data: rdata.iter().flat_map(|row| row.to_vec()).collect(),
            },
        ))
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// Matrix unary operations ---------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------

impl<F: Field> DMat<F> {
    pub fn transpose(&self) -> Self {
        let data: Vec<F::Element> = (0..self.cols)
            .flat_map(|c| (0..self.rows).map(move |r| self.data[r * self.cols + c].clone()))
            .collect();

        Self {
            rows: self.cols,
            cols: self.rows,
            data,
        }
    }
}

impl<F: Field> std::ops::Neg for &DMat<F> {
    type Output = DMat<F>;

    fn neg(self) -> Self::Output {
        DMat {
            rows: self.rows,
            cols: self.cols,
            data: self.data.iter().map(|element| F::neg(element)).collect(),
        }
    }
}

impl<F: Field> std::ops::Neg for DMat<F> {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.data
            .iter_mut()
            .for_each(|element| *element = F::neg(element));

        self
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// Matrix binary operations --------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------

impl<F: Field> DMat<F> {
    pub fn scalar_mul(s: &F::Element, m: &Self) -> Self {
        Self {
            rows: m.rows,
            cols: m.cols,
            data: m.data.iter().map(|element| F::mul(s, element)).collect(),
        }
    }

    pub fn into_scalar_mul(s: &F::Element, m: Self) -> Self {
        Self {
            rows: m.rows,
            cols: m.cols,
            data: m
                .data
                .into_iter()
                .map(|element| F::mul(s, &element))
                .collect(),
        }
    }
}

impl<F: Field> std::ops::Mul for &DMat<F> {
    type Output = DMat<F>;

    fn mul(self, rhs: Self) -> Self::Output {
        debug_assert_eq!(
            self.cols, rhs.rows,
            "matrix dimentions do not match for multiplication"
        );

        let mut res = DMat::<F>::zeros(self.rows, rhs.cols);

        let a_rows = self.rows_iter();
        let res_rows = res.rows_iter_mut();

        for (a_row, res_row) in a_rows.zip(res_rows) {
            let b_rows = rhs.rows_iter();

            for (a_val, b_row) in a_row.iter().zip(b_rows) {
                for (res_val, b_val) in res_row.iter_mut().zip(b_row.iter()) {
                    *res_val = F::add(res_val, &F::mul(a_val, b_val));
                }
            }
        }

        res
    }
}

impl<F: Field> std::ops::Mul<DMat<F>> for &DMat<F> {
    type Output = DMat<F>;

    fn mul(self, rhs: DMat<F>) -> Self::Output {
        self * &rhs
    }
}

impl<F: Field> std::ops::Mul<&DMat<F>> for DMat<F> {
    type Output = DMat<F>;

    fn mul(self, rhs: &DMat<F>) -> Self::Output {
        &self * rhs
    }
}

impl<F: Field> std::ops::Mul<DMat<F>> for DMat<F> {
    type Output = DMat<F>;

    fn mul(self, rhs: Self) -> Self::Output {
        &self * &rhs
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// Matrix formatting ---------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------

impl<F: Field> Debug for DMat<F>
where
    F::Element: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DMat")
            .field("rows", &self.rows)
            .field("cols", &self.cols)
            .field("data", &self.data)
            .finish()
    }
}

impl<F: Field> Display for DMat<F>
where
    F::Element: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let self_str: String = self
            .rows_iter()
            .map(|row| {
                let row_str = row
                    .iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<_>>()
                    .join(" ");
                "|".to_string() + &row_str + "|"
            })
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{self_str}")
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// Matrix equality -----------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------

impl<F: Field> PartialEq for DMat<F> {
    fn eq(&self, other: &Self) -> bool {
        self.rows == other.rows && self.cols == other.cols && self.data == other.data
    }
}

impl<F: Field> Eq for DMat<F> {}

// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// Matrix hash ---------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------

impl<F: Field> std::hash::Hash for DMat<F> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.rows.hash(state);
        self.cols.hash(state);
        self.data.hash(state);
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// Matrix cloning ------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------

impl<F: Field> Clone for DMat<F> {
    fn clone(&self) -> Self {
        Self {
            rows: self.rows.clone(),
            cols: self.cols.clone(),
            data: self.data.clone(),
        }
    }
}
