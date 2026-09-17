use std::fmt::Display;

use crate::algebra::{countable_underlying_set::CountableUnderlyingSet, field::Field};

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

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct F2Element {
    value: bool,
}

impl F2Element {
    const ZERO: Self = Self { value: false };
    const ONE: Self = Self { value: true };
}

impl CountableUnderlyingSet for F2Element {
    fn iter() -> impl Iterator<Item = Self> {
        [Self::ZERO, Self::ONE].into_iter()
    }
}

impl Display for F2Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.value {
            false => write!(f, "0"),
            true => write!(f, "1"),
        }
    }
}
