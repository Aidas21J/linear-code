use rand::Rng;

use crate::app::algebra::{drvec::DRVec, finite_field::FiniteField, randomizable::Randomizable};

pub trait Channel<F: FiniteField>
where
    F::Element: Randomizable,
{
    fn send_mut(&self, value: &mut F::Element, rng: &mut impl Rng);

    fn send_drvec(&self, mut values: DRVec<F>, rng: &mut impl Rng) -> DRVec<F> {
        values
            .iter_mut()
            .for_each(|value| self.send_mut(value, rng));
        values
    }
}
