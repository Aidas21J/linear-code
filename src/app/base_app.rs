use crate::app::{
    algebra::field::Field,
    core::{linear_code::LinearCode, memoryless_channel::MemorylessChannel, vec_codec::VecCodec},
};

pub trait BaseApp {
    type Code: LinearCode;
    type Channel: MemorylessChannel<<<Self::Code as LinearCode>::Field as Field>::Element>;
    type VecCodec: VecCodec<<Self::Code as LinearCode>::RowVector>;

    fn code(&self) -> &Self::Code;
    fn channel(&self) -> &Self::Channel;
    fn vec_codec(&self) -> &Self::VecCodec;
}
