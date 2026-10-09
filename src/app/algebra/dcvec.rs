use super::{DRVec, DynamicVector, Field, TransposableTo};

pub struct DCVec<F: Field> {
    data: Vec<F::Element>,
}

impl<F: Field> DynamicVector for DCVec<F> {
    type Field = F;

    fn iter(&self) -> impl ExactSizeIterator<Item = &<Self::Field as Field>::Element> {
        self.data.iter()
    }

    fn iter_mut(&mut self) -> impl ExactSizeIterator<Item = &mut <Self::Field as Field>::Element> {
        self.data.iter_mut()
    }

    fn into_iter(self) -> impl ExactSizeIterator<Item = <Self::Field as Field>::Element> {
        self.data.into_iter()
    }

    fn from_iter<I>(data: I) -> Self
    where
        I: IntoIterator<Item = <Self::Field as Field>::Element>,
    {
        Self {
            data: data.into_iter().collect(),
        }
    }
}

impl<F: Field> TransposableTo<DRVec<F>> for DCVec<F> {}

// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// Column vector cloning -----------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------

impl<F: Field> Clone for DCVec<F> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}
