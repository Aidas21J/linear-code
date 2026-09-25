use std::fmt::{Debug, Display};

use super::{field::Field, randomizable::Randomizable};

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

    pub fn dims(&self) -> (usize, usize) {
        (self.rows, self.cols)
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

    pub fn from_data(rows: usize, data: Vec<F::Element>) -> Option<Self> {
        if !data.len().is_multiple_of(rows) {
            return None;
        }

        Some(Self {
            rows,
            cols: data.len().checked_div_euclid(rows)?,
            data,
        })
    }

    pub fn generate_uniform(rows: usize, cols: usize, rng: &mut impl rand::prelude::Rng) -> Self
    where
        F::Element: Randomizable,
    {
        let data: Vec<F::Element> = (0..(rows * cols))
            .map(|_| F::Element::generate_uniform(rng))
            .collect();

        Self { rows, cols, data }
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
    pub fn join(a: &Self, b: &Self) -> Self {
        debug_assert_eq!(
            a.rows(),
            b.rows(),
            "cannot join matrices with different number of rows"
        );

        let rows = a.rows;
        let cols = a.cols + b.cols;
        let data = a
            .rows_iter()
            .zip(b.rows_iter())
            .flat_map(|(a_row, b_row)| [a_row, b_row].concat())
            .collect();

        Self { rows, cols, data }
    }

    pub fn take(&self, cols: usize) -> Self {
        let rows = self.rows();
        let data = self
            .rows_iter()
            .flat_map(|row| row.iter().take(cols))
            .cloned()
            .collect::<Vec<_>>();

        Self { rows, cols, data }
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
            rows: self.cols(),
            cols: self.rows(),
            data,
        }
    }
}

impl<F: Field> std::ops::Neg for &DMat<F> {
    type Output = DMat<F>;

    fn neg(self) -> Self::Output {
        DMat {
            rows: self.rows(),
            cols: self.cols(),
            data: self
                .data
                .iter()
                .map(|element| F::neg(element.clone()))
                .collect(),
        }
    }
}

impl<F: Field> std::ops::Neg for DMat<F> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        DMat {
            rows: self.rows(),
            cols: self.cols(),
            data: self
                .data
                .into_iter()
                .map(|element| F::neg(element))
                .collect(),
        }
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
            rows: m.rows(),
            cols: m.cols(),
            data: m.data.iter().map(|element| F::mul(s, element)).collect(),
        }
    }

    pub fn into_scalar_mul(s: &F::Element, m: Self) -> Self {
        Self {
            rows: m.rows(),
            cols: m.cols(),
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
            self.cols(),
            rhs.rows(),
            "matrix dimentions do not match for multiplication"
        );

        let mut res = DMat::<F>::zeros(self.rows(), rhs.cols());

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
                std::iter::once("|".to_string())
                    .chain(row.iter().map(|n| n.to_string()))
                    .chain(std::iter::once("|".to_string()))
                    .collect::<Vec<_>>()
                    .join(" ")
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
