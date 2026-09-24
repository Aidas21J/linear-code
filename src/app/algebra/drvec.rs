use super::{dcvec::DCVec, dmat::DMat, field::Field, finite_field::FiniteField};

pub struct DRVec<F: Field> {
    data: Vec<F::Element>,
}

// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// Row vector getters --------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------

impl<F: Field> DRVec<F> {
    pub fn cols(&self) -> usize {
        self.data.len()
    }

    pub fn into_data(self) -> Vec<F::Element> {
        self.data
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// Row vector creation -------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------

impl<F: Field> From<Vec<F::Element>> for DRVec<F> {
    fn from(row: Vec<F::Element>) -> Self {
        Self { data: row }
    }
}

impl<F: Field> DRVec<F> {
    pub fn from_row_slice(row: &[F::Element]) -> Self {
        row.to_vec().into()
    }

    pub fn zeros(cols: usize) -> Self {
        Self {
            data: vec![F::ZERO; cols],
        }
    }

    pub fn e(zeros_before: usize, zeros_after: usize) -> Self {
        let mut row = vec![F::ZERO; zeros_before + 1 + zeros_after];
        row[zeros_before] = F::ONE;

        row.into()
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// Row vector iteration ------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------

impl<F: Field> DRVec<F> {
    pub fn iter(&self) -> impl Iterator<Item = &F::Element> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut F::Element> {
        self.data.iter_mut()
    }
}

pub struct DRVecIterator<F: FiniteField> {
    field_generator: F::Element,
    field_generator_inverse: F::Element,
    next_state: Option<DRVec<F>>,
}

impl<F: FiniteField> DRVecIterator<F> {
    pub fn new(cols: usize, field_generator: F::Element) -> Option<Self> {
        let field_generator_inverse = F::recip(field_generator.clone())?;
        Some(Self {
            field_generator,
            field_generator_inverse,
            next_state: DRVec::zeros(cols).into(),
        })
    }
}

impl<F: FiniteField> Iterator for DRVecIterator<F> {
    type Item = DRVec<F>;

    fn next(&mut self) -> Option<Self::Item> {
        let current_state = self.next_state.clone()?;
        let mut next_state = current_state.clone();

        enum IncrementingState {
            NotIncremented,
            Carried,
            Incremented,
        }

        let go_to_next_val = |val: &mut F::Element| {
            if *val == F::ZERO {
                *val = F::ONE
            } else if *val == self.field_generator_inverse {
                *val = F::ZERO
            } else {
                *val = F::mul(val, &self.field_generator)
            }
        };

        let mut increment_state = IncrementingState::NotIncremented;
        for val in next_state.data.iter_mut() {
            match increment_state {
                IncrementingState::NotIncremented | IncrementingState::Carried => {
                    go_to_next_val(val);
                    if *val == F::ZERO {
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

// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// Row vector manipulation ---------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------

impl<F: Field> DRVec<F> {
    pub fn split_off(mut self, cols: usize) -> Self {
        self.data.split_off(cols).into()
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// Row vector unary operations -----------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------

impl<F: Field> DRVec<F> {
    pub fn transpose(&self) -> DCVec<F> {
        self.data.clone().into()
    }

    pub fn into_transpose(self) -> DCVec<F> {
        self.data.into()
    }

    pub fn weight(&self) -> usize {
        self.data.iter().filter(|&x| *x != F::ZERO).count()
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// Row vector binary operations ----------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------

impl<F: Field> DRVec<F> {
    pub fn hamming_distance(a: &Self, b: &Self) -> usize {
        debug_assert_eq!(
            a.data.len(),
            b.data.len(),
            "cannot calculate hamming distance between different sized vectors"
        );

        a.data
            .iter()
            .zip(b.data.iter())
            .filter(|(a_i, b_i)| a_i != b_i)
            .count()
    }

    pub fn into_scalar_mul(s: &F::Element, v: Self) -> Self {
        v.data
            .into_iter()
            .map(|element| F::mul(s, &element))
            .collect::<Vec<_>>()
            .into()
    }
}

impl<F: Field> std::ops::Add<&DRVec<F>> for &DRVec<F> {
    type Output = DRVec<F>;

    fn add(self, rhs: &DRVec<F>) -> Self::Output {
        debug_assert_eq!(
            self.cols(),
            rhs.cols(),
            "cannot add row vectors with different number of cols"
        );

        self.data
            .iter()
            .zip(rhs.data.iter())
            .map(|(x, y)| F::add(x, y))
            .collect::<Vec<_>>()
            .into()
    }
}

impl<F: Field> std::ops::Add<&DRVec<F>> for DRVec<F> {
    type Output = DRVec<F>;

    fn add(self, rhs: &DRVec<F>) -> Self::Output {
        debug_assert_eq!(
            self.cols(),
            rhs.cols(),
            "cannot add row vectors with different number of cols"
        );

        self.data
            .into_iter()
            .zip(rhs.data.iter())
            .map(|(x, y)| F::add(&x, y))
            .collect::<Vec<_>>()
            .into()
    }
}

impl<F: Field> std::ops::Add<DRVec<F>> for &DRVec<F> {
    type Output = DRVec<F>;

    fn add(self, rhs: DRVec<F>) -> Self::Output {
        debug_assert_eq!(
            self.cols(),
            rhs.cols(),
            "cannot add row vectors with different number of cols"
        );

        rhs.data
            .into_iter()
            .zip(self.data.iter())
            .map(|(y, x)| F::add(x, &y))
            .collect::<Vec<_>>()
            .into()
    }
}

impl<F: Field> std::ops::Add<DRVec<F>> for DRVec<F> {
    type Output = DRVec<F>;

    fn add(self, rhs: DRVec<F>) -> Self::Output {
        self + &rhs
    }
}

impl<F: Field> std::ops::Mul<&DMat<F>> for &DRVec<F> {
    type Output = DRVec<F>;

    fn mul(self, rhs: &DMat<F>) -> Self::Output {
        debug_assert_eq!(
            self.cols(),
            rhs.rows(),
            "matrix dimentions do not match for multiplication"
        );

        let mut res = DRVec::zeros(rhs.cols());
        let b_rows = rhs.rows_iter();

        for (a_val, b_row) in self.data.iter().zip(b_rows) {
            for (res_val, b_val) in res.data.iter_mut().zip(b_row.iter()) {
                *res_val = F::add(res_val, &F::mul(a_val, b_val));
            }
        }

        res
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// Row vector formatting -----------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------

impl<F: Field> std::fmt::Debug for DRVec<F>
where
    F::Element: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DRVec").field("data", &self.data).finish()
    }
}

impl<F: Field> std::fmt::Display for DRVec<F>
where
    F::Element: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let self_str: String = self
            .data
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join("");
        write!(f, "{self_str}")
    }
}

impl<F: Field> std::str::FromStr for DRVec<F> {
    type Err = <F::Element as std::str::FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let row = s
            .chars()
            .map(|c| c.to_string())
            .map(|str| F::Element::from_str(str.as_str()))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(row.into())
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// Row vector equality -------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------

impl<F: Field> PartialEq for DRVec<F> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<F: Field> Eq for DRVec<F> {}

// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// Row vector hash -----------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------

impl<F: Field> std::hash::Hash for DRVec<F> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.data.hash(state);
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// Row vector cloning --------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------

impl<F: Field> Clone for DRVec<F> {
    fn clone(&self) -> Self {
        self.data.clone().into()
    }
}
