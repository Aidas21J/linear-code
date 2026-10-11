use super::{Field, FiniteField, Randomizable};

/// Abstract dynamic (resizable at runtime) vector.
pub trait DynamicVector: Sized + Clone {
    type Field: Field;

    /// Iterate vector elements.
    fn iter<'a>(&'a self) -> impl ExactSizeIterator<Item = &'a <Self::Field as Field>::Element>;
    /// Iterate (and mutate) vector elements.
    fn iter_mut<'a>(
        &'a mut self,
    ) -> impl ExactSizeIterator<Item = &'a mut <Self::Field as Field>::Element>;
    /// Iterate and consume vector elements.
    fn into_iter(self) -> impl ExactSizeIterator<Item = <Self::Field as Field>::Element>;

    /// Get the dimension of vector.
    fn dim(&self) -> usize {
        self.iter().len()
    }

    /// Create a vector from an iterator.
    fn from_iter<I>(data: I) -> Self
    where
        I: IntoIterator<Item = <Self::Field as Field>::Element>;

    /// Create a zero vector of a certain dimension.
    ///
    /// Takes dimension `dim`.
    ///
    /// Returns the zero vector.
    fn zeros(dim: usize) -> Self {
        Self::from_iter(std::iter::repeat_n(Self::Field::ZERO, dim).into_iter())
    }

    /// Create a vector where exactly one element is non-zero and is equal to `value`.
    ///
    /// Takes `zeros_before`, `zeros_after` and `value`.
    ///
    /// Returns a vector with exactly `zeros_before` zeros, followed by element equal to `value`,
    /// followed by exactly `zeros_after` zeros.
    fn e(zeros_before: usize, zeros_after: usize, value: <Self::Field as Field>::Element) -> Self {
        Self::from_iter(
            std::iter::repeat_n(Self::Field::ZERO, zeros_before)
                .chain(std::iter::once(value))
                .chain(std::iter::repeat_n(Self::Field::ZERO, zeros_after).into_iter()),
        )
    }

    /// Generates a uniformly random vector.
    ///
    /// Takes dimension `dim` and reference to random number generator `rng`.
    ///
    /// Returns a vector with each element generated uniformly randomly from all possible values.
    fn generate_uniform<R>(dim: usize, rng: &mut R) -> Self
    where
        <Self::Field as Field>::Element: Randomizable,
        R: rand::Rng,
    {
        Self::from_iter((0..dim).map(|_| <Self::Field as Field>::Element::generate_uniform(rng)))
    }

    /// Takes specified number of elements from a vector and drops the rest.
    ///
    /// Takes number elements to take `dim`.
    ///
    /// Returns a vector with at most `dim` columns. If `dim` is greater than the dimension of
    /// `self`, `self` is returned.
    fn take(self, dim: usize) -> Self {
        Self::from_iter(self.into_iter().take(dim))
    }

    /// Calculates the weight of a vector.
    ///
    /// Returns the weight, i.e. number of non-zero elements, of `self`.
    fn weight(&self) -> usize {
        self.iter().filter(|&x| *x != Self::Field::ZERO).count()
    }

    /// Calculates the index of a vector.
    ///
    /// Returns the (lexicographic ordering) index of `self`.
    fn index(&self) -> usize
    where
        Self::Field: FiniteField,
        usize: From<<Self::Field as Field>::Element>,
    {
        self.iter().cloned().fold(0usize, |acc, element| {
            Self::Field::ORDER * acc + usize::from(element)
        })
    }

    /// Calculates the hamming distance to another vector.
    ///
    /// Takes a vector `other`.
    ///
    /// Returns the `self`'s hamming distance to `other`.
    fn hamming_distance_to(&self, other: &Self) -> usize {
        debug_assert_eq!(
            self.dim(),
            other.dim(),
            "cannot calculate hamming distance between different dimention vectors"
        );

        self.iter()
            .zip(other.iter())
            .filter(|(a_i, b_i)| a_i != b_i)
            .count()
    }

    /// Consumes the vector and adds it to another one.
    ///
    /// Takes a vector `other`.
    ///
    /// Returns `self` + `other`.
    fn into_add(self, other: &Self) -> Self {
        debug_assert_eq!(
            self.dim(),
            other.dim(),
            "cannot add vectors with different dimentions"
        );

        Self::from_iter(
            self.into_iter()
                .zip(other.iter())
                .map(|(x, y)| Self::Field::into_add(x, y)),
        )
    }
}

/// Calculates number of differing bits between two iterators of vectors.
///
/// Takes two iterators of vectors: `sent` and `received`. If `sent` and `received` have different
/// number of vectors, the vectors that don't have a corresponding one in the other iterator will be
/// ignored.
///
/// Returns the total number of differing bits (error count) of `sent` and `received`.
pub fn error_count<'a, F, V, IS, IR>(sent: IS, received: IR) -> usize
where
    F: Field + 'a,
    V: DynamicVector<Field = F> + 'a,
    IS: Iterator<Item = &'a V>,
    IR: Iterator<Item = &'a V>,
{
    sent.zip(received)
        .map(|(a, b)| a.hamming_distance_to(b))
        .sum()
}
