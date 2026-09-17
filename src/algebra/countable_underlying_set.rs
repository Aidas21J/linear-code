use crate::algebra::underlying_set::UnderlyingSet;

pub trait CountableUnderlyingSet: UnderlyingSet {
    fn iter() -> impl Iterator<Item = Self>;

    fn iter_pairs() -> impl Iterator<Item = (Self, Self)> {
        Self::iter().flat_map(|a| Self::iter().map(move |b| (a.clone(), b)))
    }
}
