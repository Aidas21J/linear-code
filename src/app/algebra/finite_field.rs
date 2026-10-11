use super::{Field, FiniteUnderlyingSet};

/// Type that represents a finite field.
pub trait FiniteField: Field<Element: FiniteUnderlyingSet> {
    /// Order of the finite field.
    const ORDER: usize;

    /// Generator (primitive element) of the finite field.
    const GENERATOR: Self::Element;
    /// Generator's multiplicative inverse.
    const GENERATOR_INVERSE: Self::Element;

    /// Return an iterator over non-zero finite field elements.
    fn non_zero_elements() -> impl ExactSizeIterator<Item = Self::Element>;
}
