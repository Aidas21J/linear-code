use rand::Rng;

use crate::algebra::{finite_field::FiniteField, randomizable::Randomizable};

pub trait Channel<F: FiniteField>
where
    F::Element: Randomizable,
{
    fn send(&self, value: F::Element, rng: &mut impl Rng) -> F::Element;
    #[expect(unused)]
    fn send_mut(&self, value: &mut F::Element, rng: &mut impl Rng);

    fn send_buffer(&self, values: &Vec<F::Element>, rng: &mut impl Rng) -> Vec<F::Element> {
        values
            .iter()
            .map(|value| self.send(value.clone(), rng))
            .collect()
    }

    #[expect(unused)]
    fn send_buffer_mut(&self, values: &mut Vec<F::Element>, rng: &mut impl Rng) {
        values
            .iter_mut()
            .for_each(|value| self.send_mut(value, rng))
    }
}
