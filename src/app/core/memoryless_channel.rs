use crate::app::algebra::{drvec::DRVec, field::Field};

pub trait MemorylessChannel<T> {
    fn send<R: rand::Rng>(&self, value: T, rng: &mut R) -> T;

    fn send_vec<F, R>(&self, vec: DRVec<F>, rng: &mut R) -> DRVec<F>
    where
        F: Field<Element = T>,
        R: rand::Rng,
    {
        vec.into_data()
            .into_iter()
            .map(|vec| self.send(vec, rng))
            .collect::<Vec<_>>()
            .into()
    }
}
