use super::underlying_set::UnderlyingSet;

pub trait Field {
    type Element: UnderlyingSet;

    #[allow(unused)]
    fn add(a: &Self::Element, b: &Self::Element) -> Self::Element;
    #[allow(unused)]
    fn into_add(a: Self::Element, b: &Self::Element) -> Self::Element;

    #[allow(unused)]
    fn mul(a: &Self::Element, b: &Self::Element) -> Self::Element;
    #[allow(unused)]
    fn into_mul(a: Self::Element, b: &Self::Element) -> Self::Element;

    #[allow(unused)]
    const ZERO: Self::Element;
    #[allow(unused)]
    const ONE: Self::Element;

    #[allow(unused)]
    fn neg(a: Self::Element) -> Self::Element;
    #[allow(unused)]
    fn recip(a: Self::Element) -> Option<Self::Element>;
}
