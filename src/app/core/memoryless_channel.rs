use crate::app::algebra::{DynamicVector, Field};

/// Representation of a memoryless channel where input and output sets are the same.
pub trait MemorylessChannel<T> {
    /// Send an element over the channel.
    ///
    /// Takes `value` to send and random number generator `rng`.
    ///
    /// Returns received value.
    fn send<R: rand::Rng>(&self, value: T, rng: &mut R) -> T;

    /// Send an vector of elements over the channel.
    ///
    /// Takes vector `vec` and random number generator `rng`.
    ///
    /// Returns vector with the received values.
    fn send_vec<F, V, R>(&self, vec: V, rng: &mut R) -> V
    where
        F: Field<Element = T>,
        V: DynamicVector<Field = F>,
        R: rand::Rng,
    {
        V::from_iter(vec.into_iter().map(|vec| self.send(vec, rng)))
    }
}
