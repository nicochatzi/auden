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
pub fn interleave(input: &[impl AsRef<[f32]>], output: &mut [f32]) {
    let num_channels = input.len();
    let num_samples = input
        .iter()
        .map(|channel| channel.as_ref().len())
        .min()
        .unwrap_or(0)
        .min(output.len() / num_channels);

    for sample in 0..num_samples {
        for channel in 0..num_channels {
            if let Some(channel_data) = input.get(channel) {
                output[sample * num_channels + channel] = channel_data.as_ref()[sample];
            }
        }
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

#[inline]
pub fn deinterleave(inputs: impl AsRef<[f32]>, outputs: &mut [impl AsMut<[f32]>]) {
    let buffer = inputs.as_ref();
    let num_channels = outputs.len();
    let num_samples = buffer.len() / num_channels;

    for sample in 0..num_samples {
        for channel in 0..num_channels {
            if let Some(channel_data) = outputs.get_mut(channel) {
                channel_data.as_mut()[sample] = buffer[sample * num_channels + channel];
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_deinterleave_stereo() {
        let mut output = [0.; 6];
        interleave_stereo((&[1., 3., 5.], &[2., 4., 6.]), &mut output);
        assert_eq!(output, [1., 2., 3., 4., 5., 6.]);
    }

    #[test]
    fn can_interleave_stereo() {
        let mut left = [0.; 3];
        let mut right = [0.; 3];
        deinterleave_stereo(&[1., 2., 3., 4., 5., 6.], (&mut left, &mut right));
        assert_eq!(left, [1., 3., 5.]);
        assert_eq!(right, [2., 4., 6.]);
    }

    #[test]
    fn can_deinterleave() {
        let mut output = [0.; 9];
        interleave(&[&[1., 1., 1.], &[2., 2., 2.], &[3., 3., 3.]], &mut output);
        assert_eq!(output, [1., 2., 3., 1., 2., 3., 1., 2., 3.]);
    }

    #[test]
    fn can_interleave() {
        let out = vec![0.; 3];
        let mut output = [out.clone(), out.clone(), out];
        deinterleave(&[1., 2., 3., 1., 2., 3., 1., 2., 3.], &mut output);
        assert_eq!(output[0], [1., 1., 1.]);
        assert_eq!(output[1], [2., 2., 2.]);
        assert_eq!(output[2], [3., 3., 3.]);
    }
}
