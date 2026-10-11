use super::{DCVec, DynamicVector, Field, TransposableTo};

/// Dynamic row vector.
pub struct DRVec<F: Field> {
    /// Vector of [`DRVec`] elements.
    data: Vec<F::Element>,
}

impl<F: Field> DynamicVector for DRVec<F> {
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

    /// Create vector where exactly one element is non-zero and is equal to `value`.
    ///
    /// Takes `zeros_before`, `zeros_after` and `value`.
    ///
    /// Returns vector of the form (0, ..., 0, value, 0, ..., 0) with exactly `zeros_before` zeros
    /// to the left of `value` element and with exactly `zeros_after` zeros to the right.
    fn e(zeros_before: usize, zeros_after: usize, value: <Self::Field as Field>::Element) -> Self {
        let mut data = vec![F::ZERO; zeros_before + 1 + zeros_after];
        data[zeros_before] = value;

        Self { data }
    }
}

impl<F: Field> TransposableTo<DCVec<F>> for DRVec<F> {}

impl<F: Field> std::fmt::Debug for DRVec<F>
where
    F::Element: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DRVec").field("data", &self.data).finish()
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
            .join("");
        write!(f, "{self_str}")
    }
}

impl<F: Field> std::str::FromStr for DRVec<F> {
    type Err = <F::Element as std::str::FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_iter(
            s.chars()
                .map(|c| c.to_string())
                .map(|str| F::Element::from_str(str.as_str()))
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

impl<F: Field> Clone for DRVec<F> {
    fn clone(&self) -> Self {
        Self::from_iter(self.data.clone().into_iter())
    }
}
