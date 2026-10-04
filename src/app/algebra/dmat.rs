use crate::app::algebra::{
    dcvec::DCVec, drvec::DRVec, dynamic_matrix::DynamicMatrix, field::Field,
};

pub struct DMat<F: Field> {
    rows: usize,
    cols: usize,
    data: Vec<F::Element>,
}

impl<F: Field> DynamicMatrix for DMat<F> {
    type Field = F;

    type RowVector = DRVec<F>;

    type ColVector = DCVec<F>;

    fn flat_iter(&self) -> impl ExactSizeIterator<Item = &<Self::Field as Field>::Element> {
        self.data.iter()
    }

    fn flat_iter_mut(
        &mut self,
    ) -> impl ExactSizeIterator<Item = &mut <Self::Field as Field>::Element> {
        self.data.iter_mut()
    }

    fn into_flat_iter(self) -> impl ExactSizeIterator<Item = <Self::Field as Field>::Element> {
        self.data.into_iter()
    }

    fn rows_iter(&self) -> std::slice::ChunksExact<'_, <Self::Field as Field>::Element> {
        self.data.chunks_exact(self.cols())
    }

    fn rows_iter_mut(&mut self) -> std::slice::ChunksExactMut<'_, <Self::Field as Field>::Element> {
        self.data.chunks_exact_mut(self.cols)
    }

    fn rows(&self) -> usize {
        self.rows
    }

    fn cols(&self) -> usize {
        self.cols
    }

    fn from_data<I>(rows: usize, data: I) -> Option<Self>
    where
        I: IntoIterator<Item = <Self::Field as Field>::Element>,
    {
        let data: Vec<_> = data.into_iter().collect();

        if !data.len().is_multiple_of(rows) {
            return None;
        }
        let cols = data.len().checked_div_euclid(rows)?;

        Some(Self { rows, cols, data })
    }

    fn from_data_unchecked<I>(rows: usize, data: I) -> Self
    where
        I: IntoIterator<Item = <Self::Field as Field>::Element>,
    {
        let data: Vec<_> = data.into_iter().collect();
        let cols = data.len() / rows;

        Self { rows, cols, data }
    }

    fn generate<GF>(rows: usize, cols: usize, mut generate: GF) -> Self
    where
        GF: FnMut(usize, usize) -> <Self::Field as Field>::Element,
    {
        let data = (0..cols)
            .flat_map(|x| (0..rows).map(move |y| (x, y)))
            .map(|(x, y)| generate(x, y))
            .collect();

        Self { rows, cols, data }
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// Matrix formatting ---------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------

impl<F: Field> std::fmt::Debug for DMat<F>
where
    F::Element: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DMat")
            .field("rows", &self.rows)
            .field("cols", &self.cols)
            .field("data", &self.data)
            .finish()
    }
}

impl<F: Field> std::fmt::Display for DMat<F>
where
    F::Element: std::fmt::Display,
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
