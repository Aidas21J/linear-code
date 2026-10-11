use super::DynamicVector;

/// Trait for transposing between [`DynamicVector`] types. Vector type that implements this trait
/// defines transposition to another `DynamicVector` type `V`.
pub trait TransposableTo<V>: DynamicVector
where
    V: DynamicVector<Field = Self::Field>,
{
    /// Transpose a clone of `self` to another vector.
    fn transpose(&self) -> V {
        V::from_iter(self.clone().into_iter())
    }

    /// Consume and transpose `self` to another vector.
    fn into_transpose(self) -> V {
        V::from_iter(self.into_iter())
    }
}
