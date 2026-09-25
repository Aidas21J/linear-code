use super::field::Field;

pub trait FiniteField: Field {
    const GENERATOR: Self::Element;
    const GENERATOR_INVERSE: Self::Element;

    fn non_zero_elements() -> impl ExactSizeIterator<Item = Self::Element>;
    #[expect(unused)]
    fn elements() -> impl ExactSizeIterator<Item = Self::Element>;
}
