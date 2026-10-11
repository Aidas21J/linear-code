use crate::app::algebra::{DynamicVector, F2, F2Element, Field};

use super::VecCodec;

/// Universal dynamic vector codec.
pub struct UniversalVecCodec<V: DynamicVector> {
    vec_dim: usize,
    _vector: std::marker::PhantomData<V>,
}

impl<V: DynamicVector> UniversalVecCodec<V> {
    /// Construct codec from vector dimension.
    pub const fn new(vec_dim: usize) -> Self {
        Self {
            vec_dim,
            _vector: std::marker::PhantomData,
        }
    }
}

impl<V: DynamicVector<Field = F2>> VecCodec<V> for UniversalVecCodec<V> {
    /// Parse raw bytes to vectors over GF(2).
    ///
    /// If bytes don't go evenly into vectors, last vector is padded with 0s. The number of padding
    /// bits used is tracked. It can later be used when parsing back to bytes.
    ///
    /// Takes raw bytes `bytes`.
    ///
    /// Returns tuple of parsed vectors and number of padding bits used.
    ///
    /// # Algorithm
    ///
    /// Padding size is calculated depending on how many bytes there are and the vector dimension.
    ///
    /// Each byte is decomposed into slice of bits. All bits are concatenated to a single iterator
    /// and turned into corresponding GF(2) element (0 bit to 0 in GF(2) and 1 bit to 1 in GF(2)).
    /// `pad_size` of GF(2) 0s are added to the end. The resulting iterator is `bits_iter`.
    ///
    /// While `bits_iter` has elements, chunks of GF(2) elements are taken and vectors are formed.
    /// Each vector is pushed to `vecs` which (with padding size) is returned when there is nothing
    /// left to parse.
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

    /// Parse vectors over GF(2) to raw bytes.
    ///
    /// Takes vectors `vecs` and number of padding bits at the end `pad_size`.
    ///
    /// Returns raw bytes.
    ///
    /// # Algorithm
    ///
    /// Received vectors are turned into iterator of bits `bit_iter`: each vector is turned into
    /// multiple bits (mapping 0 of GF(2) to 0 bit and 1 of GF(2) to 1 bit), all of these bit
    /// vectors are concatenated into one iterator and padding bits from the end are removed.
    ///
    /// While `bit_iter` still has bits to parse, chunks of `BITS_PER_BYTE` size are taken and
    /// turned into byte (`u8`) value. That byte value is pushed into `bytes` containing parsed
    /// bytes.
    ///
    /// When there are no more bytes to parse, the `bytes` is returned.
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
