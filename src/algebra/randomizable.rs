use rand::Rng;

use crate::algebra::underlying_set::UnderlyingSet;

pub trait Randomizable: UnderlyingSet {
    fn generate_uniform(rng: &mut impl Rng) -> Self;
}
