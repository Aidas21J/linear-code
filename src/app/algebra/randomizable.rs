use super::underlying_set::UnderlyingSet;

pub trait Randomizable: UnderlyingSet {
    fn generate_uniform(rng: &mut impl rand::Rng) -> Self;
}
