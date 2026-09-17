use std::fmt::Display;

use crate::algebra::{countable_underlying_set::CountableUnderlyingSet, f2::F2, field::Field};

pub mod algebra;

fn main() {
    print_field::<F2>();
}

fn print_field<F: Field>()
where
    F::Element: CountableUnderlyingSet + Display,
{
    println!("Addition:");
    for (a, b) in F::Element::iter_pairs() {
        let res = F::add(&a, &b);
        println!("  {a} + {b} = {res}");
    }

    println!();
    println!("Multiplication:");
    for (a, b) in F::Element::iter_pairs() {
        let res = F::mul(&a, &b);
        println!("  {a} * {b} = {res}");
    }

    println!();
    println!("Negation:");
    for a in F::Element::iter() {
        let res = F::neg(&a);
        println!("  -{a} = {res}");
    }

    println!();
    println!("Reciprocal:");
    for a in F::Element::iter() {
        let res = F::recip(&a).map_or("undefined".to_string(), |r| format!("{r}"));
        println!("  {a}^(-1) = {res}");
    }
}
