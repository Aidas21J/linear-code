use super::{DynamicVector, Field, FiniteField};

/// Iterator that goes through every possible value of a [`DynamicVector`] over a [`FiniteField`]
/// `F`.
pub struct DynamicVectorValueIterator<V: DynamicVector<Field: FiniteField>> {
    /// `DynamicVector` to go to next.
    next_state: Option<V>,
}

impl<V: DynamicVector<Field: FiniteField>> DynamicVectorValueIterator<V> {
    /// Create new vector value iterator for vectors of dimension `dim`. Iterator starts from zero
    /// vector.
    pub fn new(dim: usize) -> Self {
        Self {
            next_state: V::zeros(dim).into(),
        }
    }
}

impl<V: DynamicVector<Field: FiniteField>> Iterator for DynamicVectorValueIterator<V> {
    /// Set iterator item type to the vector type.
    type Item = V;

    /// Go to next iteration state of [`DynamicVectorValueIterator`].
    ///
    /// Let the iterator [`DynamicVector`] type be `V`.
    /// Let `V` be a vector over a [`FiniteField`] `F`.
    /// Let the order of `F` be `q`.
    ///
    /// Currently, the iterator is going through all values in reverse lexicographic order, i.e. it
    /// starts with zero vector then goes to vector with one at the start and zeros in all other
    /// places, etc.
    ///
    /// Elements in the iterator cycle like this:
    ///     0 -> 1 -> alpha -> alpha^2 -> ... -> alpha^(`q` - 2) -> 0
    /// where alpha is the generator (primitive element) of `F`.
    ///
    /// # Example
    ///
    /// If `V` is a two-dimensional binary row vector, the iterator will go through vectors in this
    /// order:
    ///     1) (0 0)
    ///     2) (1 0)
    ///     3) (0 1)
    ///     4) (1 1)
    ///
    /// If `V` is a two-dimensional row vector over GF(8), the iterator will go through vectors in
    /// this order:
    ///     1)  (0       0)
    ///     2)  (1       0)
    ///     3)  (alpha   0)
    ///     4)  (alpha^2 0)
    ///     ...
    ///     8)  (alpha^6 0)
    ///     9)  (0       1)
    ///     10) (1       1)
    ///     11) (alpha   1)
    ///     12) (alpha^2 1)
    ///     ...
    ///     62) (alpha^4 alpha^6)
    ///     63) (alpha^5 alpha^6)
    ///     64) (alpha^6 alpha^6)
    /// where alpha is the generator of GF(8).
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
