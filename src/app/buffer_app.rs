use crate::app::{
    BaseApp,
    algebra::error_count,
    core::{LinearCode, MemorylessChannel, VecCodec},
    log_activity,
};

use rayon::prelude::*;

pub trait BufferApp: BaseApp + Sync
where
    <Self::Code as LinearCode>::RowVector: Sync + Send,
{
    type VecCodec: VecCodec<<Self::Code as LinearCode>::RowVector>;

    fn vec_codec(&self) -> &Self::VecCodec;

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
