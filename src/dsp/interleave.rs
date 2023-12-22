#[inline]
pub fn interleave_stereo(input: (impl AsRef<[f32]>, impl AsRef<[f32]>), output: &mut [f32]) {
    let (l, r) = (input.0.as_ref(), input.1.as_ref());
    let num_samples = l.len().min(r.len().min(output.len()));

    for sample in 0..num_samples {
        output[sample * 2] = l[sample];
    }

    for sample in 0..num_samples {
        output[sample * 2 + 1] = r[sample];
    }
}

#[inline]
pub fn deinterleave_stereo(
    input: impl AsRef<[f32]>,
    mut output: (impl AsMut<[f32]>, impl AsMut<[f32]>),
) {
    const NUM_CHANNELS: usize = 2;

    let buffer = input.as_ref();
    let num_samples = buffer.len() / NUM_CHANNELS;

    for sample in 0..num_samples {
        output.0.as_mut()[sample] = buffer[sample * NUM_CHANNELS];
    }

    for sample in 0..num_samples {
        output.1.as_mut()[sample] = buffer[sample * NUM_CHANNELS + 1];
    }
}
