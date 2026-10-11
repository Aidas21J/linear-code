use crate::app::{
    BaseApp,
    algebra::error_count,
    core::{LinearCode, MemorylessChannel, VecCodec},
    log_activity,
};

use rayon::prelude::*;

/// Represents an app that works with byte buffers.
pub trait BufferApp: BaseApp + Sync
where
    <Self::Code as LinearCode>::RowVector: Sync + Send,
{
    type VecCodec: VecCodec<<Self::Code as LinearCode>::RowVector>;

    /// Returns a reference to the vector codec.
    fn vec_codec(&self) -> &Self::VecCodec;

    /// Sends vectors through app's channel without using error-correcting code.
    ///
    /// Takes vector buffer `input`.
    ///
    /// Returns `input` that was just sent through the app's channel.
    fn send_vecs_without_coding(
        &self,
        input: &Vec<<Self::Code as LinearCode>::RowVector>,
    ) -> Vec<<Self::Code as LinearCode>::RowVector> {
        input
            .par_iter()
            .cloned()
            .map_init(|| rand::rng(), |rng, vec| self.channel().send_vec(vec, rng))
            .collect()
    }

    /// Sends vectors through app's channel using app's error-correcting code.
    ///
    /// Takes vector buffer `input`.
    ///
    /// Returns `input` that was:
    ///     1) encoded using app's code,
    ///     2) sent through app's channel,
    ///     3) decoded using the same code.
    fn send_vecs_with_coding(
        &self,
        input: &Vec<<Self::Code as LinearCode>::RowVector>,
    ) -> Vec<<Self::Code as LinearCode>::RowVector> {
        input
            .par_iter()
            .map(|vec| self.code().encode(vec))
            .map_init(|| rand::rng(), |rng, vec| self.channel().send_vec(vec, rng))
            .map(|vec| self.code().decode(vec))
            .collect()
    }

    /// Abstract function that does the cycle of:
    ///     1) Parsing received bytes to vectors.
    ///     2) Sending those bytes (with or without coding) through the app's channel.
    ///     3) Counting number of errors that have occurred.
    ///     4) Parsing the output back to bytes.
    ///     5) Returning output bytes along with the error count.
    ///
    /// Takes byte buffer `bytes` and a `bool` representing whether to use the error-correcting code
    /// `code`.
    ///
    /// Returns output bytes and the error count.
    fn send_bytes(&self, bytes: Vec<u8>, code: bool) -> (Vec<u8>, usize) {
        let (input_vecs, pad_size) = log_activity!(
            "Parsing from bytes to vectors...",
            self.vec_codec().parse_vecs(bytes)
        );

        let output_vecs = if code {
            log_activity!(
                "Sending vectors through channel...",
                self.send_vecs_with_coding(&input_vecs)
            )
        } else {
            log_activity!(
                "Sending vectors through channel...",
                self.send_vecs_without_coding(&input_vecs)
            )
        };

        let error_count = log_activity!(
            "Counting transmission errors...",
            error_count(input_vecs.iter(), output_vecs.iter())
        );

        let output_bytes = log_activity!(
            "Parsing from vectors to bytes...",
            self.vec_codec().parse_bytes(output_vecs, pad_size)
        );

        (output_bytes, error_count)
    }
}
