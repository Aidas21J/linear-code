use super::DynamicVector;

pub trait TransposableTo<V>: DynamicVector
where
    V: DynamicVector<Field = Self::Field>,
{
    fn transpose(&self) -> V {
        V::from_iter(self.clone().into_iter())
    }

    fn into_transpose(self) -> V {
        V::from_iter(self.into_iter())
    }
}
