use super::{Field, FiniteField, FiniteUnderlyingSet, Randomizable};

/// Type representing GF(2).
pub enum F2 {}

impl Field for F2 {
    /// Set GF(2) element representing type as [`F2Element`].
    type Element = F2Element;

    /// Add two GF(2) elements.
    ///
    /// Takes `a` and `b` from GF(2).
    ///
    /// Returns `a` + `b`.
    fn add(a: &Self::Element, b: &Self::Element) -> Self::Element {
        Self::Element {
            value: a.value ^ b.value,
        }
    }

    /// Add two GF(2) elements.
    ///
    /// Takes `a` and `b` from GF(2).
    ///
    /// Returns `a` + `b`, consumes `a`.
    fn into_add(a: Self::Element, b: &Self::Element) -> Self::Element {
        Self::Element {
            value: a.value ^ b.value,
        }
    }

    /// Multiply two GF(2) elements.
    ///
    /// Takes `a` and `b` from GF(2).
    ///
    /// Returns `a` * `b`.
    fn mul(a: &Self::Element, b: &Self::Element) -> Self::Element {
        Self::Element {
            value: a.value & b.value,
        }
    }

    /// Multiply two GF(2) elements.
    ///
    /// Takes `a` and `b` from GF(2).
    ///
    /// Returns `a` * `b`, consumes `a`.
    fn into_mul(a: Self::Element, b: &Self::Element) -> Self::Element {
        Self::Element {
            value: a.value & b.value,
        }
    }

    /// Defines GF(2) additive identity value.
    const ZERO: Self::Element = Self::Element::ZERO;
    /// Defines GF(2) multiplicative identity value.
    const ONE: Self::Element = Self::Element::ONE;

    /// Negate GF(2) element.
    ///
    /// Takes and consumes `a` from GF(2).
    ///
    /// Returns the additive inverse of `a`, i.e. -`a`.
    fn neg(a: Self::Element) -> Self::Element {
        a
    }

    /// Calculate reciprocal of GF(2) element.
    ///
    /// Takes and consumes `a` from GF(2).
    ///
    /// Returns the multiplicative inverse of `a`, if `a` != 0. If `a` == 0, then `None` is
    /// returned.
    fn recip(a: Self::Element) -> Option<Self::Element> {
        match a {
            Self::ZERO => None,
            non_zero_val => Some(non_zero_val),
        }
    }
}

impl FiniteField for F2 {
    /// Set order of GF(2).
    const ORDER: usize = 2;

    /// Define the generator (primitive element) of GF(2).
    const GENERATOR: Self::Element = F2Element::ONE;
    /// Define the generator's multiplicative inverse.
    const GENERATOR_INVERSE: Self::Element = F2Element::ONE;

    /// Return an iterator over non-zero GF(2) elements.
    fn non_zero_elements() -> impl ExactSizeIterator<Item = Self::Element> {
        [F2Element::ONE].into_iter()
    }
}

/// Representation of GF(2) element.
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub struct F2Element {
    value: bool,
}

impl From<bool> for F2Element {
    /// Create GF(2) element from `bool`.
    fn from(value: bool) -> Self {
        F2Element { value }
    }
}

impl From<F2Element> for u8 {
    /// Create GF(2) element from `u8`.
    fn from(element: F2Element) -> Self {
        element.value as u8
    }
}

impl From<F2Element> for usize {
    /// Create GF(2) element from `usize`.
    fn from(element: F2Element) -> Self {
        element.value as usize
    }
}

impl std::str::FromStr for F2Element {
    type Err = String;

    /// Create GF(2) element from string.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Self::ZERO),
            "1" => Ok(Self::ONE),
            _ => Err(format!(
                "cannot parse F2 element: {s}. Only valid values are 0 and 1"
            )),
        }
    }
}

impl F2Element {
    /// Define GF(2) element 0 value.
    pub const ZERO: Self = Self { value: false };
    /// Define GF(2) element 1 value.
    pub const ONE: Self = Self { value: true };

    /// Flip the value of GF(2) element under given condition. "Flip" means mapping 0 to 1 and 1 to
    /// 0.
    ///
    /// Takes a `bool` `flip`.
    ///
    /// Returns `self`, if `flip` is `false`, and flipped value of `self`, if `flip` is `true`.
    pub const fn conditionally_flipped(self, flip: bool) -> Self {
        Self {
            value: self.value ^ flip,
        }
    }
}

impl FiniteUnderlyingSet for F2Element {}

impl Randomizable for F2Element {
    /// Generates GF(2) element with equal probability of 0 and 1.
    fn generate_uniform<R>(rng: &mut R) -> Self
    where
        R: rand::prelude::Rng + rand::RngExt,
    {
        F2Element {
            value: rng.random(),
        }
    }
}

impl std::fmt::Display for F2Element {
    /// Format a GF(2) element.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.value {
            false => write!(f, "0"),
            true => write!(f, "1"),
        }
    }
}
