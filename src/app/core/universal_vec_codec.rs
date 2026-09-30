use std::marker::PhantomData;

use crate::app::{
    algebra::{
        drvec::DRVec,
        f2::{F2, F2Element},
        field::Field,
    },
    core::vec_codec::VecCodec,
};

pub struct UniversalVecCodec<F: Field> {
    vec_dim: usize,
    _field: PhantomData<F>,
}

impl<F: Field> UniversalVecCodec<F> {
    pub const fn new(vec_dim: usize) -> Self {
        Self {
            vec_dim,
            _field: PhantomData,
        }
    }
}

impl VecCodec<F2> for UniversalVecCodec<F2> {
    fn parse_vecs(&self, bytes: Vec<u8>) -> (Vec<DRVec<F2>>, usize) {
        const BITS_PER_BYTE: usize = 8;
        let chunk_size = self.vec_dim;
        let pad_size = {
            let a = bytes.len() % chunk_size;
            let b = BITS_PER_BYTE % chunk_size;
            let remainder = (a * b) % chunk_size;

            (chunk_size.saturating_sub(remainder)) % chunk_size
        };

        let mut vecs = Vec::with_capacity(bytes.len() + pad_size);

        let mut bits_iter = bytes
            .into_iter()
            .flat_map(|byte| {
                [
                    (0b1000_0000 & byte) != 0,
                    (0b0100_0000 & byte) != 0,
                    (0b0010_0000 & byte) != 0,
                    (0b0001_0000 & byte) != 0,
                    (0b0000_1000 & byte) != 0,
                    (0b0000_0100 & byte) != 0,
                    (0b0000_0010 & byte) != 0,
                    (0b0000_0001 & byte) != 0,
                ]
            })
            .map(F2Element::from)
            .chain(std::iter::repeat_n(F2::ZERO, pad_size));

        loop {
            let mut chunk = bits_iter.by_ref().take(chunk_size).peekable();

            if chunk.peek().is_none() {
                break;
            }

            vecs.push(chunk.collect::<Vec<_>>().into());
        }

        (vecs, pad_size)
    }

    fn parse_bytes(&self, vecs: Vec<DRVec<F2>>, pad_size: usize) -> Vec<u8> {
        const BITS_PER_BYTE: usize = 8;
        let bits_to_take = (vecs.len().saturating_mul(self.vec_dim)).saturating_sub(pad_size);
        let mut bytes = Vec::with_capacity(bits_to_take / BITS_PER_BYTE);

        let mut bit_iter = vecs
            .into_iter()
            .flat_map(|v| v.into_data())
            .take(bits_to_take);

        loop {
            let chunk = bit_iter.by_ref().take(BITS_PER_BYTE);

            let (byte, bit_count) = chunk.fold((0u8, 0), |(byte, bit_count), bit| {
                ((byte << 1u8) | u8::from(bit), bit_count + 1)
            });

            if bit_count != BITS_PER_BYTE {
                break;
            }

            bytes.push(byte);
        }

        bytes
    }
}
