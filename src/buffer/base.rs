#[derive(Clone)]
pub enum ChannelBuffer<T> {
    Mono(T),
    Stereo((T, T)),
}

impl<T> ChannelBuffer<T>
where
    T: AsRef<[f32]>,
{
    pub fn from_mono(data: T) -> Self {
        ChannelBuffer::Mono(data)
    }

    pub fn from_stereo_deinterleaved(l: T, r: T) -> Self {
        ChannelBuffer::Stereo((l, r))
    }

    pub fn len(&self) -> usize {
        match self {
            ChannelBuffer::Mono(b) => b.as_ref().len(),
            ChannelBuffer::Stereo(b) => b.0.as_ref().len().min(b.1.as_ref().len()),
        }
    }

    pub fn size(&self) -> usize {
        match self {
            ChannelBuffer::Mono(b) => b.as_ref().len(),
            ChannelBuffer::Stereo(b) => b.0.as_ref().len() + b.1.as_ref().len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            ChannelBuffer::Mono(b) => b.as_ref().is_empty(),
            ChannelBuffer::Stereo(b) => b.1.as_ref().is_empty() && b.1.as_ref().is_empty(),
        }
    }

    pub fn is_stereo(&self) -> bool {
        !matches!(self, ChannelBuffer::Mono(_))
    }

    pub fn left(&self) -> &T {
        match self {
            ChannelBuffer::Mono(b) => b,
            ChannelBuffer::Stereo(b) => &b.0,
        }
    }

    pub fn right(&self) -> &T {
        match self {
            ChannelBuffer::Mono(b) => b,
            ChannelBuffer::Stereo(b) => &b.1,
        }
    }
}
