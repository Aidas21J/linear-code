use super::UnderlyingSet;

/// Type that represents a field.
pub trait Field {
    type Element: UnderlyingSet;

    /// Add two field elements.
    ///
    /// Takes `a` and `b` from the field.
    ///
    /// Returns `a` + `b`.
    #[allow(unused)]
    fn add(a: &Self::Element, b: &Self::Element) -> Self::Element;
    /// Add two field elements.
    ///
    /// Takes `a` and `b` from the field.
    ///
    /// Returns `a` + `b`, consumes `a`.
    #[allow(unused)]
    fn into_add(a: Self::Element, b: &Self::Element) -> Self::Element;

    /// Multiply two field elements.
    ///
    /// Takes `a` and `b` from the field.
    ///
    /// Returns `a` * `b`.
    #[allow(unused)]
    fn mul(a: &Self::Element, b: &Self::Element) -> Self::Element;
    /// Multiply two field elements.
    ///
    /// Takes `a` and `b` from the field.
    ///
    /// Returns `a` * `b`, consumes `a`.
    #[allow(unused)]
    fn into_mul(a: Self::Element, b: &Self::Element) -> Self::Element;

    /// Defines field additive identity value.
    #[allow(unused)]
    const ZERO: Self::Element;
    /// Defines field multiplicative identity value.
    #[allow(unused)]
    const ONE: Self::Element;

    /// Negate field element.
    ///
    /// Takes and consumes `a` from field.
    ///
    /// Returns the additive inverse of `a`, i.e. -`a`.
    #[allow(unused)]
    fn neg(a: Self::Element) -> Self::Element;
    /// Calculate reciprocal of field element.
    ///
    /// Takes and consumes `a` from field.
    ///
    /// Returns the multiplicative inverse of `a`, if `a` != 0. If `a` == 0, then `None` is
    /// returned.
    #[allow(unused)]
    fn recip(a: Self::Element) -> Option<Self::Element>;
}
