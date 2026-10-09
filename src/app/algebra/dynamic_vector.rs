use crate::app::algebra::{field::Field, finite_field::FiniteField, randomizable::Randomizable};

pub trait DynamicVector: Sized + Clone {
    type Field: Field;

    fn iter<'a>(&'a self) -> impl ExactSizeIterator<Item = &'a <Self::Field as Field>::Element>;
    fn iter_mut<'a>(
        &'a mut self,
    ) -> impl ExactSizeIterator<Item = &'a mut <Self::Field as Field>::Element>;
    fn into_iter(self) -> impl ExactSizeIterator<Item = <Self::Field as Field>::Element>;

    fn dim(&self) -> usize {
        self.iter().len()
    }

    fn from_iter<I>(data: I) -> Self
    where
        I: IntoIterator<Item = <Self::Field as Field>::Element>;

    fn zeros(dim: usize) -> Self {
        Self::from_iter(std::iter::repeat_n(Self::Field::ZERO, dim).into_iter())
    }

    fn e(zeros_before: usize, zeros_after: usize, value: <Self::Field as Field>::Element) -> Self {
        Self::from_iter(
            std::iter::repeat_n(Self::Field::ZERO, zeros_before)
                .chain(std::iter::once(value))
                .chain(std::iter::repeat_n(Self::Field::ZERO, zeros_after).into_iter()),
        )
    }

    fn generate_uniform<R>(dim: usize, rng: &mut R) -> Self
    where
        <Self::Field as Field>::Element: Randomizable,
        R: rand::Rng,
    {
        Self::from_iter((0..dim).map(|_| <Self::Field as Field>::Element::generate_uniform(rng)))
    }

    fn take(self, cols: usize) -> Self {
        Self::from_iter(self.into_iter().take(cols))
    }

    fn weight(&self) -> usize {
        self.iter().filter(|&x| *x != Self::Field::ZERO).count()
    }

    fn index(&self) -> usize
    where
        Self::Field: FiniteField,
        usize: From<<Self::Field as Field>::Element>,
    {
        self.iter().cloned().fold(0usize, |acc, element| {
            Self::Field::ORDER * acc + usize::from(element)
        })
    }

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
