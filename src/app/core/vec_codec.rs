use crate::app::algebra::DynamicVector;

/// Object that parses between raw bytes and [`DynamicVector`] vectors.
pub trait VecCodec<V: DynamicVector>: Sized {
    /// Parse raw bytes to vectors.
    ///
    /// If bytes don't go evenly into vectors, last vector is padded with 0s. The number of padding
    /// bits used is tracked. It can later be used when parsing back to bytes.
    ///
    /// Takes raw bytes `bytes`.
    ///
    /// Returns tuple of parsed vectors and number of padding bits used.
    fn parse_vecs(&self, bytes: Vec<u8>) -> (Vec<V>, usize);
    /// Parse vectors to raw bytes.
    ///
    /// Takes vectors `vecs` and number of padding bits at the end `pad_size`.
    ///
    /// Returns raw bytes.
    fn parse_bytes(&self, vecs: Vec<V>, pad_size: usize) -> Vec<u8>;
}
