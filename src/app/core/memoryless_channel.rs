use crate::app::algebra::{dynamic_vector::DynamicVector, field::Field};

pub trait MemorylessChannel<T> {
    fn send<R: rand::Rng>(&self, value: T, rng: &mut R) -> T;

    fn send_vec<F, V, R>(&self, vec: V, rng: &mut R) -> V
    where
        F: Field<Element = T>,
        V: DynamicVector<Field = F>,
        R: rand::Rng,
    {
        V::from_iter(vec.into_iter().map(|vec| self.send(vec, rng)))
    }
}
