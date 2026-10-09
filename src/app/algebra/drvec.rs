use super::{DCVec, DynamicVector, Field, TransposableTo};

pub struct DRVec<F: Field> {
    data: Vec<F::Element>,
}

impl<F: Field> DynamicVector for DRVec<F> {
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

    fn e(zeros_before: usize, zeros_after: usize, value: <Self::Field as Field>::Element) -> Self {
        let mut data = vec![F::ZERO; zeros_before + 1 + zeros_after];
        data[zeros_before] = value;

        Self { data }
    }
}

impl<F: Field> TransposableTo<DCVec<F>> for DRVec<F> {}

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

// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// Row vector cloning --------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------

impl<F: Field> Clone for DRVec<F> {
    fn clone(&self) -> Self {
        Self::from_iter(self.data.clone().into_iter())
    }
}
