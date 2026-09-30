use crate::app::algebra::finite_underlying_set::FiniteUnderlyingSet;

use super::field::Field;

pub trait FiniteField: Field<Element: FiniteUnderlyingSet> {
    const GENERATOR: Self::Element;
    const GENERATOR_INVERSE: Self::Element;

    fn non_zero_elements() -> impl ExactSizeIterator<Item = Self::Element>;
}
