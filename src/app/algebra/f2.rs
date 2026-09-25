use rand::RngExt as _;

use crate::app::algebra::finite_field::FiniteField;

use super::{field::Field, randomizable::Randomizable};

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

    fn neg(a: Self::Element) -> Self::Element {
        a
    }

    fn recip(a: Self::Element) -> Option<Self::Element> {
        match a {
            Self::ZERO => None,
            non_zero_val => Some(non_zero_val),
        }
    }
}

impl FiniteField for F2 {
    const GENERATOR: Self::Element = F2Element::ONE;
    const GENERATOR_INVERSE: Self::Element = F2Element::ONE;

    fn non_zero_elements() -> impl ExactSizeIterator<Item = Self::Element> {
        [F2Element::ONE].into_iter()
    }

    fn elements() -> impl ExactSizeIterator<Item = Self::Element> {
        [F2Element::ZERO, F2Element::ONE].into_iter()
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

impl From<F2Element> for u8 {
    fn from(element: F2Element) -> Self {
        element.value as u8
    }
}

impl std::str::FromStr for F2Element {
    type Err = String;

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
    pub const ZERO: Self = Self { value: false };
    pub const ONE: Self = Self { value: true };

    pub fn flip(&mut self) {
        self.value = !self.value
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
