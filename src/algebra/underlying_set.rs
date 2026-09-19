use std::{fmt::Debug, hash::Hash};

pub trait UnderlyingSet: 'static + Clone + PartialEq + Eq + Debug + Hash {}

impl<T> UnderlyingSet for T where T: 'static + Clone + PartialEq + Eq + Debug + Hash {}
