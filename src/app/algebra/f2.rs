use rand::RngExt as _;

use super::{field::Field, finite_underlying_set::FiniteUnderlyingSet, randomizable::Randomizable};

pub enum F2 {}

impl Field for F2 {
    type Element = F2Element;

    fn add(a: &Self::Element, b: &Self::Element) -> Self::Element {
        Self::Element {
            value: a.value ^ b.value,
        }
    }

    fn mul(a: &Self::Element, b: &Self::Element) -> Self::Element {
        Self::Element {
            value: a.value & b.value,
        }
    }

    const ZERO: Self::Element = Self::Element::ZERO;
    const ONE: Self::Element = Self::Element::ONE;

    fn neg(a: &Self::Element) -> Self::Element {
        a.clone()
    }

    fn recip(a: &Self::Element) -> Option<Self::Element> {
        match a.clone() {
            Self::ZERO => None,
            non_zero_val => Some(non_zero_val),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub struct F2Element {
    value: bool,
}

impl From<bool> for F2Element {
    fn from(value: bool) -> Self {
        F2Element { value }
    }
}

impl std::str::FromStr for F2Element {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Self::ZERO),
            "1" => Ok(Self::ONE),
            _ => Err(format!("cannot parse F2 element: {s}")),
        }
    }
}

impl F2Element {
    pub const ZERO: Self = Self { value: false };
    pub const ONE: Self = Self { value: true };

    pub fn flip(&mut self) {
        self.value = !self.value
    }
}

impl FiniteUnderlyingSet for F2Element {
    fn iter() -> impl ExactSizeIterator<Item = Self> {
        [Self::ZERO, Self::ONE].into_iter()
    }
}

impl Randomizable for F2Element {
    fn generate_uniform(rng: &mut impl rand::prelude::Rng) -> Self {
        F2Element {
            value: rng.random(),
        }
    }
}

impl std::fmt::Display for F2Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.value {
            false => write!(f, "0"),
            true => write!(f, "1"),
        }
    }
}
