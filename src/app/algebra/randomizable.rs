use super::UnderlyingSet;

/// Type that can be generated randomly.
pub trait Randomizable: UnderlyingSet {
    fn generate_uniform<R: rand::Rng>(rng: &mut R) -> Self;
}
