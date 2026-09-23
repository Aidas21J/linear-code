use super::underlying_set::UnderlyingSet;

pub trait FiniteUnderlyingSet: UnderlyingSet {
    fn iter() -> impl ExactSizeIterator<Item = Self>;
}
