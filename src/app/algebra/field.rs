use super::underlying_set::UnderlyingSet;

pub trait Field {
    type Element: UnderlyingSet;

    fn add(a: &Self::Element, b: &Self::Element) -> Self::Element;
    fn mul(a: &Self::Element, b: &Self::Element) -> Self::Element;

    const ZERO: Self::Element;
    const ONE: Self::Element;

    fn neg(a: Self::Element) -> Self::Element;
    fn recip(a: Self::Element) -> Option<Self::Element>;
}
