use crate::app::algebra::{dynamic_vector::DynamicVector, field::Field, finite_field::FiniteField};

pub struct DynamicVectorValueIterator<V: DynamicVector<Field: FiniteField>> {
    next_state: Option<V>,
}

impl<V: DynamicVector<Field: FiniteField>> DynamicVectorValueIterator<V> {
    pub fn new(cols: usize) -> Self {
        Self {
            next_state: V::zeros(cols).into(),
        }
    }
}

impl<V: DynamicVector<Field: FiniteField>> Iterator for DynamicVectorValueIterator<V> {
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        let current_state = self.next_state.clone()?;
        let mut next_state = current_state.clone();

        enum IncrementingState {
            NotIncremented,
            Carried,
            Incremented,
        }

        let go_to_next_val = |val: &mut <V::Field as Field>::Element| {
            *val = if *val == V::Field::ZERO {
                V::Field::ONE
            } else if *val == V::Field::GENERATOR_INVERSE {
                V::Field::ZERO
            } else {
                V::Field::mul(val, &V::Field::GENERATOR)
            }
        };

        let mut increment_state = IncrementingState::NotIncremented;
        for val in next_state.iter_mut() {
            match increment_state {
                IncrementingState::NotIncremented | IncrementingState::Carried => {
                    go_to_next_val(val);
                    if *val == V::Field::ZERO {
                        increment_state = IncrementingState::Carried;
                    } else {
                        increment_state = IncrementingState::Incremented;
                    }
                }
                IncrementingState::Incremented => break,
            }
        }

        self.next_state = match increment_state {
            IncrementingState::Incremented => Some(next_state),
            IncrementingState::NotIncremented | IncrementingState::Carried => None,
        };

        Some(current_state)
    }
}
