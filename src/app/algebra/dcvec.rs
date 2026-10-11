use super::{DRVec, DynamicVector, Field, TransposableTo};

/// Dynamic column vector.
pub struct DCVec<F: Field> {
    /// Vector of [`DCVec`] elements.
    data: Vec<F::Element>,
}

impl<F: Field> DynamicVector for DCVec<F> {
    type Field = F;

    /// Iterate vector elements.
    ///
    /// Returns immutable iterator over vector data.
    fn iter(&self) -> impl ExactSizeIterator<Item = &<Self::Field as Field>::Element> {
        self.data.iter()
    }

    /// Iterate (and mutate) vector elements.
    ///
    /// Returns mutable iterator over vector data.
    fn iter_mut(&mut self) -> impl ExactSizeIterator<Item = &mut <Self::Field as Field>::Element> {
        self.data.iter_mut()
    }

    /// Iterate and consume vector elements.
    ///
    /// Returns into iterator over vector data.
    fn into_iter(self) -> impl ExactSizeIterator<Item = <Self::Field as Field>::Element> {
        self.data.into_iter()
    }

    /// Create vector from an iterator of elements.
    ///
    /// Takes an element iterator `data`.
    ///
    /// Returns a vector with those elements.
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

impl<F: Field> Clone for DCVec<F> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}
