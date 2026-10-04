use crate::app::algebra::dynamic_vector::DynamicVector;

pub trait VecCodec<V: DynamicVector>: Sized {
    fn parse_vecs(&self, bytes: Vec<u8>) -> (Vec<V>, usize);
    fn parse_bytes(&self, vecs: Vec<V>, pad_size: usize) -> Vec<u8>;
}
