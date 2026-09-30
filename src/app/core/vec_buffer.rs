use crate::app::{
    algebra::{drvec::DRVec, field::Field},
    core::{linear_code::LinearCode, memoryless_channel::MemorylessChannel},
};

pub trait VecBuffer<F: Field>: IntoIterator<Item = DRVec<F>> + Sized {
    fn send_through_channel<Channel>(self, channel: &Channel) -> impl Iterator<Item = DRVec<F>>
    where
        Channel: MemorylessChannel<F::Element>,
    {
        let mut rng = rand::rng();

        self.into_iter()
            .map(move |vec| channel.send_vec(vec, &mut rng))
    }

    fn send_through_channel_with_coding<Code, Channel>(
        self,
        channel: &Channel,
        code: &Code,
    ) -> impl Iterator<Item = DRVec<F>>
    where
        Channel: MemorylessChannel<F::Element>,
        Code: LinearCode<Field = F>,
    {
        self.into_iter()
            .map(|vec| code.encode(&vec))
            .send_through_channel(channel)
            .map(|vec| code.decode(vec))
    }
}

impl<I, F> VecBuffer<F> for I
where
    F: Field,
    I: IntoIterator<Item = DRVec<F>>,
{
}

pub fn error_count<'a, F, IS, IR>(sent: IS, received: IR) -> usize
where
    F: Field + 'a,
    IS: Iterator<Item = &'a DRVec<F>>,
    IR: Iterator<Item = &'a DRVec<F>>,
{
    sent.zip(received)
        .map(|(a, b)| DRVec::hamming_distance(a, b))
        .sum()
}
