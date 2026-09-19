use crate::algebra::{field::Field, finite_underlying_set::FiniteUnderlyingSet};

pub trait FiniteField: Field
where
    Self::Element: FiniteUnderlyingSet,
{
    fn elements() -> impl ExactSizeIterator<Item = Self::Element> {
        Self::Element::iter()
    }

    fn non_zero_elements() -> impl Iterator<Item = Self::Element> {
        Self::elements().filter(|x| *x != Self::ZERO)
    }
}

impl<F: Field> FiniteField for F where F::Element: FiniteUnderlyingSet {}
