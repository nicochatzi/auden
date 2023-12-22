use auden::buffer::shared::SharedAudioBuffer;
use auden::sample_pool::manifest::PoolManifest;
use auden::sample_pool::pool::{PoolHolding, SamplePool};
use auden::stream::{play, plot};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    sample_dir: Option<PathBuf>,

    #[arg(long)]
    manifest: Option<PathBuf>,

    #[arg(long)]
    save: Option<PathBuf>,

    #[arg(long)]
    print: bool,

    #[arg(long)]
    plot: Option<PathBuf>,

    #[arg(long)]
    play: Option<PathBuf>,
}

pub fn main() {
    env_logger::init();

    let args = Args::parse();
    let mut pool = SamplePool::default();

    if let Some(manifest) = args.manifest {
        let manifest = PoolManifest::from_file(manifest).unwrap();
        pool = SamplePool::from_manifest(manifest).unwrap();
    }

    if let Some(dir) = args.sample_dir {
        pool.add_samples(&dir).unwrap();
    }

    if let Some(ref save) = args.save {
        pool.build_manifest().unwrap().save(save).unwrap();
    }

    if args.print {
        log::info!("{:#?}", pool.build_manifest().unwrap());
    }

    if let Some(file) = args.plot {
        let mut pool = SamplePool::default();
        pool.add_sample(file).unwrap();
        for (id, buf) in pool.samples() {
            let id = format!("{id:?}");
            match buf {
                SharedAudioBuffer::Mono(b) => plot::mono_waveform(&id, b),
                SharedAudioBuffer::Stereo(b) => plot::stereo_waveform(&id, b.l, b.r),
            };
        }
    }

    if let Some(file) = args.play {
        let id = pool.add_sample(file).unwrap();
        let config = cpal::StreamConfig {
            channels: 2,
            sample_rate: cpal::SampleRate(44100),
            buffer_size: cpal::BufferSize::Fixed(1024),
        };
        play::Stream::launch_with_timeout(&config, std::time::Duration::from_secs(3), {
            let mut i = 0;
            move |output, channels| {
                let sample = pool.sample(id).unwrap();
                for frame in output.chunks_mut(channels) {
                    frame[0] = sample.left()[i];
                    frame[1] = sample.right()[i];
                    i += 1;
                }
            }
        })
        .unwrap();
    }
}
