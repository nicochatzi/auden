use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sample_pool::pool::*;
use std::path::PathBuf;

fn build_sample_dir() -> PathBuf {
    const NUM_FILES: usize = 10;
    const NUM_SAMPLES: usize = 44100;

    let dir = tempfile::tempdir().unwrap();

    for i in 0..NUM_FILES {
        let path = dir.path().join(format!("{}.wav", i));
        let spec = hound::WavSpec {
            channels: 2,
            sample_rate: 44100,
            bits_per_sample: 32,
            sample_format: hound::SampleFormat::Float,
        };
        let mut writer = hound::WavWriter::create(&path, spec).unwrap();
        for _ in 0..NUM_SAMPLES {
            writer.write_sample(0.0).unwrap();
            writer.write_sample(0.0).unwrap();
        }
    }

    dir.path().to_owned()
}

pub fn build_manifest(c: &mut Criterion) {
    c.bench_function("SamplePool::build_manifest", |b| {
        b.iter(|| {
            let pool = SamplePool::from_dir(build_sample_dir()).unwrap();
            black_box(pool.build_manifest().unwrap())
        })
    });
}

pub fn from_manifest(c: &mut Criterion) {
    c.bench_function("SamplePool::from_manifest", |b| {
        b.iter(|| {
            let pool = SamplePool::from_dir(build_sample_dir()).unwrap();
            let manifest = pool.build_manifest().unwrap();
            black_box(SamplePool::from_manifest(manifest).unwrap())
        })
    });
}

pub fn from_dir(c: &mut Criterion) {
    c.bench_function("SamplePool::from_dir", |b| {
        b.iter(|| {
            let dir = build_sample_dir();
            black_box(SamplePool::from_dir(dir).unwrap())
        })
    });
}

criterion_group!(pool, build_manifest, from_manifest, from_dir);
criterion_main!(pool);
