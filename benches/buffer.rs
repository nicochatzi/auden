use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sample_pool::buffer::*;

pub fn from_iter_l1(c: &mut Criterion) {
    let data = [1., 2., 3., 4., 5., 6., 7., 8.];

    c.bench_function("Buffer::from_iter | L1", |b| {
        b.iter(|| black_box(SharedBuffer::from_iter(data.iter().copied(), data.len())))
    });
}

pub fn from_iter_l2(c: &mut Criterion) {
    const N: usize = 10_000;

    c.bench_function("Buffer::from_iter | L2", |b| {
        b.iter(|| {
            let data = vec![1.; N];
            black_box(SharedBuffer::from_iter(data.iter().copied(), data.len()))
        })
    });
}

pub fn interleaving_l1(c: &mut Criterion) {
    const N: usize = 3;

    c.bench_function("AudioBuffer::from_stereo_deinterleaved | L1", |b| {
        b.iter(|| {
            let (l, r) = (vec![1.0; N], vec![1.0; N]);
            black_box(SharedAudioBuffer::from_stereo_deinterleaved(l.into(), r.into()))
        })
    });
}

pub fn deinterleaving_l1(c: &mut Criterion) {
    const N: usize = 6;

    c.bench_function("AudioBuffer::from_stereo_interleaved | L1", |b| {
        b.iter(|| {
            let data = vec![1.0; N];
            black_box(SharedAudioBuffer::from_stereo_interleaved(data.into()))
        })
    });
}

fn build_vec(num_samples: usize) -> SharedBuffer {
    (0..num_samples)
        .map(|_| rand::random::<f32>())
        .collect::<Vec<f32>>()
        .into()
}

pub fn interleaving_l2(c: &mut Criterion) {
    const N: usize = 10_000;

    c.bench_function("AudioBuffer::from_stereo_deinterleaved | L2", |b| {
        b.iter(|| {
            let (l, r) = (build_vec(N), build_vec(N));
            black_box(SharedAudioBuffer::from_stereo_deinterleaved(l, r))
        })
    });
}

pub fn deinterleaving_l2(c: &mut Criterion) {
    const N: usize = 10_000;

    c.bench_function("AudioBuffer::from_stereo_interleaved | L2", |b| {
        b.iter(|| {
            let data = build_vec(N);
            black_box(SharedAudioBuffer::from_stereo_interleaved(data))
        })
    });
}

criterion_group!(
    buffer,
    from_iter_l1,
    from_iter_l2,
    interleaving_l1,
    deinterleaving_l1,
    interleaving_l2,
    deinterleaving_l2,
);
criterion_main!(buffer);
