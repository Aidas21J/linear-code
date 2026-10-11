use std::{fmt::Debug, hash::Hash, str::FromStr};

/// Trait for a type that represents an underlying set.
pub trait UnderlyingSet: 'static + Clone + PartialEq + Eq + Debug + Hash + FromStr {}

impl<T> UnderlyingSet for T where T: 'static + Clone + PartialEq + Eq + Debug + Hash + FromStr {}
