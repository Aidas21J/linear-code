use super::UnderlyingSet;

pub trait Randomizable: UnderlyingSet {
    fn generate_uniform<R: rand::Rng>(rng: &mut R) -> Self;
}
