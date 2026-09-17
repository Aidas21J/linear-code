use std::fmt::{Debug, Display};

use crate::algebra::field::Field;

#[derive(Clone)]
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
    fn rows_iter(&self) -> std::slice::ChunksExact<'_, F::Element> {
        self.data.chunks_exact(self.cols)
    }

    fn rows_iter_mut(&mut self) -> std::slice::ChunksExactMut<'_, F::Element> {
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
        assert_eq!(
            a.rows, b.rows,
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

    pub fn split(&self, cols: usize) -> (Self, Self) {
        assert!(cols <= self.cols, "cannot split more cols than matrix has");

        let rows = self.rows;
        let (lcols, rcols) = (cols, self.cols - cols);
        let (ldata, rdata): (Vec<&[_]>, Vec<&[_]>) =
            self.rows_iter().map(|row| row.split_at(lcols)).unzip();

        (
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
        )
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

// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// Matrix binary operations --------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------

impl<F: Field> DMat<F> {
    pub fn add(a: &Self, b: &Self) -> Self {
        assert_eq!(
            a.rows, b.rows,
            "cannot add matrices with different number of rows"
        );
        assert_eq!(
            a.cols, b.cols,
            "cannot add matrices with different number of cols"
        );

        Self {
            rows: a.rows,
            cols: a.cols,
            data: a
                .data
                .iter()
                .zip(b.data.iter())
                .map(|(x, y)| F::add(x, y))
                .collect(),
        }
    }

    pub fn scalar_mul(s: &F::Element, m: &Self) -> Self {
        Self {
            rows: m.rows,
            cols: m.cols,
            data: m.data.iter().map(|element| F::mul(s, element)).collect(),
        }
    }

    pub fn mul(a: &Self, b: &Self) -> Self {
        assert_eq!(
            a.cols, b.rows,
            "matrix dimentions do not match for multiplication"
        );

        let mut res = Self::zeros(a.rows, b.cols);

        let a_rows = a.rows_iter();
        let res_rows = res.rows_iter_mut();

        for (a_row, res_row) in a_rows.zip(res_rows) {
            let b_rows = b.rows_iter();

            for (a_val, b_row) in a_row.iter().zip(b_rows) {
                for (res_val, b_val) in res_row.iter_mut().zip(b_row.iter()) {
                    *res_val = F::add(res_val, &F::mul(a_val, b_val));
                }
            }
        }

        res
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
