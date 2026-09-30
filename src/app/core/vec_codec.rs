use crate::app::algebra::{drvec::DRVec, field::Field};

pub trait VecCodec<F: Field>: Sized {
    fn parse_vecs(&self, bytes: Vec<u8>) -> (Vec<DRVec<F>>, usize);
    fn parse_bytes(&self, vecs: Vec<DRVec<F>>, pad_size: usize) -> Vec<u8>;
}
