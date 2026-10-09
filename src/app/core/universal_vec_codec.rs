use crate::app::algebra::{DynamicVector, F2, F2Element, Field};

use super::VecCodec;

pub struct UniversalVecCodec<V: DynamicVector> {
    vec_dim: usize,
    _vector: std::marker::PhantomData<V>,
}

impl<V: DynamicVector> UniversalVecCodec<V> {
    pub const fn new(vec_dim: usize) -> Self {
        Self {
            vec_dim,
            _vector: std::marker::PhantomData,
        }
    }
}

impl<V: DynamicVector<Field = F2>> VecCodec<V> for UniversalVecCodec<V> {
    fn parse_vecs(&self, bytes: Vec<u8>) -> (Vec<V>, usize) {
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

            vecs.push(V::from_iter(chunk));
        }

        (vecs, pad_size)
    }

    fn parse_bytes(&self, vecs: Vec<V>, pad_size: usize) -> Vec<u8> {
        const BITS_PER_BYTE: usize = 8;
        let bits_to_take = (vecs.len().saturating_mul(self.vec_dim)).saturating_sub(pad_size);
        let mut bytes = Vec::with_capacity(bits_to_take / BITS_PER_BYTE);

        let mut bit_iter = vecs
            .into_iter()
            .flat_map(|v| v.into_iter())
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
