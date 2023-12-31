use crate::dsp::interleave::*;
use core::{mem::MaybeUninit, ops::Deref};
use std::{sync::Arc, vec, vec::Vec};

#[derive(Debug, Clone)]
pub struct SharedBuffer(Arc<[f32]>);

impl AsRef<[f32]> for SharedBuffer {
    #[inline(always)]
    fn as_ref(&self) -> &[f32] {
        self.0.as_ref()
    }
}

impl Deref for SharedBuffer {
    type Target = [f32];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl From<&[f32]> for SharedBuffer {
    #[inline(always)]
    fn from(values: &[f32]) -> Self {
        Self(Arc::from(values))
    }
}

impl<const N: usize> From<[f32; N]> for SharedBuffer {
    #[inline(always)]
    fn from(values: [f32; N]) -> Self {
        Self(Arc::from(values))
    }
}

impl From<Vec<f32>> for SharedBuffer {
    #[inline(always)]
    fn from(data: Vec<f32>) -> Self {
        Self(data.into())
    }
}

impl SharedBuffer {
    /// Construct the inner buffer with a single allocation straight into the Arc
    #[inline]
    pub fn from_iter(samples: impl Iterator<Item = f32>, num_samples: usize) -> Self {
        let mut container = Arc::<[f32]>::new_zeroed_slice(num_samples);
        let data = unsafe { Arc::get_mut_unchecked(&mut container) };

        for (sample, value) in samples.zip(data.iter_mut()) {
            *value = MaybeUninit::new(sample);
        }

        Self(unsafe { container.assume_init() })
    }

    /// Is this buffer the only reference to its data?
    #[inline]
    pub fn is_unique(&self) -> bool {
        Arc::strong_count(&self.0) == 1
    }
}

#[derive(Clone)]
pub struct SharedStereoBuffer {
    pub l: SharedBuffer,
    pub r: SharedBuffer,
}

#[derive(Clone)]
pub enum SharedAudioBuffer {
    Mono(SharedBuffer),
    Stereo(SharedStereoBuffer),
}

impl SharedAudioBuffer {
    pub fn from_mono(data: SharedBuffer) -> Self {
        SharedAudioBuffer::Mono(data)
    }

    pub fn from_stereo_deinterleaved(l: SharedBuffer, r: SharedBuffer) -> Self {
        SharedAudioBuffer::Stereo(SharedStereoBuffer { l, r })
    }

    pub fn from_stereo_interleaved(data: SharedBuffer) -> Self {
        let len = data.len() / 2;
        let (mut l, mut r) = (vec![0.; len], vec![0.; len]);
        deinterleave_stereo(data, (&mut l, &mut r));
        SharedAudioBuffer::Stereo(SharedStereoBuffer {
            l: l.into(),
            r: r.into(),
        })
    }

    pub fn into_stereo(self) -> SharedStereoBuffer {
        match self {
            SharedAudioBuffer::Mono(b) => SharedStereoBuffer { l: b.clone(), r: b },
            SharedAudioBuffer::Stereo(b) => b,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Mono(b) => b.len(),
            Self::Stereo(b) => b.l.len().min(b.r.len()),
        }
    }

    pub fn size(&self) -> usize {
        match self {
            Self::Mono(b) => b.len(),
            Self::Stereo(b) => b.l.len() + b.r.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Self::Mono(b) => b.is_empty(),
            Self::Stereo(b) => b.l.is_empty() && b.r.is_empty(),
        }
    }

    pub fn is_stereo(&self) -> bool {
        !matches!(self, Self::Mono(_))
    }

    pub fn is_unique(&self) -> bool {
        match self {
            Self::Mono(b) => b.is_unique(),
            Self::Stereo(b) => b.l.is_unique() && b.r.is_unique(),
        }
    }

    pub fn left(&self) -> &[f32] {
        match self {
            Self::Mono(b) => b.as_ref(),
            Self::Stereo(b) => b.l.as_ref(),
        }
    }

    pub fn right(&self) -> &[f32] {
        match self {
            Self::Mono(b) => b.as_ref(),
            Self::Stereo(b) => b.r.as_ref(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_deinterleave_stereo() {
        let input = vec![1., 2., 3., 4., 5., 6.];
        let output = SharedAudioBuffer::from_stereo_interleaved(input.into());
        assert_eq!(output.left().as_ref(), &[1., 3., 5.]);
        assert_eq!(output.right().as_ref(), &[2., 4., 6.]);
    }

    #[test]
    fn can_interleave_stereo() {
        let l: SharedBuffer = [1., 3., 5.].into();
        let r: SharedBuffer = [2., 4., 6.].into();
        let input = SharedAudioBuffer::from_stereo_deinterleaved(l, r);
        let mut output = vec![0.; 6];
        interleave_stereo((input.left(), input.right()), &mut output);
        assert_eq!(output, vec![1., 2., 3., 4., 5., 6.]);
    }

    #[test]
    fn mono_view_has_correct_dimensions() {
        const LEN: usize = 3;
        let buf = SharedAudioBuffer::from_mono(vec![3.; LEN].into());
        assert_eq!(buf.len(), LEN);
        assert_eq!(buf.size(), LEN);
        assert!(!buf.is_empty());
        assert!(!buf.is_stereo());
    }

    #[test]
    fn stereo_view_has_correct_dimensions() {
        const LEN: usize = 3;
        let buf = SharedAudioBuffer::from_stereo_deinterleaved(
            vec![3.; LEN].into(),
            vec![3.; LEN].into(),
        );
        assert_eq!(buf.len(), LEN);
        assert_eq!(buf.size(), LEN * 2);
        assert!(!buf.is_empty());
        assert!(buf.is_stereo());
    }
}
