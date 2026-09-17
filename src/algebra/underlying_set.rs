use std::fmt::Debug;

pub trait UnderlyingSet: 'static + Clone + PartialEq + Eq + Debug {}

impl<T> UnderlyingSet for T where T: 'static + Clone + PartialEq + Eq + Debug {}
